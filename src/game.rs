use std::{fmt, slice::{Iter, IterMut}, vec::IntoIter};

use crate::{Frame, ScoreProvider, TenPinScoreProvider};

/// Contains the data for a bowling game.
#[derive(Clone, Debug, Default, Hash)]
pub struct Game {
    /// The total number of frames allowed for the [Game].
    pub frame_count: usize,
    /// The frames of the [Game].
    frames: Vec<Frame>
}

impl Game {
    /// Create a new [Game].
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            frame_count: 10,
            frames
        }
    }

    /// Create a new [Game] with a custom frame count.
    pub fn with_frame_count(frame_count: usize, frames: Vec<Frame>) -> Self {
        Self {
            frame_count,
            frames
        }
    }

    /// Returns an iterator for the [Frame]s of the [Game].
    pub fn iter(&self) -> Iter<Frame> {
        self.frames.iter()
    }

    /// Returns a mutable iterator for the [Frame]s of the [Game].
    pub fn iter_mut(&mut self) -> IterMut<Frame> {
        self.frames.iter_mut()
    }

    /// Creates a consuming iterator for the [Frame]s of the [Game].
    pub fn into_iter(self) -> IntoIter<Frame> {
        self.frames.into_iter()
    }

    /// Add a [Frame] to the [Game].
    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    /// Remove a [Frame] to the [Game].
    pub fn remove_frame(&mut self, index: usize) -> Frame {
        self.frames.remove(index)
    }

    /// Get a [Frame] from the [Game].
    pub fn frame(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }

    /// Set a [Frame] from the [Game].
    pub fn set_frame(&mut self, index: usize, frame: Frame) {
        self.frames[index] = frame;
    }

    /// Check if the [Game] is perfect (300).
    pub fn is_perfect(&self) -> bool {
        self.score() == 300
    }

    /// Calculate the score for each [Frame] of the [Game] using a [ScoreProvider].
    pub fn computed_with_provider(&self, provider: impl ScoreProvider) -> Vec<usize> {
        provider.computed(&self)
    }

    /// Calculate the score for each [Frame] of the [Game] using the [TenPinScoreProvider].
    pub fn computed(&self) -> Vec<usize> {
        self.computed_with_provider(TenPinScoreProvider)
    }

    /// Caculate the current score for the [Game] using a [ScoreProvider].
    pub fn score_with_provider(&self, provider: impl ScoreProvider) -> usize {
        provider.score(&self)
    }

    /// Caculate the current score for the [Game] using the [TenPinScoreProvider].
    pub fn score(&self) -> usize {
        self.score_with_provider(TenPinScoreProvider)
    }

    /// Calculate the maximum score achievable for the [Game] using a [ScoreProvider].
    pub fn max_with_provider(&self, provider: impl ScoreProvider) -> usize {
        provider.max(&self, &provider)
    }

    /// Calculate the maximum score achievable for the [Game] using the [TenPinScoreProvider].
    pub fn max(&self) -> usize {
        self.max_with_provider(TenPinScoreProvider)
    }
}

