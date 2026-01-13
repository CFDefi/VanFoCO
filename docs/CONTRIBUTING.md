# Contributing to VanFoCO Projects

## Adding a New Project

To add a new project to the VanFossen Corporation portfolio:

### 1. Update projects.json

Add a new entry to the `projects` array in `projects.json` with the following structure:

```json
{
  "id": "unique-project-id",
  "name": "Project Name",
  "description": "Brief description of the project",
  "status": "planned|in-progress|completed|archived",
  "category": "web-development|mobile-development|data-science|infrastructure|tools|general",
  "technologies": ["Technology1", "Technology2"],
  "repository": "https://github.com/organization/repo",
  "website": "https://project-website.com",
  "startDate": "YYYY-MM-DD",
  "visibility": "public|private"
}
```

### 2. Create Project Directory (Optional)

If your project has documentation, assets, or additional resources:

```bash
mkdir projects/your-project-name
```

### 3. Add Project Documentation (Optional)

Create a README.md in your project directory:

```bash
touch projects/your-project-name/README.md
```

## Project Guidelines

- Use clear, descriptive project names
- Keep descriptions concise (1-2 sentences)
- Update project status as work progresses
- Include all relevant technologies
- Ensure all URLs are valid and accessible

## Project Statuses

- **planned**: Project is in planning phase
- **in-progress**: Active development
- **completed**: Project is finished and deployed
- **archived**: Project is no longer active

## Categories

- **web-development**: Web applications and websites
- **mobile-development**: Mobile apps (iOS/Android)
- **data-science**: Data analysis and ML projects
- **infrastructure**: DevOps and infrastructure projects
- **tools**: Development tools and utilities
- **general**: Other projects
