use std::{cmp::Ordering, fmt};

/// The [Score] of a shot.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq)]
pub enum Score {
    /// Shot hasn't happened yet.
    #[default]
    EMPTY,
    /// Shot knocked down `x` pins.
    PINS(usize),
    /// Shot was a spare.
    SPARE,
    /// Shot was a strike.
    STRIKE
}

impl Score {
    /// Caculate the value of the [Score].
    pub fn value(&self) -> usize {
        match self {
            Score::EMPTY => 0,
            Score::PINS(value) => *value.min(&10),
            Score::SPARE | Score::STRIKE => 10
        }
    }

    /// Check if the [Score] is a strike.
    pub fn is_strike(&self) -> bool {
        if *self == Score::SPARE {
            return false;
        }

        if let Score::PINS(value) = self {
            if value >= &10 {
                return true;
            }
        }

        *self == Score::STRIKE
    }
}

impl fmt::Display for Score {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = match self {
            Score::EMPTY => String::from("<empty>"),
            Score::PINS(v) => v.to_string(),
            Score::SPARE => String::from("/"),
            Score::STRIKE => String::from("X")
        };

        if value == "0" {
            value = String::from("-");
        }
        
        write!(formatter, "{}", value)
    }
}

impl<V> From<V> for Score where V: Into<usize> {
    fn from(value: V) -> Self {
        let v = value.into();

        if v >= 10 {
            return Self::STRIKE;
        }

        Self::PINS(v)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Score::EMPTY => match other {
                Score::EMPTY => Ordering::Equal,
                Score::PINS(_) => Ordering::Less,
                Score::SPARE => Ordering::Less,
                Score::STRIKE => Ordering::Less
            },
            Score::PINS(a) => match other {
                Score::EMPTY => a.cmp(&0),
                Score::PINS(b) => a.cmp(b),
                Score::SPARE => Ordering::Less,
                Score::STRIKE => Ordering::Less
            },
            Score::SPARE => match other {
                Score::EMPTY => Ordering::Greater,
                Score::PINS(b) => self.value().cmp(b),
                Score::SPARE => Ordering::Equal,
                Score::STRIKE => Ordering::Less
            },
            Score::STRIKE => match other {
                Score::EMPTY => Ordering::Greater,
                Score::PINS(_) => Ordering::Greater,
                Score::SPARE => Ordering::Greater,
                Score::STRIKE => Ordering::Equal
            }
        })
    }
}