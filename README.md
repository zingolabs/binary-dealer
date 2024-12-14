# Binary Dealer

## Overview

**Binary Dealer** is a Rust-based tool designed to streamline the distribution and managemen
t of static binary files across multiple projects. With an emphasis on version control and e
ase of access, Binary Dealer aims to facilitate the seamless retrieval and deployment of bin
aries, while automating the build process as it evolves.

## Planned Features

- **Static Binary Serving**: Initially, the server will serve static binary files, making it
 easy to access precompiled binaries for various projects.
- **Version Management**: Manage multiple versions of binaries efficiently and serve them up
on request.
- **Automated Compilation**: Future iterations will include automated compilation of binarie
s from their source code, ensuring up-to-date versions are readily available.
- **Event-Driven Updates**: Ultimately, the tool will be enhanced to compile binaries automa
tically whenever new tags or releases are available in the source repositories.

## Roadmap

1. **Phase 1**: Serve static binary files
   - [ ] During this phase, the server will be set up to serve binaries that are manually compil
ed and placed in the designated directory.

2. **Phase 2**: Automate Compilation
   - [ ] Implement additional automation features to facilitate the compilation process, reducin
g manual effort.

3. **Phase 3**: Event-Driven Compilation
   - [ ] Integrate event-driven compilation that triggers builds based on updates in source repo
sitories, ensuring all binaries are current and accessible.

## Dependencies

Binary Dealer is built using [Exum](https://github.com/tokio-rs/axum) as its core dependency, which provides the foundationa
l capabilities needed for the server functionality.

## Getting Started

To get started with Binary Dealer, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/zingolabs/binary-dealer.git
   cd binary-dealer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run --release
   ```

4. Place your precompiled binaries in the specified directory for access.
