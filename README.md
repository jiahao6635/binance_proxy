# Binance Proxy API - Rust Implementation

A high-performance, lightweight proxy API server built with Rust. This project forwards requests to Binance's API while allowing query parameter manipulation.

## Features

- Supports forwarding requests to both Binance's Spot and Futures APIs.
- Lightweight and high-performance implementation using `actix-web` and `reqwest`.
- Asynchronous and non-blocking architecture for maximum throughput.
- Safe and efficient query parameter management.
- Error handling with detailed logs for debugging.

---

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Configuration](#configuration)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

---

## Installation

### Prerequisites

- **Rust** (Nightly or Stable)
  - Install Rust via [rustup](https://rustup.rs):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
  - Verify installation:
    ```bash
    rustc --version
    cargo --version
    ```

- **Git** (optional)
  - Clone this repository:
    ```bash
    git clone https://github.com/your_username/binance-proxy-api.git
    cd binance-proxy-api
    ```

---

## Usage

### Running the Application

1. Compile the project:
   ```bash
   cargo build --release
