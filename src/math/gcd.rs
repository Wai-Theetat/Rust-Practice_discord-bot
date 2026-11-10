pub fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		(a, b) = (b, a % b);
	}
	a
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_gcd() {
		assert_eq!(gcd(48, 18), 6);
		assert_eq!(gcd(100, 25), 25);
		assert_eq!(gcd(17, 13), 1);
	}

	#[test]
	fn test_zero() {
		assert_eq!(gcd(0, 5), 5);
		assert_eq!(gcd(5, 0), 5);
		assert_eq!(gcd(0, 0), 0);
	}

	#[test]
	fn test_same_number() {
		assert_eq!(gcd(10, 10), 10);
	}

}
