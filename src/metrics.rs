use prometheus::{IntCounter, IntCounterVec, Opts};

pub(crate) struct Metrics {
    pub(crate) counters: IntCounterVec,
    pub(crate) beatmaps: IntCounter,
    pub(crate) matches: IntCounter,
    pub(crate) recent_scores: IntCounter,
    pub(crate) scores: IntCounter,
    pub(crate) top_scores: IntCounter,
    pub(crate) users: IntCounter,
}

impl Metrics {
    #[rustfmt::skip]
    pub(crate) fn new() -> Self {
        let opts = Opts::new("osu_requests", "osu!api request count");
        let counters = IntCounterVec::new(opts, &["type"]).unwrap();

        Self {
            beatmaps: counters.get_metric_with_label_values(&["Beatmaps"]).unwrap(),
            matches: counters.get_metric_with_label_values(&["Matches"]).unwrap(),
            recent_scores: counters.get_metric_with_label_values(&["RecentScores"]).unwrap(),
            scores: counters.get_metric_with_label_values(&["Scores"]).unwrap(),
            top_scores: counters.get_metric_with_label_values(&["TopScores"]).unwrap(),
            users: counters.get_metric_with_label_values(&["Users"]).unwrap(),

            counters,
        }
    }
}
