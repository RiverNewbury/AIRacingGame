//! Wrapper module for the [`Leaderboard`] type

use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::sim::Score;

pub struct Leaderboard {
    rankings: BTreeSet<RankedSource>,
}

struct RankedSource {
    username: String,
    score: Score,
    source: String,
}

impl PartialEq for RankedSource {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for RankedSource {}

impl PartialOrd for RankedSource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RankedSource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.source.len().cmp(&other.source.len()))
    }
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            rankings: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, username: String, code: String, score: Score) {
        self.rankings.insert(RankedSource {
            username,
            score,
            source: code,
        });
    }
}
