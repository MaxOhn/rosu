use super::{Osu, OsuRef};
use crate::{ratelimit::RateLimiter, OsuError, OsuResult};

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

use reqwest::ClientBuilder as ReqwestClientBuilder;
use std::{sync::Arc, time::Duration};

/// A builder for the main [`Osu`] client.
#[derive(Debug)]
pub struct OsuBuilder {
    reqwest_client: Option<ReqwestClientBuilder>,
    timeout: Duration,
    api_key: Option<Box<str>>,
}

impl Default for OsuBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            reqwest_client: None,
            api_key: None,
        }
    }
}

impl OsuBuilder {
    /// Create a new builder to build an [`Osu`] struct.
    pub fn new(api_key: impl Into<Box<str>>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    /// Build the [`Osu`] struct.
    ///
    /// # Errors
    ///
    /// Errors if `reqwest` fails to build the client
    pub fn build(self) -> OsuResult<Osu> {
        let http = self
            .reqwest_client
            .unwrap_or_else(ReqwestClientBuilder::new)
            .timeout(self.timeout)
            .build()
            .map_err(OsuError::BuildingClient)?;

        let inner = OsuRef {
            http,
            api_key: self.api_key.unwrap(),
            ratelimiter: RateLimiter::new(15, 1),
            #[cfg(feature = "metrics")]
            metrics: Metrics::new(),
        };

        Ok(Osu(Arc::new(inner)))
    }

    /// Set a pre-configured reqwest client builder to build off of.
    ///
    /// The timeout settings in the reqwest client will be overwritten by
    /// those in this builder.
    ///
    /// The default client uses Rustls as its TLS backend.
    pub fn reqwest_client(mut self, client: ReqwestClientBuilder) -> Self {
        self.reqwest_client.replace(client);

        self
    }

    /// Set the timeout for HTTP requests, defaults to 10 seconds.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }
}
