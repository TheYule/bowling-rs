use crate::{Frame, Game, Score};

/// Trait for all [ScoreProvider]s.
pub trait ScoreProvider: Clone + Copy {
    /// Calculate the score for each [Frame] of a [Game].
    fn computed(&self, game: &Game) -> Vec<usize>;

    /// Caculate the current score for a [Game].
    fn score(&self, game: &Game) -> usize {
        let computed = self.computed(game);

        if let Some(score) = computed.last() {
            return *score;
        }

        0
    }

    /// Calculate the maximum score achievable for a [Game].
    fn max(&self, game: &Game, provider: &impl ScoreProvider) -> usize;
}

/// [ScoreProvider] for classic 10-pin bowling.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TenPinScoreProvider;

impl ScoreProvider for TenPinScoreProvider {
    fn computed(&self, game: &Game) -> Vec<usize> {
        let mut frames = vec![];
        let mut score = 0;

        for i in 0..game.frame_count {
            if let Some(frame) = game.frame(i) {
                // Add frame's score
                score += frame.value();

                // Add bonus points
				// TODO: fix bonus point system
                if i < game.frame_count - 1 {
                    if let Some(next) = game.frame(i + 1) {
                        if frame.is_spare() {
                            score += next.first.value();
                        } else if frame.is_strike() {
                            score += next.first.value();

							// Broken here?
                            if next.is_strike() {
                                if i == game.frame_count - 2 {
                                    // Fix for spares
                                    score += next.second.value();
                                } else {
                                    if let Some(next2) = game.frame(i + 2) {
                                        // Fix for spares
                                        score += next2.first.value();
                                    }
                                }
                            } else if next.is_spare() {
                                score += 10 - next.first.value();
                            } else {
                                score += next.second.value().min(10 - next.first.value());
                            }
                        }
                    }
                }

                println!("({}): {} -> {}", i, frame, score);

                frames.push(score);
            }
        }

        frames
    }

    fn max(&self, game: &Game, provider: &impl ScoreProvider) -> usize {
        let mut frames = vec![];

        for i in 0..game.frame_count {
            if let Some(frame) = game.frame(i) {
                if i == game.frame_count - 1 {
                    let mut f = frame.clone();

                    if f.first.is_empty() {
                        f.first = Score::STRIKE;
                    }

                    if f.second.is_empty() {
                        if f.first == Score::STRIKE {
                            f.second = Score::STRIKE;
                        } else {
                            f.second = Score::SPARE;
                        }
                    }

                    if let Some(bonus) = frame.bonus {
                        if bonus.is_empty() {
                            if f.second.is_strike() {
                                f.bonus = Some(Score::STRIKE);
                            } else {
                                f.bonus = Some(Score::SPARE);
                            }
                        }
                    } else {
                        if f.second.is_strike() {
                            f.bonus = Some(Score::STRIKE);
                        } else {
                            f.bonus = Some(Score::SPARE);
                        }
                    }

                    frames.push(f);
                } else {
                    if frame.first.is_strike() {
                        frames.push(Frame::strike());
                    } else if frame.second.is_empty() {
                        frames.push(Frame::spare(frame.first.value()))
                    } else {
                        frames.push(frame.clone());
                    }
                }
            } else {
                if i == game.frame_count - 1 {
                    frames.push(Frame::triple_strike());
                } else {
                    frames.push(Frame::strike());
                }
            }
        }

        Game::with_frame_count(game.frame_count, frames).score_with_provider(*provider)
    }
}