// TODO: fix formatting for frames
impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Game ({} frame{}) [\n{:#?}\n]", self.frame_count, if self.frame_count == 1 { "" } else { "s" }, self.frames)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Frame, Game, Score};

    #[test]
    fn game_300() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::triple_strike());
        
        assert_eq!(game.score(), 300);
    }

    #[test]
    fn game_299() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::with_bonus(Score::STRIKE, Score::STRIKE, Score::PINS(9)));
        
        assert_eq!(game.score(), 299);
    }

    #[test]
    fn game_290_0() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::with_bonus(Score::STRIKE, Score::STRIKE, Score::PINS(0)));
        
        assert_eq!(game.score(), 290);
    }

    #[test]
    fn game_290_1() {
        let mut game = Game::new(vec![Frame::spare(9)]);
        game.frames.append(&mut vec![Frame::strike(); 8]);
        game.frames.push(Frame::triple_strike());
        
        assert_eq!(game.score(), 290);
    }

    #[test]
    fn game_289() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::with_bonus(Score::STRIKE, Score::PINS(9), Score::SPARE));
        
        assert_eq!(game.score(), 289);
    }

    #[test]
    fn game_280() {
        let mut game = Game::new(vec![Frame::strike(), Frame::spare(9)]);
        game.frames.append(&mut vec![Frame::strike(); 7]);
        game.frames.push(Frame::triple_strike());
        
        assert_eq!(game.score(), 280);
    }

    #[test]
    fn game_279_0() {
        let mut game = Game::new(vec![Frame::strike(), Frame::strike(), Frame::spare(9)]);
        game.frames.append(&mut vec![Frame::strike(); 6]);
        game.frames.push(Frame::triple_strike());
        
        assert_eq!(game.score(), 279);
    }

    #[test]
    fn game_279_1() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::with_bonus(Score::PINS(9), Score::SPARE, Score::STRIKE));
        
        assert_eq!(game.score(), 279);
    }

    #[test]
    fn game_245() {
        let game = Game::new(vec![Frame::new(Score::PINS(7), Score::PINS(1)), Frame::spare(9), Frame::strike(), Frame::spare(8), Frame::strike(), Frame::strike(), Frame::strike(), Frame::strike(), Frame::strike(), Frame::with_bonus(Score::STRIKE, Score::STRIKE, Score::PINS(7))]);
        assert_eq!(game.score(), 245);
    }

    #[test]
    fn game_223() {
        let game = Game::new(vec![Frame::spare(9), Frame::spare(8), Frame::strike(), Frame::strike(), Frame::strike(), Frame::strike(), Frame::spare(8), Frame::strike(), Frame::strike(), Frame::new(Score::PINS(9), Score::PINS(0))]);
        assert_eq!(game.score(), 223);
    }

    #[test]
    fn game_200() {
        let game = Game::new(vec![Frame::spare(9), Frame::strike(), Frame::spare(9), Frame::strike(), Frame::spare(9), Frame::strike(), Frame::spare(9), Frame::strike(), Frame::spare(9), Frame::with_bonus(Score::STRIKE, Score::PINS(9), Score::SPARE)]);
        assert_eq!(game.score(), 200);
    }

    #[test]
    fn game_0() {
        let game = Game::new(vec![]);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn spare_strike() {
        let game = Game::new(vec![Frame::spare(9), Frame::strike()]);
        assert_eq!(game.score(), 30);
    }

    #[test]
    fn spare_5() {
        let game = Game::new(vec![Frame::spare(9), Frame::new(Score::PINS(5), Score::PINS(0))]);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn strike_5_2_1() {
        let game = Game::new(vec![Frame::strike(), Frame::new(Score::PINS(5), Score::PINS(2)), Frame::new(Score::PINS(1), Score::PINS(0))]);
        assert_eq!(game.score(), 25);
    }

    #[test]
    fn spare_strike_spare() {
        let game = Game::new(vec![Frame::spare(9), Frame::strike(), Frame::spare(9)]);
        assert_eq!(game.score(), 50);
    }

    #[test]
    fn max_300() {
        let game = Game::new(vec![]);
        assert_eq!(game.max(), 300);
    }

    #[test]
    fn max_290() {
        let game = Game::new(vec![Frame::new(Score::PINS(9), Score::EMPTY)]);
        assert_eq!(game.max(), 290);
    }

    #[test]
    fn max_289() {
        let mut game = Game::new(vec![Frame::strike(); 9]);
        game.frames.push(Frame::with_bonus(Score::STRIKE, Score::PINS(9), Score::EMPTY));
        
        assert_eq!(game.max(), 289);
    }

    #[test]
    fn max_279_0() {
        let game = Game::new(vec![Frame::new(Score::PINS(9), Score::PINS(0))]);
        assert_eq!(game.max(), 279);
    }

    #[test]
    fn max_279_1() {
        let game = Game::new(vec![Frame::strike(), Frame::strike(), Frame::new(Score::PINS(9), Score::EMPTY)]);
        assert_eq!(game.max(), 279);
    }
}