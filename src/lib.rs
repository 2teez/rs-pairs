#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Pairs<T, R> {
    pub first: T,
    pub second: R,
}

impl <T: Copy, R: Copy> Pairs<T, R> {
    pub fn new(first: T, second: R) -> Self {
        Pairs {
            first, second
        }
    }

    pub fn from_tuple(val: (T, R)) -> Self {
        Pairs {
            first: val.0,
            second: val.1
        }
    }

    pub fn to_tuple(&self) -> (T, R) {
        (self.first(), self.second())
    }

    pub fn first(&self) -> T {
        self.first
    }

    pub fn second(&self) -> R {
        self.second
    }
}

pub mod pairs {
    
    pub type Pairs<T> = super::Pairs<&'static str, T>;
}

#[cfg(test)]
mod test;
/*mod test {
    use super::*;

    #[test]
    fn generate_pairs() {
        let pairs: Pairs<&str, u32> = Pairs::new("tim", 23);
        assert_eq!(pairs, Pairs {first: "tim", second: 23});
    }

    #[test]
    #[should_panic(expected="tolu")]
    fn get_second() {
        let pairs = Pairs::new(1, "kunle");
        assert_eq!(pairs.second, "tolu");
    }

    #[test]
    fn get_first_value() {
        let pairs = pairs::Pairs::<_>::new("temiloluwa", 11);
        assert_eq!(pairs.first, "temiloluwa");
        assert_eq!(pairs.second, 11);
    }
}*/
