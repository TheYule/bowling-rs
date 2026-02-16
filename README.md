# Bowling.rs

A basic bowling score calculator written in Rust.

## Installation

```norust
cargo add bowling-rs
```

## Usage

You can create games manually with frames.

```rust
use bowling_rs::{Frame, Game};

// Create a game with 12 strikes
let mut game = Game::new(vec![Frame::strike(); 9]);
// A perfect game has three strikes in the 10th frame, so we add that here
game.add_frame(Frame::triple_strike());

assert_eq!(game.score(), 300);
```

```rust
use bowling_rs::{Frame, Game, Score};

// A more realistic game
let game = Game::new(vec![
    // You can manually specify pins for each shot in a frame
    Frame::new(Score::PINS(7), Score::PINS(1)),
    // Or, you can use helper functions, such as `Frame::spare`
    Frame::spare(9),
    // All ten pins
    Frame::strike(),
    Frame::spare(8), Frame::strike(),
    Frame::strike(), Frame::strike(),
    Frame::strike(), Frame::strike(),
    Frame::with_bonus(Score::STRIKE, Score::STRIKE, Score::PINS(7))
]);

assert_eq!(game.score(), 245);
```

### Parsing

You can also create games with a `Vec<Vec<usize>>` or `Vec<usize>` using `parse_score` or `parse_raw_score`, respectively.

```rust
use bowling_rs::{Game, parse_score};

// Each frame is a `Vec<usize>`
let game = parse_score(vec![
    vec![7, 1], vec![9, 1],
    vec![10],   vec![8, 2],
    vec![10],   vec![10],
    vec![10],   vec![10],
    vec![10],   vec![10, 10, 7] // Bonus shot
]);

assert_eq!(game.score(), 245);
```

`parse_raw_score` takes in shots, rather than frames.

```rust
use bowling_rs::{Game, parse_raw_score};

// Each shot is a `usize`
let game = parse_raw_score(vec![
    7, 1, // These two shots make up one frame
    9, 1,
    10, // This shot is also a frame, since all ten pins were knocked down
    8, 2,
    10, 10, 10, 10, 10,
    10, 10, 7 // Bonus shot
], true); // You must specify whether there was a bonus shot or not

assert_eq!(game.score(), 245);
```

### Custom Bowling Scoring

You can create custom scoring for games using a `ScoreProvider`.

```rust
use bowling_rs::{Frame, Game, ScoreProvider};

#[derive(Clone, Copy)]
pub struct CustomScoreProvider;

// Implement `ScoreProvider`
impl ScoreProvider for CustomScoreProvider {
    // Calculate the score for each frame of a game
    fn computed(&self, game: &Game) -> Vec<usize> {
        let mut frames = vec![];
        let mut score = 0;

        for i in 0..game.frame_count {
            if let Some(frame) = game.frame(i) {
                // Add the frame's value to the total score
                score += frame.value();

                // You can also impliment bonus points for frames here

                frames.push(score);
            }
        }

        frames
    }

    // Calculate the maximum score achievable for a game
    fn max(&self, game: &Game, provider: &impl ScoreProvider) -> usize {
        unimplemented!()
    }
}

// Create a basic game with 10 strikes
let game = Game::new(vec![Frame::strike(); 10]);
// Calculate the game's score with our `CustomScoreProvider`
let score = game.score_with_provider(CustomScoreProvider);

// The score should be 100, since there are 10 pins per strike
assert_eq!(score, 100);
```

## Contributing

Please create an [issue](https://github.com/TheYule/bowling-rs/issues/new) at this time.

## License 

[MIT](LICENSE)