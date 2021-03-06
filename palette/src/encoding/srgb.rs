//! The sRGB standard.

use encoding::TransferFn;
use float::Float;
use luma::LumaStandard;
use rgb::{Primaries, RgbSpace, RgbStandard};
use white_point::{WhitePoint, D65};
use {from_f64, FromF64};
use {FloatComponent, Yxy};

///The sRGB color space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Srgb;

impl Primaries for Srgb {
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.6400), from_f64(0.3300), from_f64(0.212656))
    }
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.3000), from_f64(0.6000), from_f64(0.715158))
    }
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T> {
        Yxy::with_wp(from_f64(0.1500), from_f64(0.0600), from_f64(0.072186))
    }
}

impl RgbSpace for Srgb {
    type Primaries = Srgb;
    type WhitePoint = D65;
}

impl RgbStandard for Srgb {
    type Space = Srgb;
    type TransferFn = Srgb;
}

impl LumaStandard for Srgb {
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

impl TransferFn for Srgb {
    fn into_linear<T: Float + FromF64>(x: T) -> T {
        if x <= from_f64(0.04045) {
            x / from_f64(12.92)
        } else {
            ((x + from_f64(0.055)) / from_f64(1.055)).powf(from_f64(2.4))
        }
    }

    fn from_linear<T: Float + FromF64>(x: T) -> T {
        if x <= from_f64(0.0031308) {
            x * from_f64(12.92)
        } else {
            x.powf(T::one() / from_f64(2.4)) * from_f64(1.055) - from_f64(0.055)
        }
    }
}
