use std::ops;
use super::polar::Polar;
use super::rectangular::Rectangular;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Bin<F>(pub F);
wrapper!(Bin);
cross_cast!(Bin, Rectangular, Polar);

impl ops::Mul for Bin<Rectangular> {
	type Output = Bin<Rectangular>;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(Rectangular {
			cosine: self.cosine * rhs.cosine - self.sine * rhs.sine,
			sine: self.sine * rhs.cosine + self.cosine * rhs.sine,
		})
	}
}

impl ops::Div for Bin<Rectangular> {
	type Output = Bin<Rectangular>;

	fn div(self, rhs: Self) -> Self::Output {
		let denominator = rhs.cosine * rhs.cosine + rhs.sine * rhs.sine;
		let cosine = (self.cosine * rhs.cosine + self.sine * rhs.sine) / denominator;
		let sine = (self.sine * rhs.cosine - self.cosine * rhs.sine) / denominator;
		Self(Rectangular { cosine, sine })
	}
}

impl ops::Mul for Bin<Polar> {
	type Output = Bin<Polar>;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(Polar {
			magnitude: self.magnitude * rhs.magnitude,
			phase: self.phase + rhs.phase,
		})
	}
}

impl ops::Div for Bin<Polar> {
	type Output = Bin<Polar>;

	fn div(self, rhs: Self) -> Self::Output {
		Self(Polar {
			magnitude: self.magnitude / rhs.magnitude,
			phase: self.phase - rhs.phase,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_multiply_rectangular() {
		let a = Bin(Rectangular { cosine: 1.0, sine: 2.0 });
		let b = Bin(Rectangular { cosine: 3.0, sine: 4.0 });
		assert_eq!(a * b, Bin(Rectangular { cosine: -5.0, sine: 10.0 }));
	}

	#[test]
	fn test_divide_rectangular() {
		let a = Bin(Rectangular { cosine: 1.0, sine: 2.0 });
		let b = Bin(Rectangular { cosine: 3.0, sine: 4.0 });
		let c = a * b;
		assert_eq!(c / a, b);
		assert_eq!(c / b, a);
	}

	#[test]
	fn test_multiply_polar() {
		let a = Bin(Rectangular { cosine: 1.0, sine: 2.0 });
		let b = Bin(Rectangular { cosine: 3.0, sine: 4.0 });
		let polar_a: Bin<Polar> = a.into();
		let polar_b: Bin<Polar> = b.into();
		let polar_c: Bin<Polar> = polar_a * polar_b;
		let polar_rectangular: Bin<Rectangular> = polar_c.into();
		assert_eq!(polar_rectangular, a * b);
	}

	#[test]
	fn test_divide_polar() {
		let a = Bin(Polar { magnitude: 1.0, phase: 2.0 });
		let b = Bin(Polar { magnitude: 3.0, phase: 4.0 });
		let c = a * b;
		assert_eq!(c / a, b);
		assert_eq!(c / b, a);
	}
}
