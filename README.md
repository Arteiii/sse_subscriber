# SSE Subscriber CLI Tool

A simple CLI tool to subscribe to Server-Sent Events (SSE) streams using the `reqwest_eventsource` crate for managing
the connection and retry policies.

## Features

- Connects to an SSE stream from a URL.
- Supports configurable retry policies:
    - **Constant**: Always retries after a fixed delay.
    - **ExponentialBackoff**: Retries with an exponentially increasing delay.
    - **Never**: No retries, fails immediately on error.
- Uses logging to track connection events and messages.
- Implements proper error handling to ensure robust behavior.

## Installation

Make sure you have `Rust` and `Cargo` installed.
You can follow the installation instructions on the [Rust website](https://www.rust-lang.org/tools/install).

To clone this repository, run:

```bash
git clone https://github.com/yourusername/sse-subscribe.git
cd sse-subscribe
```

To build the project:

```bash
cargo build --release
```

## Usage

Run the CLI tool with the URL of the SSE stream and the desired retry policy.

```bash
cargo run -- [URL] --retry-policy [policy] --max-retries [max_retries] --initial-delay [initial_delay_ms] --factor [factor] --max-delay [max_delay_ms]
```

### Arguments:

- `--retry-policy [policy]`: Choose one of the following retry policies:
    - `constant`: Always retry with a fixed delay.
    - `exponential`: Retry with exponentially increasing delays.
    - `never`: No retries, fails immediately on error.

- `--max-retries [max_retries]`: Maximum number of retry attempts (applicable for retry policies).

- `--initial-delay [initial_delay_ms]`: The initial delay for retries in milliseconds.

- `--factor [factor]`: The factor by which the delay increases when using exponential backoff.

- `--max-delay [max_delay_ms]`: Maximum delay in milliseconds for exponential backoff.

### Example:

```bash
cargo run -- http://example.com/events --retry-policy exponential --max-retries 5 --initial-delay 500 --factor 2.0 --max-delay 5000
```

This will connect to `http://example.com/events` with an exponential backoff retry policy, 
starting with a 500ms delay and doubling the delay up to a maximum of 5000ms, retrying up to 5 times.

## Logging

The program uses the `log` crate to log events. 
To view the logs, set the `RUST_LOG` environment variable to one of the following levels:

- `info`: Shows connection open and message received logs.
- `error`: Shows errors related to connection issues or failed retries.

For example:

```bash
export RUST_LOG=info
cargo run -- http://example.com/events --retry-policy constant --max-retries 3 --initial-delay 500
```

This will log connection events and messages at the `info` level.

## Contributing

Feel free to open issues or submit pull requests for enhancements and fixes.

### Steps to Contribute:

1. Fork the repository.
2. Make your changes in a feature branch.
3. Open a pull request to merge your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE-MIT) file for more details.
