macro_rules! poll_req {
    ($ty: ty, $ret: ty) => {
        impl ::std::future::Future for $ty {
            type Output = $crate::error::OsuResult<Option<$ret>>;

            fn poll(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                use ::std::task::Poll;

                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        let bytes = match fut.as_mut().poll(cx) {
                            Poll::Ready(Ok(bytes)) => bytes,
                            Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                            Poll::Pending => return Poll::Pending,
                        };

                        let bytes = bytes.as_ref();

                        let value = serde_json::from_slice::<Vec<$ret>>(bytes)
                            .map(|mut vec| vec.pop())
                            .map_err(|source| crate::OsuError::Parsing {
                                body: String::from_utf8_lossy(bytes).into_owned(),
                                source,
                            });

                        return Poll::Ready(value);
                    } else {
                        self.as_mut().start();
                    }
                }
            }
        }
    };
}

macro_rules! poll_vec_req {
    ($ty: ty, $ret: ty) => {
        impl ::std::future::Future for $ty {
            type Output = $crate::error::OsuResult<Vec<$ret>>;

            fn poll(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                use ::std::task::Poll;
                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        let bytes = match fut.as_mut().poll(cx) {
                            Poll::Ready(Ok(bytes)) => bytes,
                            Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                            Poll::Pending => return Poll::Pending,
                        };

                        let bytes = bytes.as_ref();
                        return Poll::Ready(serde_json::from_slice(bytes).map_err(|source| {
                            crate::OsuError::Parsing {
                                body: String::from_utf8_lossy(bytes).into_owned(),
                                source,
                            }
                        }));
                    } else {
                        self.as_mut().start()
                    }
                }
            }
        }
    };
}

mod beatmap;
mod r#match;
mod score;
mod user;
mod user_score;

pub use beatmap::{GetBeatmap, GetBeatmaps};
pub use r#match::GetMatch;
pub use score::{GetScore, GetScores};
pub use user::GetUser;
pub use user_score::{GetUserBest, GetUserRecent};

use crate::OsuResult;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{fmt, future::Future, pin::Pin};

type Pending<'a> = Pin<Box<dyn Future<Output = OsuResult<Bytes>> + Send + 'a>>;

const TYPE_TAG: &str = "type";
const USER_TAG: &str = "u";

#[derive(Debug)]
pub(crate) struct Request(pub(crate) String);

/// Identifies a user either by id or by name.
///
/// Not needed to use explicitely, only required as `Into<UserIdentification>` i.e. `u32`, `String`, `&str`, or `&String`.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserIdentification {
    Name(String),
    Id(u32),
}

impl fmt::Display for UserIdentification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{}=string&{}={}", TYPE_TAG, USER_TAG, name),
            Self::Id(id) => write!(f, "{}=id&{}={}", TYPE_TAG, USER_TAG, id),
        }
    }
}

impl From<u32> for UserIdentification {
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}

impl From<String> for UserIdentification {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

impl From<&str> for UserIdentification {
    fn from(name: &str) -> Self {
        Self::Name(name.to_owned())
    }
}

impl From<&String> for UserIdentification {
    fn from(name: &String) -> Self {
        Self::Name(name.to_owned())
    }
}
