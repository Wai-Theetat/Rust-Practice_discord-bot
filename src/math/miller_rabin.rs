//https://www.youtube.com/watch?v=zmhUlVck3J0
//https://artofproblemsolving.com/wiki/...
//https://mathworld.wolfram.com/Fermats...
use super::modular_arithmetic::pow_mod;

fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}

	// Handle small primes
	const SMALL_PRIMES: [u64; 12] = 
		[2, 3, 5, 7 ,11, 13, 17, 19, 23 ,29 ,31, 37];

	for p in SMALL_PRIMES {
		if n == p {
			return  true;
		}
		if n%p == 0 {
			return  false;
		}
	}

	//Big one
	//Decompose n-1 = d * 2^s, where d is odd.
	let mut d = n - 1;
	let mut s = 0;

	while d % 2 == 0 {
		d /= 2;
		s += 1;
	}

	//rand num from u64 range
	const WITNESSES: [u64; 7] = [
		2,
		325,
		9375,
		28178,
		450775,
		9780504,
		1795265022,
	];

	for a in WITNESSES {
		let a = a % n;

		if a == 0 {
			continue;
		}

		let mut x = pow_mod(a, d, n);

		if x == 1 || x == n - 1 {
			continue;
		}

		let mut passed = false;

		for _ in 1..s {
			x = pow_mod(x, 2, n);

			if x == n - 1 {
				passed = true;
				break;
			}
		}

		if !passed {
			return false;
		}
	}
	
	true
}




#[cfg(test)]
mod tests {
	use super::*;

	// Simple reference implementation using trial division.
	fn trial_division(n: u64) -> bool {
		if n < 2 {
			return false;
		}

		let mut i = 2;

		while i <= n / i {
			if n % i == 0 {
				return false;
			}

			i += 1;
		}

		true
	}

	#[test]
	fn test_small_numbers() {
		assert!(!is_prime(0));
		assert!(!is_prime(1));

		assert!(is_prime(2));
		assert!(is_prime(3));
		assert!(is_prime(5));
		assert!(is_prime(7));

		assert!(!is_prime(4));
		assert!(!is_prime(9));
		assert!(!is_prime(15));
	}

	#[test]
	fn test_trial_division_comparison() {
		for n in 0..200_000 {
			assert_eq!(
				is_prime(n),
				trial_division(n),
				"Mismatch for n = {n}"
			);
		}
	}

	#[test]
	fn test_carmichael_numbers() {
		let numbers = [561, 1105, 1729];

		for n in numbers {
			assert!(!is_prime(n), "{n} should be composite");
		}
	}

	#[test]
	fn test_strong_pseudoprimes() {
		let numbers = [
			2047,
			3215031751,
			341550071728321,
		];

		for n in numbers {
			assert!(!is_prime(n), "{n} should be composite");
		}
	}

	#[test]
	fn test_large_primes() {
		assert!(is_prime(4294967291));
		assert!(is_prime(18446744073709551557));
	}

	#[test]
	fn test_u64_max() {
		assert!(!is_prime(u64::MAX));
	}
}
