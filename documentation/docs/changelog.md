# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-21

### Added

- Initial release of Memista
- High-performance vector search service combining SQLite metadata storage with USearch vector indexing
- RESTful HTTP API with endpoints for inserting chunks, searching, and dropping databases
- Multi-database support with isolated SQLite tables and USearch indexes
- Auto-generated OpenAPI documentation with Swagger, Redoc, and RapiDoc interfaces
- Environment-based configuration
- Comprehensive Rust documentation for library usage
- Examples demonstrating both HTTP server and library usage
- Proper Cargo.toml metadata for publishing to crates.io

### Changed

- Upgraded usearch dependency from version 2.12.0 to 2.19.4
- Made usearch a required dependency instead of optional
- Fixed API compatibility issues with the updated usearch version
- Improved code organization and documentation

### Fixed

- Resolved compilation warnings and errors
- Fixed mutable variable usage in insert_chunk function
- Cleaned up unused imports and variables
