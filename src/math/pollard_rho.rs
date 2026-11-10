// https://youtu.be/7lhlJTtCsiw
// https://youtu.be/PvrxZaH_eZ4
// https://youtu.be/XMPP-qg0m2c
use std::sync::atomic::{AtomicBool, Ordering};

use super::gcd::gcd;
use super::modular_arithmetic::{add_mod, mul_mod};


//Calc f(x) - (x^2 + c)(mod n)
fn rho_function(x:u64, c:u64, n:u64) -> u64 {
	add_mod(mul_mod(x, x, n), c, n)
}

//Floyd's cycle
//Returns Some(d) where 1 < d < n, or None if unsuccessful.
pub fn pollard_rho(
	n: u64, 
	seed: u64, 
	c: u64,
	cancel: &AtomicBool,
    iteration_budget: usize,
	) -> Option<u64> {

	if n < 4 {
		return None;
	}

	// Handle small factors.
	if n % 2 == 0 {
		return Some(2);
	}
	if n % 3 == 0 {
		return Some(3);
	}

	if cancel.load(Ordering::Relaxed) {
        return None;
    }

	let mut tortoise = seed % n;
	let mut hare = seed % n;

	for i in 0..iteration_budget {
		if i % 4096 == 0
            && cancel.load(Ordering::Relaxed)
        {
            return None;
        }

		// Tortoise 1 step
		tortoise = rho_function(tortoise, c, n);

		// Hare 2 steps.
		hare = rho_function(hare, c, n);
		hare = rho_function(hare, c, n);

		// Calculate the distance.
		let diff = tortoise.abs_diff(hare);
		let d = gcd(diff, n);

		if d == 1 {
			continue;
		}
		if d < n {
			return Some(d);
		}

		return None;
	}
	None
}