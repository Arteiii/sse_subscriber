use reqwest_eventsource::{EventSource, Event};
use reqwest_eventsource::retry::{Constant, ExponentialBackoff, Never};
use std::time::Duration;
use reqwest::header::USER_AGENT;
use tokio_stream::StreamExt;

pub enum RetryPolicyType {
    Constant(Constant),
    Exponential(ExponentialBackoff),
    Never(Never),
}

impl RetryPolicyType {
    pub fn from_args(
        policy: &str,
        max_retries: usize,
        initial_delay: u64,
        factor: f64,
        max_delay: u64,
    ) -> Self {
        match policy.to_lowercase().as_str() {
            "constant" => RetryPolicyType::Constant(Constant::new(
                Duration::from_millis(initial_delay),
                Some(max_retries),
            )),
            "exponential" => RetryPolicyType::Exponential(ExponentialBackoff::new(
                Duration::from_millis(initial_delay),
                factor,
                Some(Duration::from_millis(max_delay)),
                Some(max_retries),
            )),
            "never" => RetryPolicyType::Never(Never),
            _ => {
                println!("Invalid retry policy. Defaulting to ExponentialBackoff.");
                RetryPolicyType::Exponential(ExponentialBackoff::new(
                    Duration::from_millis(initial_delay),
                    factor,
                    Some(Duration::from_millis(max_delay)),
                    Some(max_retries),
                ))
            }
        }
    }
}

pub async fn subscribe_to_sse(
    url: String,
    retry_policy: RetryPolicyType,
    user_agent: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to SSE stream at: {}", &url);

    let client = reqwest::Client::new();
    let request = client.get(&url).header(USER_AGENT, user_agent);

    let mut es = EventSource::new(request)?;

    match retry_policy {
        RetryPolicyType::Constant(policy) => es.set_retry_policy(Box::new(policy)),
        RetryPolicyType::Exponential(policy) => es.set_retry_policy(Box::new(policy)),
        RetryPolicyType::Never(policy) => es.set_retry_policy(Box::new(policy)),
    }

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => {
                println!("Connection opened.");
            }
            Ok(Event::Message(msg)) => {
                println!("Message received: Event: {}, Data: {}", msg.event, msg.data);
            }
            Err(err) => {
                println!("Error: {}", err);
                es.close();
                break;
            }
        }
    }

    Ok(())
}
