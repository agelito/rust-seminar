---
title: API Server
---

# API Server

This section is a walkthrough for writing a server exposing a simple API in Rust. The API server
will expose an API for storing and retrieving movie information. We will not connect to any database
and instead store all state in memory.

## Specification

### Endpoints

| Endpoint | Description |
| --- | --- |
| `GET /api/v1/status` | Health check endpoint. |
| `GET /api/v1/movies` | Returns a list of all movies. |
| `GET /api/v1/movies/{:id}` | Returns a single movie by ID. |
| `PUT /api/v1/movies/{:id}` | Updates a single movie by ID. |
| `DELETE /api/v1/movies/{:id}` | Delete a movie by ID. |
| `POST /api/v1/movies` | Create a new movie. |

### Request / Response

#### Movie Response

```json
{
    "id": "string",
    "title": "string",
    "director": "string",
    "release_year": "int",
}
```

#### Create Movie Request

```json
{
    "title": "string",
    "director": "string",
    "release_year": "int"
}
```

#### Update Movie Request

```json
{
    "id": "string",
    "title": "string",
    "director": "string",
    "release_year": "int"
}
```
