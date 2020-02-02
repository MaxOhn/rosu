mod ratelimiter;

pub(crate) use ratelimiter::RateLimiter;

/// Provide an iterator over substring of the given length on the given source string
pub(crate) fn cut(mut source: &str, n: usize) -> impl Iterator<Item = &str> {
    std::iter::from_fn(move || {
        if source.is_empty() {
            None
        } else {
            let end_idx = source
                .char_indices()
                .nth(n - 1)
                .map_or_else(|| source.len(), |(idx, ch)| idx + ch.len_utf8());
            let (sub_str, rest) = source.split_at(end_idx);
            source = rest;
            Some(sub_str)
        }
    })
}
