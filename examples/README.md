# Foreci Examples

Example projects for testing foreci CI/CD integration.

## Projects

| Project | Port | Description |
|---------|------|-------------|
| nodejs-app | 3000 | Node.js HTTP server |

## Usage

```bash
cd examples
docker compose up --build
```

### Test with foreci runner
```bash
foreci read examples/nodejs-app/Dockerfile
```
