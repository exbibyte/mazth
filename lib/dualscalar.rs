use std::ops::{Add, Div, Mul, Sub};

///(real, img) pair
#[derive(Debug, Clone)]
pub struct DualScalar(f64, f64);

impl DualScalar {
    pub fn new(a: f64, b: f64) -> Self {
        Self(a, b)
    }
    pub fn real(&self) -> f64 {
        self.0
    }
    pub fn dual(&self) -> f64 {
        self.1
    }
    pub fn real_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
    pub fn dual_mut(&mut self) -> &mut f64 {
        &mut self.1
    }
    pub fn conjugate(&self) -> Self {
        Self::new(self.real(), -self.dual())
    }
    pub fn invert(&self) -> Self {
        let a = 1. / self.real();
        let b = -self.dual() * a * a;
        Self::new(a, b)
    }
    pub fn norm(&self) -> f64 {
        self.real()
    }
    pub fn pow(&self, e: Self) -> Self {
        let a = self.real().powf(e.real());
        let b = self.dual() / self.real() * e.real() * a + e.dual() * a * a.ln();
        Self::new(a, b)
    }
    pub fn sqrt(&self) -> Self {
        let a = self.real().sqrt();
        let b = self.dual() / (2. * a);
        Self::new(a, b)
    }
}

impl Add for DualScalar {
    type Output = DualScalar;
    fn add(self, rhs: DualScalar) -> Self::Output {
        DualScalar::new(self.real() + rhs.real(), self.dual() + rhs.dual())
    }
}
impl Sub for DualScalar {
    type Output = DualScalar;
    fn sub(self, rhs: DualScalar) -> Self::Output {
        DualScalar::new(self.real() - rhs.real(), self.dual() - rhs.dual())
    }
}
impl Mul for DualScalar {
    type Output = DualScalar;
    fn mul(self, rhs: DualScalar) -> Self::Output {
        DualScalar::new(
            self.real() * rhs.real(),
            self.real() * rhs.dual() + self.dual() * rhs.real(),
        )
    }
}
impl Div for DualScalar {
    type Output = DualScalar;
    fn div(self, rhs: DualScalar) -> Self::Output {
        DualScalar::new(
            self.real() / rhs.real(),
            (rhs.real() * self.dual() - self.real() * rhs.dual()) / (self.real() * rhs.real()),
        )
    }
}
