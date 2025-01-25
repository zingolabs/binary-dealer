# Binary Dealer

## Overview

**Binary Dealer** is a Rust-based tool designed to streamline the distribution and management of static binary files across multiple projects. With an emphasis on version control and e
ase of access, Binary Dealer aims to facilitate the seamless retrieval and deployment of binaries, while automating the build process as it evolves.

## Planned Features

- **Static Binary Serving**: Initially, the server will serve static binary files, making it easy to access precompiled binaries for various projects.
- **Version Management**: Manage multiple versions of binaries efficiently and serve them up on request.
- **Automated Compilation**: Future iterations will include automated compilation of binaries from their source code, ensuring up-to-date versions are readily available.
- **Event-Driven Updates**: Ultimately, the tool will be enhanced to compile binaries automatically whenever new tags or releases are available in the source repositories.

## Roadmap

1. **Phase 1**: Serve static binary files
   - [ ] During this phase, the server will be set up to serve binaries that are manually compiled and placed in the designated directory.

2. **Phase 2**: Automate Compilation
   - [ ] Implement additional automation features to facilitate the compilation process, reducing manual effort.

3. **Phase 3**: Event-Driven Compilation
   - [ ] Integrate event-driven compilation that triggers builds based on updates in source repositories, ensuring all binaries are current and accessible.

## Dependencies

Binary Dealer is built using [Axum](https://github.com/tokio-rs/axum) as its core dependency, which provides the foundational capabilities needed for the server functionality.

## Getting Started

To get started with Binary Dealer, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/zingolabs/binary-dealer.git
   cd binary-dealer
   ```
2. Generate or add your certs. We've used Let's Encrypt.
   Customize your domain name with your certs by altering the source code in `src/main.rs` under `config = `.
   Place your precompiled binaries in the specified directory for access.

3. Build the project:
   ```bash
   cargo build --release
 
4. Run the server:
   ```bash
   cargo run --release
   ```

