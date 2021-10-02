#![allow(incomplete_features, non_upper_case_globals, mixed_script_confusables)]
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
mod energy;
mod force;
mod length;
mod mass;
mod mass_rate;
mod mole;
mod pixel;
mod population;
mod position;
mod power;
mod pressure;
mod speed;
mod temperature;
mod time;
mod unit;
mod vector;

#[cfg(test)]
mod test;

pub use accel::*;
pub use angle::*;
pub use area::*;
pub use credits::*;
pub use energy::*;
pub use force::*;
pub use length::*;
pub use mass::*;
pub use mass_rate::*;
pub use mole::*;
pub use pixel::*;
pub use population::*;
pub use position::*;
pub use power::*;
pub use pressure::*;
pub use speed::*;
pub use temperature::*;
pub use time::*;
pub use unit::*;
pub use vector::*;

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f64 {
    type Output = f64;
    #[inline]
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

impl Sqrt for f32 {
    type Output = f32;
    #[inline]
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

pub trait Squared {
    type Output;
    fn squared(self) -> Self::Output;
}

impl const Squared for f64 {
    type Output = Self;
    #[inline]
    fn squared(self) -> Self::Output {
        self * self
    }
}

impl const Squared for f32 {
    type Output = Self;
    #[inline]
    fn squared(self) -> Self::Output {
        self * self
    }
}

pub trait Wrapper: Copy {
    type Inner;
    fn value(self) -> Self::Inner;
}

pub trait New {
    type Value;
    fn new(value: Self::Value) -> Self;
    fn value(self) -> Self::Value;
}

impl const New for f64 {
    type Value = f64;

    #[inline]
    fn new(value: f64) -> Self {
        value
    }

    #[inline]
    fn value(self) -> f64 {
        self
    }
}

impl const New for f32 {
    type Value = f32;

    #[inline]
    fn new(value: f32) -> Self {
        value
    }

    #[inline]
    fn value(self) -> f32 {
        self
    }
}

pub trait Scalar: Sized {
    type Vector;
    fn vector(x: Self, y: Self) -> Self::Vector;
}

pub trait Scalar3: Sized {
    type Vector;
    fn vector3(x: Self, y: Self, z: Self) -> Self::Vector;
}
