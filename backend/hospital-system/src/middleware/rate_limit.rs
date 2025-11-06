use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor, governor::middleware::NoOpMiddleware};

pub fn configure_rate_limiter(requests_per_minute: u64) -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second((60 / requests_per_minute).max(1))
        .burst_size(requests_per_minute as u32)
        .finish()
        .unwrap();
    
    Governor::new(&governor_conf)
}