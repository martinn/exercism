#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl HighScores<'_> {
    pub fn new<'a>(scores: &'a [u32]) -> HighScores<'a> {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut ordered_scores = self.scores.to_vec();
        ordered_scores.sort_unstable_by(|a, b| b.cmp(a));
        ordered_scores.iter().take(3).copied().collect()
    }
}
