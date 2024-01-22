pub trait Pow<T> {
	fn pow(&self, rhs: T) -> T;
}

impl Pow<u32> for u32 {
	fn pow(&self, rhs: u32) -> u32 {
		self.wrapping_pow(rhs)
	}
}
