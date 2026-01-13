//! Streaming data sources for live measurements
//!
//! Supports file watching (CSV), WebSocket streaming, and rolling fits.

use crate::error::{EngineError, Result};
use crate::stats::{MeasurementData, FitResult};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use notify::{Watcher, RecursiveMode, Event};

/// Streaming data source manager
pub struct StreamingManager {
    sources: Arc<Mutex<Vec<Box<dyn DataSource + Send>>>>,
    receiver: mpsc::UnboundedReceiver<StreamEvent>,
    sender: mpsc::UnboundedSender<StreamEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    NewData {
        source_id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        data: DataPoint,
    },
    SourceConnected {
        source_id: String,
    },
    SourceDisconnected {
        source_id: String,
        reason: String,
    },
    FitUpdated {
        params: Vec<f64>,
        uncertainties: Vec<f64>,
        log_likelihood: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub observable: String,
    pub value: f64,
    pub uncertainty: f64,
    pub metadata: serde_json::Value,
}

pub trait DataSource {
    fn id(&self) -> &str;
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn is_active(&self) -> bool;
}

/// CSV file watcher
pub struct CsvFileWatcher {
    id: String,
    path: PathBuf,
    watcher: Option<Box<dyn Watcher + Send>>,
    sender: mpsc::UnboundedSender<StreamEvent>,
    last_position: Arc<Mutex<u64>>,
}

impl CsvFileWatcher {
    pub fn new(
        id: String,
        path: PathBuf,
        sender: mpsc::UnboundedSender<StreamEvent>,
    ) -> Self {
        Self {
            id,
            path,
            watcher: None,
            sender,
            last_position: Arc::new(Mutex::new(0)),
        }
    }

    fn parse_csv_line(&self, line: &str) -> Result<DataPoint> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            return Err(EngineError::Parse(
                "CSV line must have at least 3 columns: observable,value,uncertainty".to_string()
            ));
        }

        Ok(DataPoint {
            observable: parts[0].trim().to_string(),
            value: parts[1].trim().parse()
                .map_err(|e| EngineError::Parse(format!("Invalid value: {}", e)))?,
            uncertainty: parts[2].trim().parse()
                .map_err(|e| EngineError::Parse(format!("Invalid uncertainty: {}", e)))?,
            metadata: serde_json::json!({}),
        })
    }

    fn read_new_lines(&self) -> Result<Vec<DataPoint>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader, Seek, SeekFrom};

        let mut file = File::open(&self.path)?;
        let mut last_pos = self.last_position.lock().unwrap();
        
        file.seek(SeekFrom::Start(*last_pos))?;
        let reader = BufReader::new(file);
        
        let mut data_points = Vec::new();
        let mut current_pos = *last_pos;
        
        for line in reader.lines() {
            let line = line?;
            current_pos += line.len() as u64 + 1; // +1 for newline
            
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            match self.parse_csv_line(&line) {
                Ok(point) => data_points.push(point),
                Err(e) => eprintln!("Error parsing CSV line: {:?}", e),
            }
        }
        
        *last_pos = current_pos;
        Ok(data_points)
    }
}

impl DataSource for CsvFileWatcher {
    fn id(&self) -> &str {
        &self.id
    }

    fn start(&mut self) -> Result<()> {
        let sender = self.sender.clone();
        let path = self.path.clone();
        let id = self.id.clone();
        let reader = Arc::new(Mutex::new(self.clone()));

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    // Read new lines
                    let reader = reader.lock().unwrap();
                    if let Ok(points) = reader.read_new_lines() {
                        for point in points {
                            let _ = sender.send(StreamEvent::NewData {
                                source_id: id.clone(),
                                timestamp: chrono::Utc::now(),
                                data: point,
                            });
                        }
                    }
                }
            }
        }).map_err(|e| EngineError::Io(
            format!("Failed to create watcher: {}", e).into()
        ))?;

        watcher.watch(&path, RecursiveMode::NonRecursive)
            .map_err(|e| EngineError::Io(
                format!("Failed to watch file: {}", e).into()
            ))?;

        self.watcher = Some(Box::new(watcher));

        let _ = self.sender.send(StreamEvent::SourceConnected {
            source_id: self.id.clone(),
        });

        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.watcher = None;
        let _ = self.sender.send(StreamEvent::SourceDisconnected {
            source_id: self.id.clone(),
            reason: "Stopped by user".to_string(),
        });
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.watcher.is_some()
    }
}

