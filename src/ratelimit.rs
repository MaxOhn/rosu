use std::time::{Duration, Instant};
use tokio::{sync::Mutex, time::sleep};

/// Basic ratelimiter that grants access for a certain amount of times within a time span.
/// Implemented through token bucket algorithm.
pub(crate) struct RateLimiter {
    rate: f32,
    rate_per_ms: f32,
    guarded: Mutex<Inner>,
}

struct Inner {
    allowance: f32,
    last_call: Instant,
}

impl RateLimiter {
    /// Creates a new [`RateLimiter`].
    /// Allows for up to `rate` amount of access calls within `per_seconds` amount of seconds.
    pub(crate) fn new(rate: u32, per_seconds: u32) -> Self {
        Self {
            rate: rate as f32,
            rate_per_ms: rate as f32 / per_seconds as f32 / 1000.0,
            guarded: Mutex::new(Inner {
                allowance: 0.0,
                last_call: Instant::now(),
            }),
        }
    }

    /// Wait until the next access
    pub(crate) async fn await_access(&self) {
        let mut guarded = self.guarded.lock().await;

        let Inner {
            allowance,
            last_call,
        } = &mut *guarded;

        let elapsed = last_call.elapsed().as_millis() as f32; // ms
        *allowance += elapsed * self.rate_per_ms; // msgs

        if *allowance > self.rate {
            *allowance = self.rate - 1.0;
        } else if *allowance < 1.0 {
            let ms_left = (1.0 - *allowance) / self.rate_per_ms; // s
            sleep(Duration::from_micros((1000.0 * ms_left).round() as u64)).await;
            *allowance = 0.0;
        } else {
            *allowance -= 1.0;
        }

        *last_call = Instant::now();
    }
}
