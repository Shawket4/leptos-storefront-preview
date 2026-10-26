use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Sub};

/// An amount of Egyptian pounds in piastres (1 EGP = 100 piastres).
///
/// Origin: Madar stores every money amount as integer piastres (`price: i32` on menu items,
/// `subtotal`/`total` on delivery orders, see MadarRust `openapi.json`). We widen to `i64` so a
/// cart total can never overflow.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Piastres(pub i64);

impl Piastres {
    pub const ZERO: Piastres = Piastres(0);

    pub const fn from_pounds(pounds: i64) -> Self {
        Piastres(pounds * 100)
    }

    /// Whole pounds (truncated toward zero).
    pub const fn pounds(self) -> i64 {
        self.0 / 100
    }

    /// The piastre remainder, always 0..=99.
    pub const fn cents(self) -> i64 {
        (self.0 % 100).abs()
    }

    /// Percentage saved going from `self` (the original) to `now`, rounded to the nearest int.
    pub fn percent_off(self, now: Piastres) -> u8 {
        if self.0 <= 0 || now >= self {
            return 0;
        }
        (((self.0 - now.0) as f64 / self.0 as f64) * 100.0).round() as u8
    }
}

impl Add for Piastres {
    type Output = Piastres;
    fn add(self, rhs: Piastres) -> Piastres {
        Piastres(self.0 + rhs.0)
    }
}

impl AddAssign for Piastres {
    fn add_assign(&mut self, rhs: Piastres) {
        self.0 += rhs.0;
    }
}

impl Sub for Piastres {
    type Output = Piastres;
    fn sub(self, rhs: Piastres) -> Piastres {
        Piastres(self.0 - rhs.0)
    }
}

impl Mul<u32> for Piastres {
    type Output = Piastres;
    fn mul(self, rhs: u32) -> Piastres {
        Piastres(self.0 * rhs as i64)
    }
}

impl std::iter::Sum for Piastres {
    fn sum<I: Iterator<Item = Piastres>>(iter: I) -> Piastres {
        iter.fold(Piastres::ZERO, |a, b| a + b)
    }
}