impl Clone for CsvFileWatcher {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            path: self.path.clone(),
            watcher: None,
            sender: self.sender.clone(),
            last_position: Arc::clone(&self.last_position),
        }
    }
}

/// WebSocket streaming source
pub struct WebSocketSource {
    id: String,
    url: String,
    sender: mpsc::UnboundedSender<StreamEvent>,
    active: Arc<Mutex<bool>>,
    handle: Option<tokio::task::JoinHandle<()>>,
}

impl WebSocketSource {
    pub fn new(
        id: String,
        url: String,
        sender: mpsc::UnboundedSender<StreamEvent>,
    ) -> Self {
        Self {
            id,
            url,
            sender,
            active: Arc::new(Mutex::new(false)),
            handle: None,
        }
    }
}

impl DataSource for WebSocketSource {
    fn id(&self) -> &str {
        &self.id
    }

    fn start(&mut self) -> Result<()> {
        use tokio_tungstenite::{connect_async, tungstenite::Message};
        use futures_util::StreamExt;
        
        *self.active.lock().unwrap() = true;
        
        let url = self.url.clone();
        let sender = self.sender.clone();
        let id = self.id.clone();
        let active = Arc::clone(&self.active);
        
        let handle = tokio::spawn(async move {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let _ = sender.send(StreamEvent::SourceConnected {
                        source_id: id.clone(),
                    });
                    
                    let (_, mut read) = ws_stream.split();
                    
                    while *active.lock().unwrap() {
                        match read.next().await {
                            Some(Ok(Message::Text(text))) => {
                                // Parse JSON message: {"observable": "sigma_z", "value": 0.5, "uncertainty": 0.01}
                                if let Ok(point) = serde_json::from_str::<DataPoint>(&text) {
                                    let _ = sender.send(StreamEvent::NewData {
                                        source_id: id.clone(),
                                        timestamp: chrono::Utc::now(),
                                        data: point,
                                    });
                                }
                            }
                            Some(Ok(Message::Close(_))) | None => {
                                break;
                            }
                            Some(Err(e)) => {
                                eprintln!("WebSocket error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                    
                    let _ = sender.send(StreamEvent::SourceDisconnected {
                        source_id: id,
                        reason: "Connection closed".to_string(),
                    });
                }
                Err(e) => {
                    let _ = sender.send(StreamEvent::SourceDisconnected {
                        source_id: id,
                        reason: format!("Connection failed: {}", e),
                    });
                }
            }
        });
        
        self.handle = Some(handle);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        *self.active.lock().unwrap() = false;
        
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
        
        let _ = self.sender.send(StreamEvent::SourceDisconnected {
            source_id: self.id.clone(),
            reason: "Stopped by user".to_string(),
        });
        
        Ok(())
    }

    fn is_active(&self) -> bool {
        *self.active.lock().unwrap()
    }
}

/// Rolling fit engine - updates parameter estimates as data streams in
pub struct RollingFitEngine {
    window_size: usize,
    data_buffer: VecDeque<DataPoint>,
    current_params: Vec<f64>,
    last_fit_time: Option<Instant>,
    fit_interval: Duration,
}

impl RollingFitEngine {
    pub fn new(window_size: usize, initial_params: Vec<f64>) -> Self {
        Self {
            window_size,
            data_buffer: VecDeque::new(),
            current_params: initial_params,
            last_fit_time: None,
            fit_interval: Duration::from_secs(1),
        }
    }

    pub fn add_data(&mut self, point: DataPoint) {
        self.data_buffer.push_back(point);
        
        // Maintain window size
        while self.data_buffer.len() > self.window_size {
            self.data_buffer.pop_front();
        }
    }

    pub fn should_refit(&self) -> bool {
        if let Some(last_fit) = self.last_fit_time {
            last_fit.elapsed() >= self.fit_interval
        } else {
            true
        }
    }

    pub fn refit(&mut self) -> Result<FitResult> {
        if self.data_buffer.is_empty() {
            return Err(EngineError::InvalidState(
                "No data available for fitting".to_string()
            ));
        }

        // Convert buffer to MeasurementData
        let data = self.buffer_to_measurement_data();
        
        // TODO: Implement incremental MLE update
        // For now, use gradient descent step
        let step_result = self.gradient_descent_step(&data)?;
        
        self.current_params = step_result.best_params.clone();
        self.last_fit_time = Some(Instant::now());
        
        Ok(step_result)
    }

    fn buffer_to_measurement_data(&self) -> MeasurementData {
        // Group by observable
        let mut observables = std::collections::HashMap::new();
        
        for point in &self.data_buffer {
            let entry = observables.entry(point.observable.clone())
                .or_insert_with(|| (Vec::new(), Vec::new()));
            entry.0.push(point.value);
            entry.1.push(point.uncertainty);
        }

        MeasurementData {
            observables,
            num_shots: self.data_buffer.len(),
            metadata: serde_json::json!({}),
        }
    }

    fn gradient_descent_step(&self, _data: &MeasurementData) -> Result<FitResult> {
        // Stub implementation - would compute gradient and update params
        Ok(FitResult {
            best_params: self.current_params.clone(),
            uncertainties: vec![0.01; self.current_params.len()],
            log_likelihood: -100.0,
            fisher_info: vec![vec![1.0; self.current_params.len()]; self.current_params.len()],
            converged: false,
            iterations: 1,
        })
    }

    pub fn current_estimate(&self) -> (&[f64], Option<Duration>) {
        let age = self.last_fit_time.map(|t| t.elapsed());
        (&self.current_params, age)
    }
}

impl StreamingManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Self {
            sources: Arc::new(Mutex::new(Vec::new())),
            receiver,
            sender,
        }
    }

