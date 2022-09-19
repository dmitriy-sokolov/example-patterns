pub struct Adder;

impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

pub struct Kilometers(i32);

pub struct Meters(i32);

pub fn is_marathon_distance(meters: Meters) -> bool {
    meters.0 == 42_195
}

impl From<Kilometers> for Meters {
    fn from(source: Kilometers) -> Self {
        Meters(source.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy() {
        let strategy_1 = |x, y| x + y;
        let strategy_2 = |x, y| 2 * x + y;
        assert_eq!(9, Adder::add(4, 5, strategy_1));
        assert_eq!(5, Adder::add(1, 3, strategy_2));
    }

    #[test]
    fn newtype() {
        let distance_to_the_moon: Kilometers = Kilometers(384_000);
        let result = is_marathon_distance(distance_to_the_moon.into());
        assert_eq!(false, result);
    }
}
