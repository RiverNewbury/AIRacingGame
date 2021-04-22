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

// The entry corresponding to a single run in the leaderboard. This is essentially just what we're
// storing in the leaderboard, minus the source code.
#[derive(serde::Serialize)]
pub struct LeaderboardEntry {
    username: String,
    score: Score,
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

    // Produces an iterator over the top `n` entries in the leaderboard
    pub fn top_n(&self, n: usize) -> impl '_ + Iterator<Item = LeaderboardEntry> {
        self.rankings
            .iter()
            .rev()
            .take(n)
            .map(|e| LeaderboardEntry {
                username: e.username.clone(),
                score: e.score.clone(),
            })
    }
}
