mod utils;

use clap::Parser;
use utils::{subscribe_to_sse, RetryPolicyType};
use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to subscribe to SSE events
    url: String,

    /// Retry policy: "constant", "exponential", or "never"
    #[arg(long, default_value = "exponential")]
    retry_policy: String,

    /// Max retries (applicable for constant and exponential policies)
    #[arg(long, default_value = "5")]
    max_retries: usize,

    /// Delay for constant retries or initial delay for exponential (in ms)
    #[arg(long, default_value = "500")]
    initial_delay: u64,

    /// Exponential backoff factor (ignored for constant policy)
    #[arg(long, default_value = "2.0")]
    factor: f64,

    /// Max delay for exponential backoff (in ms, ignored for constant policy)
    #[arg(long, default_value = "5000")]
    max_delay: u64,

    /// User-Agent header to send with the SSE request
    #[arg(long, default_value = "MySSEClient/1.0")]
    user_agent: String,
}

fn main() {
    let args = Args::parse();

    let retry_policy = RetryPolicyType::from_args(
        &*args.retry_policy,
        args.max_retries,
        args.initial_delay,
        args.factor,
        args.max_delay,
    );

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    if let Err(e) = runtime.block_on(subscribe_to_sse(args.url, retry_policy, &args.user_agent)) {
        println!("Application error: {}", e);
    }
}
