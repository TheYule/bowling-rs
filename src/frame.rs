use std::fmt;

use crate::Score;

/// A bowling [Frame].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Frame {
    /// The first shot of the [Frame].
    pub first: Score,
    /// The second shot of the [Frame].
    pub second: Score,
    /// The optional bonus shot of the [Frame].
    pub bonus: Option<Score>
}

impl Frame {
    /// Create a new [Frame].
    pub fn new(first: Score, second: Score) -> Self {
        Self {
            first,
            second,
            bonus: None
        }
    }

    /// Create a new [Frame] with a bonus shot.
    pub fn with_bonus(first: Score, second: Score, bonus: Score) -> Self {
        Self {
            first,
            second,
            bonus: Some(bonus)
        }
    }

    /// Create an empty [Frame].
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new spare [Frame].
    pub fn spare(value: usize) -> Self {
        Self::new(Score::PINS(value), Score::SPARE)
    }

    /// Create a new strike [Frame].
    pub fn strike() -> Self {
        Self::new(Score::STRIKE, Score::EMPTY)
    }

    /// Create a new [Frame] with three strikes.
    pub fn triple_strike() -> Self {
        Self::with_bonus(Score::STRIKE, Score::STRIKE, Score::STRIKE)
    }

    /// Check if the [Frame] is a spare.
    pub fn is_spare(&self) -> bool {
        !self.is_strike() && (self.second == Score::SPARE || self.first.value() + self.second.value() == 10)
    }

    /// Check if the [Frame] is a strike.
    pub fn is_strike(&self) -> bool {
        self.first.is_strike()
    }

    /// Calculate the value of the [Frame].
    pub fn value(&self) -> usize {
        let mut value = self.first.value();

        if self.second == Score::SPARE {
            value += 10 - value;
        } else if !self.first.is_strike() || self.bonus.is_some() {
            value += self.second.value();
        }

        if let Some(bonus) = self.bonus {
            if bonus == Score::SPARE {
                value += 10 - self.second.value();
            } else {
                value += bonus.value();
            }

			value = value.min(30);
        } else {
			value = value.min(10);
		}

        value
    }
}

impl<V> From<[V; 2]> for Frame where V: Into<usize> + Copy {
    fn from(value: [V; 2]) -> Self {
        Self::new(Score::PINS(value[0].into()), Score::PINS(value[1].into()))
    }
}

impl<V> From<[V; 3]> for Frame where V: Into<usize> + Copy {
    fn from(value: [V; 3]) -> Self {
        Self::with_bonus(Score::PINS(value[0].into()), Score::PINS(value[1].into()), Score::PINS(value[2].into()))
    }
}

impl<V> From<Vec<V>> for Frame where V: Into<usize> + Copy {
    fn from(value: Vec<V>) -> Self {
        let first = value.first();
        let second = value.get(1);
        let bonus = value.get(2);

        Self {
            first: first.and_then(|f| Some(Score::PINS((*f).into()))).unwrap_or(Score::EMPTY),
            second: second.and_then(|s| Some(Score::PINS((*s).into()))).unwrap_or(Score::EMPTY),
            bonus: bonus.and_then(|b| Some(Score::PINS((*b).into()))),
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(bonus) = self.bonus {
            return write!(formatter, "Frame[{}, {}, {}]", self.first, self.second, bonus);
        }
        
        if self.is_strike() {
            return write!(formatter, "Frame[X]");
        }

        write!(formatter, "Frame[{}, {}]", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Frame, Score};

    #[test]
    fn is_spare() {
        let frame = Frame::new(Score::PINS(5), Score::PINS(7));
        assert!(frame.is_spare());
    }

    #[test]
    fn is_strike() {
        let frame = Frame::new(Score::PINS(11), Score::PINS(1));
        assert!(frame.is_strike());
    }

    #[test]
    fn triple_strike() {
        let frame = Frame::triple_strike();
        assert_eq!(frame.value(), 30);
    }

    #[test]
    fn bonus_frame() {
        let frame = Frame::with_bonus(Score::STRIKE, Score::PINS(9), Score::EMPTY);
        assert_eq!(frame.value(), 19);
    }

    #[test]
    fn spare_value() {
        let frame = Frame::spare(9);
        assert_eq!(frame.value(), 10);
    }
}