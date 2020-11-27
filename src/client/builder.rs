use super::{Osu, OsuRef};
use crate::{ratelimit::RateLimiter, OsuError, OsuResult};

#[cfg(feature = "cache")]
use super::OsuCached;

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

#[cfg(feature = "cache")]
use darkredis::ConnectionPool;

use reqwest::ClientBuilder as ReqwestClientBuilder;
use std::{sync::Arc, time::Duration};

/// A builder for the main [`Osu`](crate::Osu) client.
#[derive(Debug)]
pub struct OsuBuilder {
    reqwest_client: Option<ReqwestClientBuilder>,
    timeout: Duration,
    api_key: Option<String>,
    #[cfg(feature = "cache")]
    redis: Option<ConnectionPool>,
    #[cfg(feature = "cache")]
    duration: Option<u32>,
    #[cfg(feature = "cache")]
    cached: OsuCached,
}

impl Default for OsuBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            reqwest_client: None,
            api_key: None,
            #[cfg(feature = "cache")]
            redis: None,
            #[cfg(feature = "cache")]
            duration: None,
            #[cfg(feature = "cache")]
            cached: OsuCached::default(),
        }
    }
}

impl OsuBuilder {
    /// Create a new builder to build an [`Osu`](crate::Osu) struct.
    #[cfg(not(feature = "cache"))]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    /// Create a new builder to build an [`Osu`](crate::Osu) struct.
    #[cfg(feature = "cache")]
    pub fn new(api_key: impl Into<String>, pool: ConnectionPool) -> Self {
        Self {
            api_key: Some(api_key.into()),
            redis: Some(pool),
            ..Default::default()
        }
    }

    /// Build the [`Osu`](crate::Osu) struct.
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
            #[cfg(feature = "cache")]
            redis: self.redis.unwrap(),
            #[cfg(feature = "cache")]
            duration: self.duration.unwrap_or(300),
            #[cfg(feature = "cache")]
            cached: self.cached,
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

    /// Set the api key to use for requests.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key.replace(api_key.into());

        self
    }

    #[cfg(feature = "cache")]
    /// Set which requests should be cached
    ///
    /// *See also*:
    /// If visually preferred, you can use [`add_cached`] and chain it
    /// in order to add types separately.
    ///
    /// *Info*:
    /// OsuCached are a bitflag, you can combine them by performing the
    /// `|`-operator.
    ///
    /// [`add_cached`]: #method.add_cached
    pub fn cached(mut self, cached: OsuCached) -> Self {
        self.cached = cached;

        self
    }

    #[cfg(feature = "cache")]
    /// Adds a single [`OsuCached`](crate::OsuCached), this method can be called
    /// repetitively to add multiple values.
    ///
    /// *See also*:
    /// If visually preferred, you can use [`cached`] and specify all
    /// [`OsuCached`] at once. In theory you could also achieve the same result
    /// by passing the combined `OsuCached`-bitflag to this method.
    ///
    /// [`cached`]: #method.cached
    pub fn add_cached(mut self, cached: OsuCached) -> Self {
        self.cached.insert(cached);

        self
    }

    #[cfg(feature = "cache")]
    /// Specify how long values should be cached, defaults to 300 seconds.
    pub fn cache_duration(mut self, seconds: u32) -> Self {
        self.duration.replace(seconds);

        self
    }
}
