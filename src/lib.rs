#![allow(incomplete_features)]
#![feature(
    const_panic,
    const_float_classify,
    const_trait_impl,
    const_fn_floating_point_arithmetic
)]

use num_format::{Locale, ToFormattedString};
use std::fmt::{Display, Formatter};

#[macro_use]
mod macros;

mod constants {
    pub const G: f64 = 6.6743015e-11;
}

mod accel;
mod angle;
mod area;
mod credits;
mod length;
mod mass;
mod mass_rate;
mod population;
mod speed;
mod temperature;
mod time;

pub use accel::*;
pub use angle::*;
pub use area::*;
pub use credits::*;
pub use length::*;
pub use mass::*;
pub use mass_rate::*;
pub use population::*;
pub use speed::*;
pub use temperature::*;
pub use time::*;

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f64 {
    type Output = f64;
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

pub trait Squared {
    type Output;
    fn squared(self) -> Self::Output;
}

impl Squared for f64 {
    type Output = Self;
    fn squared(self) -> Self::Output {
        self * self
    }
}

pub trait Wrapper: Copy {
    type Inner;
    fn value(self) -> Self::Inner;
}

trait New {
    fn new(value: f64) -> Self;
    fn value(self) -> f64;
}

impl const New for f64 {
    fn new(value: f64) -> Self {
        value
    }

    fn value(self) -> f64 {
        self
    }
}
