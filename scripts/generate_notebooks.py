#!/usr/bin/env python3
"""
Jupyter notebook generator for quantum theory engine examples
Creates interactive notebooks for exploring quantum simulations
"""

import json
from pathlib import Path


def create_rabi_notebook():
    """Create notebook for Rabi oscillation example"""
    
    cells = [
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "# Rabi Oscillations in the Quantum Theory Engine\n",
                "\n",
                "This notebook demonstrates how to simulate Rabi oscillations using the quantum theory engine.\n",
                "\n",
                "## Physical System\n",
                "- Two-level quantum system (qubit)\n",
                "- Driven by oscillating field\n",
                "- Hamiltonian: $H = \\frac{\\omega}{2}\\sigma_z + \\Omega \\sigma_x$"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "import numpy as np\n",
                "import matplotlib.pyplot as plt\n",
                "from quantum_theory_engine import load_model, run_simulation\n",
                "\n",
                "# Note: Python bindings not yet implemented\n",
                "# This is a demonstration of the planned API"
            ]
        },
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "## Load the DSL Model\n",
                "\n",
                "The model is defined in `rabi.phys`:"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "# Load model from DSL file\n",
                "model = load_model('../dsl_examples/rabi.phys')\n",
                "print(f\"Model loaded: {model}\")"
            ]
        },
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "## Run Simulation with Different Parameters"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "# Parameter sweep\n",
                "omegas = [0.5, 1.0, 2.0]\n",
                "Omega = 0.2\n",
                "\n",
                "results = []\n",
                "for omega in omegas:\n",
                "    params = {'omega': omega, 'Omega': Omega}\n",
                "    result = run_simulation(model, params)\n",
                "    results.append((omega, result))"
            ]
        },
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "## Visualize Results"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "fig, axes = plt.subplots(1, 3, figsize=(15, 4))\n",
                "\n",
                "for i, (omega, result) in enumerate(results):\n",
                "    times = result.times\n",
                "    # Extract ground state probability\n",
                "    p0 = [m.probabilities[0] for m in result.measurements]\n",
                "    \n",
                "    axes[i].plot(times, p0, 'b-', linewidth=2)\n",
                "    axes[i].set_xlabel('Time')\n",
                "    axes[i].set_ylabel('P(|0⟩)')\n",
                "    axes[i].set_title(f'ω = {omega}, Ω = {Omega}')\n",
                "    axes[i].grid(True, alpha=0.3)\n",
                "    axes[i].set_ylim([-0.1, 1.1])\n",
                "\n",
                "plt.tight_layout()\n",
                "plt.show()"
            ]
        },
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "## Theoretical Prediction\n",
                "\n",
                "The Rabi frequency is given by:\n",
                "$$\\Omega_{\\text{eff}} = \\sqrt{\\omega^2 + \\Omega^2}$$\n",
                "\n",
                "And the oscillation period:\n",
                "$$T = \\frac{2\\pi}{\\Omega_{\\text{eff}}}$$"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "for omega in omegas:\n",
                "    omega_eff = np.sqrt(omega**2 + Omega**2)\n",
                "    period = 2 * np.pi / omega_eff\n",
                "    print(f\"ω={omega}: Ωeff={omega_eff:.3f}, T={period:.3f}\")"
            ]
        },
        {
            "cell_type": "markdown",
            "metadata": {},
            "source": [
                "## Compare with Experimental Data"
            ]
        },
        {
            "cell_type": "code",
            "execution_count": None,
            "metadata": {},
            "source": [
                "import pandas as pd\n",
                "\n",
                "# Load experimental data\n",
                "data = pd.read_csv('../dsl_examples/rabi_measurements.csv')\n",
                "print(data.head())\n",
                "\n",
                "# Plot experimental vs theoretical\n",
                "plt.figure(figsize=(10, 6))\n",
                "\n",
                "# Theoretical\n",
                "plt.plot(results[1][1].times, \n",
                "         [m.probabilities[0] for m in results[1][1].measurements],\n",
                "         'b-', linewidth=2, label='Theory')\n",
                "\n",
                "# Experimental\n",
                "exp_times = data.groupby('time').sum().reset_index()\n",
                "exp_times['p0'] = data[data['outcome']==0].groupby('time')['count'].sum() / \\\n",
                "                  data.groupby('time')['count'].sum()\n",
                "plt.scatter(exp_times['time'], exp_times['p0'], \n",
                "           c='red', s=50, alpha=0.6, label='Experiment')\n",
                "\n",
                "plt.xlabel('Time')\n",
                "plt.ylabel('P(|0⟩)')\n",
                "plt.title('Rabi Oscillations: Theory vs Experiment')\n",
                "plt.legend()\n",
                "plt.grid(True, alpha=0.3)\n",
                "plt.show()"
            ]
        }
    ]
    
    notebook = {
        "cells": cells,
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3"
            },
            "language_info": {
                "name": "python",
                "version": "3.9.0"
            }
        },
        "nbformat": 4,
        "nbformat_minor": 4
    }
    
    return notebook


def main():
    """Generate all example notebooks"""
    
    notebooks_dir = Path("notebooks")
    notebooks_dir.mkdir(exist_ok=True)
    
    # Create Rabi oscillation notebook
    rabi_nb = create_rabi_notebook()
    with open(notebooks_dir / "rabi_oscillations.ipynb", "w") as f:
        json.dump(rabi_nb, f, indent=2)
    
    print("✓ Created rabi_oscillations.ipynb")
    print("\nTo use:")
    print("  1. Build Python bindings: cd python_bindings && maturin develop")
    print("  2. Launch Jupyter: jupyter notebook notebooks/")


if __name__ == "__main__":
    main()
