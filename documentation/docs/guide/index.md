# User Guide

This guide covers how to use Memista effectively, whether as a library or HTTP server.

## Architecture Overview

Memista combines two storage systems:

```
┌─────────────────────────────────────────────────────────┐
│                      Memista                            │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────┐         ┌─────────────────┐       │
│  │     SQLite      │         │     USearch     │       │
│  │                 │         │                 │       │
│  │  - Text chunks  │         │  - Embeddings   │       │
│  │  - Metadata     │         │  - Vector index │       │
│  │  - Chunk IDs    │         │  - Fast search  │       │
│  └─────────────────┘         └─────────────────┘       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**SQLite** stores:

- Text content
- JSON metadata
- Auto-generated chunk IDs

**USearch** stores:

- Vector embeddings
- Similarity index for fast search

## Multi-Database Support

Memista supports multiple isolated databases through the `database_id` parameter:

- Each `database_id` creates a separate SQLite table
- Each `database_id` creates a separate USearch index file
- Databases are completely isolated from each other

This enables:

- Multi-tenant applications
- Separate indexes for different document types
- Easy cleanup by dropping specific databases

## Guide Sections

- [Configuration](configuration.md) - Environment variables and settings
- [Library Usage](library-usage.md) - Using Memista as a Rust library
- [HTTP Server](http-server.md) - Running as a standalone server
