use super::Pending;
use crate::{model::Match, routing::Route, Osu, OsuError, OsuResult};

/// Retrieve a [`Match`].
pub struct GetMatch<'a> {
    fut: Option<Pending<'a>>,
    osu: &'a Osu,

    match_id: u32,
}

impl<'a> GetMatch<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            match_id,
        }
    }

    fn start(&mut self) {
        let route = Route::GetMatch {
            match_id: self.match_id,
        };

        #[cfg(feature = "metrics")]
        self.osu.0.metrics.matches.inc();

        self.fut.replace(Box::pin(self.osu.request_bytes(route)));
    }
}

impl<'a> std::future::Future for GetMatch<'a> {
    type Output = OsuResult<Match>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        use std::task::Poll;

        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                let bytes = match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => bytes,
                    Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                    Poll::Pending => return Poll::Pending,
                };

                let bytes = bytes.as_ref();

                let value =
                    serde_json::from_slice::<Match>(bytes).map_err(|source| OsuError::Parsing {
                        body: String::from_utf8_lossy(bytes).into_owned(),
                        source,
                    });

                return Poll::Ready(value.map_err(|_| OsuError::InvalidMultiplayerMatch));
            } else {
                self.as_mut().start();
            }
        }
    }
}
