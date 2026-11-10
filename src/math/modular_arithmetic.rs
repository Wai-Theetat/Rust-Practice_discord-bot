//  (a + b) % m without overflowing u64.
// if m == 0 -> panic
pub fn add_mod(a:u64, b:u64, m:u64) -> u64 {
	assert!(m != 0, "modulus cannot be zero");

	((a as u128 + b as u128) % m as u128) as u64
}

pub fn mul_mod(a:u64, b:u64, m:u64) -> u64 {
	assert!(m != 0, "modulus cannot be zero");

	((a as u128 * b as u128) % m as u128) as u64
}

/// Returns (base ^ exp) % m using square-and-multiply.
pub fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
	assert!(m != 0, "modulus cannot be zero");

	let mut result = 1 % m;
	base %= m;

	while exp > 0 {
		if exp & 1 == 1 {
			result = mul_mod(result, base, m);
		}

		base = mul_mod(base, base, m);
		exp >>= 1;
	}

	result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_mod() {
        assert_eq!(add_mod(10, 20, 7), 2);
        assert_eq!(add_mod(0, 0, 5), 0);
        assert_eq!(add_mod(u64::MAX, u64::MAX, u64::MAX), 0);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(mul_mod(10, 20, 7), 4);
        assert_eq!(mul_mod(0, 100, 7), 0);
        assert_eq!(mul_mod(u64::MAX, u64::MAX, u64::MAX), 0);
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(pow_mod(2, 10, 1000), 24);
        assert_eq!(pow_mod(3, 13, 7), 3);
        assert_eq!(pow_mod(5, 0, 7), 1);
        assert_eq!(pow_mod(5, 10, 1), 0);
        assert_eq!(pow_mod(0, 10, 7), 0);
    }

    #[test]
    fn test_large_values() {
        let max = u64::MAX;

        assert_eq!(
            mul_mod(max - 1, max - 2, max),
            (((max - 1) as u128 * (max - 2) as u128) % max as u128) as u64
        );

        assert_eq!(
            add_mod(max - 1, max - 2, max),
            (((max - 1) as u128 + (max - 2) as u128) % max as u128) as u64
        );
    }

    #[test]
    fn test_fermat_primes() {
        let primes = [3, 5, 7, 11, 97, 101, 65537];

        for p in primes {
            assert_eq!(pow_mod(2, p - 1, p), 1);
        }
    }

    #[test]
    #[should_panic(expected = "modulus cannot be zero")]
    fn test_zero_modulus() {
        mul_mod(10, 20, 0);
    }
}
