use log::{error, info};
use reqwest_eventsource::{EventSource, Event};
use reqwest_eventsource::retry::{Constant, ExponentialBackoff, Never};
use std::time::Duration;
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
                error!("Invalid retry policy. Defaulting to ExponentialBackoff.");
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
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to SSE stream at: {}", &url);

    let client = reqwest::Client::new();
    let request = client.get(&url);

    // Create the EventSource
    let mut es = EventSource::new(request)?;

    // Set the retry policy
    match retry_policy {
        RetryPolicyType::Constant(policy) => es.set_retry_policy(Box::new(policy)),
        RetryPolicyType::Exponential(policy) => es.set_retry_policy(Box::new(policy)),
        RetryPolicyType::Never(policy) => es.set_retry_policy(Box::new(policy)),
    }

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => {
                info!("Connection opened.");
            }
            Ok(Event::Message(msg)) => {
                info!("Message received: Event: {}, Data: {}", msg.event, msg.data);
            }
            Err(err) => {
                error!("Error: {}", err);
                es.close(); // Close the EventSource when an error occurs
                break;
            }
        }
    }

    Ok(())
}