    pub fn add_csv_source(&self, id: String, path: PathBuf) -> Result<()> {
        let source = CsvFileWatcher::new(id, path, self.sender.clone());
        self.sources.lock().unwrap().push(Box::new(source));
        Ok(())
    }

    pub fn add_websocket_source(&self, id: String, url: String) -> Result<()> {
        let source = WebSocketSource::new(id, url, self.sender.clone());
        self.sources.lock().unwrap().push(Box::new(source));
        Ok(())
    }

    pub fn start_source(&self, id: &str) -> Result<()> {
        let mut sources = self.sources.lock().unwrap();
        
        if let Some(source) = sources.iter_mut().find(|s| s.id() == id) {
            source.start()
        } else {
            Err(EngineError::NotFound(format!("Source not found: {}", id)))
        }
    }

    pub fn stop_source(&self, id: &str) -> Result<()> {
        let mut sources = self.sources.lock().unwrap();
        
        if let Some(source) = sources.iter_mut().find(|s| s.id() == id) {
            source.stop()
        } else {
            Err(EngineError::NotFound(format!("Source not found: {}", id)))
        }
    }

    pub async fn next_event(&mut self) -> Option<StreamEvent> {
        self.receiver.recv().await
    }

    pub fn list_sources(&self) -> Vec<(String, bool)> {
        self.sources.lock().unwrap()
            .iter()
            .map(|s| (s.id().to_string(), s.is_active()))
            .collect()
    }
}

impl Default for StreamingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_fit_buffer() {
        let mut engine = RollingFitEngine::new(10, vec![1.0, 0.5]);
        
        for i in 0..15 {
            engine.add_data(DataPoint {
                observable: "sigma_z".to_string(),
                value: 0.5 + (i as f64) * 0.01,
                uncertainty: 0.01,
                metadata: serde_json::json!({}),
            });
        }

        assert_eq!(engine.data_buffer.len(), 10);
        assert!(engine.should_refit());
    }

    #[test]
    fn test_csv_line_parsing() {
        let (sender, _) = mpsc::unbounded_channel();
        let watcher = CsvFileWatcher::new(
            "test".to_string(),
            PathBuf::from("test.csv"),
            sender,
        );

        let point = watcher.parse_csv_line("sigma_z,0.5,0.01").unwrap();
        assert_eq!(point.observable, "sigma_z");
        assert_eq!(point.value, 0.5);
        assert_eq!(point.uncertainty, 0.01);
    }
}
