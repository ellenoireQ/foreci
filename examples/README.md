# ForeCi Examples

Example projects demonstrating ForeCi usage with Docker Compose.

## Projects

| Project | Description | Port |
|---------|-------------|------|
| [nodejs-app](./nodejs-app/) | Node.js Express application | 3000 |
| [python-api](./python-api/) | Python FastAPI application | 8000 |
| [go-service](./go-service/) | Go HTTP service | 8080 |

## Structure

```
examples/
├── nodejs-app/
│   ├── docker-compose.yml
│   ├── Dockerfile
│   ├── index.js
│   └── package.json
├── python-api/
│   ├── docker-compose.yml
│   ├── Dockerfile
│   ├── main.py
│   └── requirements.txt
└── go-service/
    ├── docker-compose.yml
    ├── Dockerfile
    ├── main.go
    └── go.mod
```
