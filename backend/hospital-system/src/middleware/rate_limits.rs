use actix_governor::{Governor, GovernorConfigBuilder};
use std::time::Duration;

pub fn configure_rate_limiter(requests_per_minute: u64) -> Governor {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(Duration::from_secs(60) / requests_per_minute as u32)
        .burst_size(requests_per_minute as u32)
        .finish()
        .unwrap();
    
    Governor::new(&governor_conf)
}