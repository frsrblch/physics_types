use crate::Length;
use std::ops::Mul;

scalar! {
    struct PixelScale(f32) {
        fn in_px_per_m(pixel_per_meter) -> Self;
    }
}

impl Mul<PixelScale> for Length {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: PixelScale) -> Self::Output {
        self.value as f32 * rhs.value
    }
}

impl Mul<Length> for PixelScale {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: Length) -> Self::Output {
        rhs * self
    }
}
