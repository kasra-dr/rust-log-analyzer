# Fast Log Analyzer

[![CI/CD Pipeline](https://github.com/kasra-dr/rust-log-analyzer/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/kasra-dr/rust-log-analyzer/actions/workflows/rust-ci.yml)

A high-performance command-line tool written in Rust for parsing and summarizing web server access logs. It provides quick insights into traffic patterns, status codes, and top visitors directly from the terminal.

This project was developed to showcase modern software development and DevOps practices, including high-performance systems programming with Rust, professional containerization with multi-stage Docker builds, and a fully automated CI/CD pipeline using GitHub Actions.

---

## Key Features

* **High Performance:** Built with Rust for blazing fast log processing, making it suitable for analyzing large files efficiently.
* **Structured Parsing:** Uses regular expressions to parse standard Nginx log formats to extract key fields like IP address, timestamp, HTTP method, URL, and status code.
* **Insightful Summaries:** Aggregates data to provide useful summaries, including:
    * Total number of processed log entries.
    * Distribution of HTTP status codes (e.g., 2xx, 4xx, 5xx).
    * A list of the top IP addresses making requests.
* **Flexible Input:** Accepts input from both files and standard input (stdin), allowing it to be easily integrated into shell pipelines (e.g., with `cat`, `grep`, or `zcat`).

---

## Technologies Used

* **Language:** Rust
* **Containerization:** Docker (using a multi-stage build for an optimized, minimal image)
* **CI/CD:** GitHub Actions
* **Core Crates:** `regex` for log parsing.

---

## Installation & Usage

There are multiple ways to use `log-analyzer`.

### 1. From the Docker Image (Recommended)

A pre-built, minimal Docker image is automatically published to the GitHub Container Registry (GHCR) via the CI/CD pipeline.

**A. Pull the image:**
```bash
docker pull ghcr.io/kasra-dr/rust-log-analyzer:latest
```

**B. Run the container on a log file:
```bash
docker run --rm -v /path/to/your/access.log:/data/access.log ghcr.io/kasra-dr/rust-log-analyzer:latest /data/access.log
```

### 2. From GitHub Releases

1.  Navigate to the "Releases" page of this repository.
2.  Download the latest binary for your architecture.
3.  Place it in a directory in your `$PATH` (e.g., `/usr/local/bin`).
4.  Run it directly:
    ```bash
    log-analyzer /path/to/your/access.log
    ```

---

## CI/CD Pipeline

This project is equipped with a full CI/CD pipeline using GitHub Actions (`.github/workflows/rust-ci.yml`). On every push to the `main` branch, the pipeline automatically executes the following stages:

1.  **Lint:** Checks the code for quality and style issues using `cargo clippy`.
2.  **Test:** Runs the automated test suite with `cargo test`.
3.  **Build & Push:** Compiles an optimized release binary within a multi-stage `Dockerfile` and pushes the final, minimal Docker image to the GitHub Container Registry.

This ensures that every change is automatically verified and a deployable artifact is produced, demonstrating a complete and modern software delivery lifecycle.

---

## Building from Source

If you want to build the project yourself, ensure you have the Rust toolchain installed.

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/kasra-dr/rust-log-analyzer.git](https://github.com/kasra-dr/rust-log-analyzer.git)
    cd rust-log-analyzer
    ```

2.  **Build the release binary:**
    ```bash
    cargo build --release
    ```
    The final executable will be located at `target/release/log_analyzer`.
