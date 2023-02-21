#![allow(dead_code)]

#[derive(Debug, Clone)]
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
