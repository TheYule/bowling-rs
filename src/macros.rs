/// Shorthand for creating a [Frame].
/// 
/// Both numbers and special symbols can be used. Numbers and groups of symbols must be separated by a comma.
/// - `-` Gutter
/// - `/` Spare
/// - `x` Strike
/// 
/// # Examples
/// 
/// You can pass in up to two `usize`s for the shots of the [Frame].
/// 
/// ```rust
/// # use bowling_rs::frame;
/// // A missed spare
/// let missed = frame!(9);
/// assert_eq!(missed.value(), 9);
/// 
/// // Also a missed spare
/// let also_missed = frame!(8, 1);
/// assert_eq!(also_missed.value(), 9);
/// ```
/// 
/// You can use `/` to represent a spare. Invalid spare patterns will fail to parse.
/// 
/// ```rust
/// # use bowling_rs::frame;
/// // A normal spare frame
/// let spare = frame!(9, /);
/// assert_eq!(spare.value(), 10);
/// ```
/// 
/// You can use `x` to represent a strike. More than one `x` will trigger the [Frame]'s bonus [Score].
/// 
/// ```rust
/// # use bowling_rs::frame;
/// // A normal strike frame
/// let strike = frame!(x);
/// assert_eq!(strike.value(), 10);
/// 
/// // Two strikes in one frame with an empty bonus shot
/// let double = frame!(xx);
/// assert_eq!(double.value(), 20);
/// 
/// // Three strikes in one frame
/// let triple = frame!(xxx);
/// assert_eq!(triple.value(), 30);
/// ```
/// 
/// Bonus [Frame]s are also supported.
/// 
/// ```rust
/// # use bowling_rs::frame;
/// // A spare and some bonus pins
/// let spare_and_bonus = frame!(9, /, 9);
/// assert_eq!(spare_and_bonus.value(), 19);
/// 
/// // A spare and a strike
/// let spare_and_strike = frame!(9, /x);
/// assert_eq!(spare_and_strike.value(), 20);
/// ```
/// 
/// # Note
/// 
/// Any omitted portions of a [Frame] count as [Score]::EMPTY, **not** as [Score]::PINS(0).
#[macro_export]
macro_rules! frame {
    () => ($crate::Frame::empty());

    (-) => (frame!(0));
    (--) => (frame!(0, 0));
    (-/) => (frame!(0, /));
    (-/-) => (frame!(0, /, 0));
    (-/, $c:expr) => (frame!(0, /, $c));

    (x) => ($crate::Frame::strike());
    (xx) => ($crate::Frame::with_bonus($crate::Score::STRIKE, $crate::Score::STRIKE, $crate::Score::EMPTY));
    (xxx) => ($crate::Frame::triple_strike());
    (x-) => (frame!(x, 0));
    (x--) => (frame!(x, 0, 0));
    (x-/) => (frame!(x, 0, /));
    (x-, $c:expr) => (frame!(x, 0, $c));
    (x, $b:expr) => ($crate::Frame::with_bonus($crate::Score::STRIKE, $crate::Score::PINS($b), $crate::Score::EMPTY));
    (x, $b:expr, /) => ($crate::Frame::with_bonus($crate::Score::STRIKE, $crate::Score::PINS($b), $crate::Score::SPARE));
    (xx, $c:expr) => ($crate::Frame::with_bonus($crate::Score::STRIKE, $crate::Score::STRIKE, $crate::Score::PINS($c)));

    ($a:expr) => ($crate::Frame::new($crate::Score::PINS($a), $crate::Score::EMPTY));
    ($a:expr, $b:expr) => ($crate::Frame::new($crate::Score::PINS($a), $crate::Score::PINS($b)));
    ($a:expr, /) => ($crate::Frame::spare($a));
    ($a:expr, /x) => ($crate::Frame::with_bonus($crate::Score::PINS($a), $crate::Score::SPARE, $crate::Score::STRIKE));
    ($a:expr, /, $c:expr) => ($crate::Frame::with_bonus($crate::Score::PINS($a), $crate::Score::SPARE, $crate::Score::PINS($c)));
}

/// Parse a list of frames into a [Game] struct.
#[macro_export]
macro_rules! parse_score {
    ($($a:literal $($b:literal $($c:literal)?)?);*) => ({
        let mut frames = vec![];
        $({
            frames.push(vec![$a, $($b, $($c)?)?]);
        })*
        $crate::parse_score(frames)
    });
    ($($a:expr $(, $b:expr $(, $c:expr)?)?);*) => ({
        let mut frames = vec![];
        $({
            frames.push(vec![$a, $($b, $($c)?)?]);
        })*
        $crate::parse_score(frames)
    });
}

/// Parse a list of shots into a [Game] struct.
/// 
/// You need to specify if there was a bonus shot:
/// 
/// ```rust
/// # use bowling_rs::parse_raw_score;
/// let game = parse_raw_score!(bonus 10 10 10 10 10 10 10 10 10 10 10 10);
/// assert_eq!(game.score(), 300);
/// ```
#[macro_export]
macro_rules! parse_raw_score {
    (bonus $($a:expr)*) => ({
        let mut shots = vec![];
        $({
            shots.push($a)
        })*
        $crate::parse_raw_score(shots, true)
    });
    (bonus $($a:expr),*) => ({
        let mut shots = vec![];
        $({
            shots.push($a)
        })*
        $crate::parse_raw_score(shots, true)
    });
    ($($a:expr)*) => ({
        let mut shots = vec![];
        $({
            shots.push($a)
        })*
        $crate::parse_raw_score(shots, false)
    });
    ($($a:expr),*) => ({
        let mut shots = vec![];
        $({
            shots.push($a)
        })*
        $crate::parse_raw_score(shots, false)
    });
}

#[cfg(test)]
mod tests {
    // TODO: add more macro tests

    #[test]
    fn triple_strike() {
        let game = parse_raw_score!(bonus 10 10 10);
        assert_eq!(game.score(), 30);
    }
}