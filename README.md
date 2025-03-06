# Mostro Desktop Client

The **Mostro Desktop Client** is a graphical user interface (GUI) application for interacting with the Mostro Protocol, a decentralized, peer-to-peer Bitcoin exchange platform built on the Lightning Network and integrated with Nostr. This client aims to provide a user-friendly way to log in, manage seed phrases, and eventually perform Bitcoin trades securely and privately, all while maintaining the censorship-resistant and privacy-focused ethos of the Mostro Protocol.

This project is in early development and currently focuses on a basic login interface. Future updates will expand functionality to include trading and additional features aligned with Mostro’s goals.

## Installation

### Prerequisites
- Rust (latest stable version recommended)
- Git

### Steps
#### Clone the Repository:
```bash
git clone https://github.com/fabohax/mostro-desk.git
cd mostro-desktop
```

#### Install Dependencies:
Ensure you have Rust installed, then run:
```bash
cargo build
```

#### Run the Application:
```bash
cargo run
```

## Dependencies

The project uses `iced`, a cross-platform GUI library for Rust.

Add to your `Cargo.toml`:
```toml
[dependencies]
iced = "0.12"
```

## Roadmap
- Integrate **BIP-39** for proper seed phrase generation and validation.
- Implement **Mostro Protocol authentication** using the seed phrase.
- Add **trading interface** for buying/selling Bitcoin via the Lightning Network.
- Enhance UI with **error messages and better feedback**.
- Support for **multiple languages**.
- Package releases for **Windows, macOS, and Linux**.

## License
This project is licensed under the **MIT License** (`LICENSE`).

## Acknowledgments
- Inspired by the **Mostro Protocol** and its mission for financial sovereignty.
- Built with **Rust** and **iced**.
- Supported by the **open-source community**.
