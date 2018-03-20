use factor::Factor;

pub trait Lerp {
    fn lerp(self, end: Self, factor: Factor) -> Self;
}

impl Lerp for f32 {
	fn lerp(self, end: Self, factor: Factor) -> Self {
		let start = self;
		let factor_clamped: f32 = factor.into().into();
		let diff = end - start;

		(1.0 - factor_clamped) * start + factor_clamped * (start + diff)
	}
}

impl Lerp for f64 {
	fn lerp(self, end: Self, factor: Factor) -> Self {
		let start = self;
		let factor_clamped: f64 = factor.into().into();
		let diff = end - start;

		(1.0 - factor_clamped) * start + factor_clamped * (start + diff)
	}
}
