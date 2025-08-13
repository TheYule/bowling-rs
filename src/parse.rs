use crate::{Frame, Game, Score};

/// Parse a `Vec<Vec<usize>>` of frames into a [Game] struct.
pub fn parse_score(frames: Vec<Vec<usize>>) -> Game {
    let mut parsed_frames = vec![];

    for frame in &frames {
        let first = frame.first();
        let second = frame.get(1);
        let bonus = frame.get(2);

        // No values should count as `Score::EMPTY`
        parsed_frames.push(Frame {
            first: first.and_then(|f| Some(Score::PINS(*f))).unwrap_or(Score::EMPTY),
            second: second.and_then(|s| Some(Score::PINS(*s))).unwrap_or(Score::EMPTY),
            bonus: bonus.and_then(|b| Some(Score::PINS(*b)))
        });
    }

	Game::with_frame_count(frames.len(), parsed_frames)
}

/// Parse a `Vec<usize>` into a [Game] struct.
pub fn parse_raw_score(shots: Vec<usize>, bonus: bool) -> Game {
    let mut parsed_frames = vec![];
    let mut skip = false;

    for i in 0..shots.len() {
        let final_frame = i >= shots.len() - 3;

        if skip {
            skip = false;
            continue;
        }

        let shot = *shots.get(i).unwrap_or(&0);

        if shot >= 10 {
            if final_frame && bonus {
                parsed_frames.push(Frame {
                    first: Score::STRIKE,
                    second: Score::PINS(*shots.get(i + 1).unwrap_or(&0)),
                    bonus: shots.get(i + 2).and_then(|b| Some(Score::PINS(*b)))
                });
                break;
            } else {
                parsed_frames.push(Frame::strike());
            }

            skip = false;
        } else {
            if final_frame && bonus {
                parsed_frames.push(Frame {
                    first: Score::PINS(shot),
                    second: Score::PINS(*shots.get(i + 1).unwrap_or(&0)),
                    bonus: shots.get(i + 2).and_then(|b| Some(Score::PINS(*b)))
                });
                break;
            } else {
                parsed_frames.push(Frame::new(Score::PINS(shot), Score::PINS(*shots.get(i + 1).unwrap_or(&0))));
            }

            skip = true;
        }
    }

    Game::with_frame_count(parsed_frames.len(), parsed_frames)
}

#[cfg(test)]
mod tests {
    use crate::{parse_score, parse_raw_score};

    #[test]
    fn parse_game_300() {
        let mut frames = vec![vec![10]; 9];
        frames.push(vec![10, 10, 10]);
        let game = parse_score(frames);

        assert_eq!(game.score(), 300);
    }

    #[test]
    fn parse_game_245() {
        let game = parse_score(vec![vec![7, 1], vec![9, 1], vec![10], vec![8, 2], vec![10], vec![10], vec![10], vec![10], vec![10], vec![10, 10, 7]]);
        assert_eq!(game.score(), 245);
    }

    #[test]
    fn parse_game_223() {
        let game = parse_score(vec![vec![9, 1], vec![8, 2], vec![10], vec![10], vec![10], vec![10], vec![8, 2], vec![10], vec![10], vec![9]]);
        assert_eq!(game.score(), 223);
    }

    #[test]
    fn parse_raw_game_300() {
        let game = parse_raw_score(vec![10; 12], true);
        assert_eq!(game.score(), 300);
    }

    #[test]
    fn parse_raw_game_245() {
        let game = parse_raw_score(vec![7, 1, 9, 1, 10, 8, 2, 10, 10, 10, 10, 10, 10, 10, 7], true);
        assert_eq!(game.score(), 245);
    }

    #[test]
    fn parse_raw_game_223() {
        let game = parse_raw_score(vec![9, 1, 8, 2, 10, 10, 10, 10, 8, 2, 10, 10, 9], false);
        assert_eq!(game.score(), 223);
    }
}