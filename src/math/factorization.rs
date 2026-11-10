use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

use super::miller_rabin::is_prime;
use super::pollard_rho::pollard_rho;

#[derive(Debug, PartialEq, Eq)]
pub enum FactorizationError {
	InvalidInput,
	BudgetExhausted,
	WorkerFailed,
}

impl fmt::Display for FactorizationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidInput => 
				write!(f, "Input must be greater than or equal to 2"),
			Self::BudgetExhausted => 
				write!(f, "Factorization iteration budget exhausted"),
			Self::WorkerFailed => 
				write!(f, "A factorization worker panicked"),
		}
	}
}

impl std::error::Error for FactorizationError {}


fn find_factor_parallel(
	n: u64,
	workers: usize,
	iteration_budget: usize,
	max_rounds: usize,
)-> Result<u64, FactorizationError> {

	let workers = workers.clamp(1, 4);

	for round in 0..max_rounds {
		let cancel = AtomicBool::new(false);

		let result = std::thread::scope(|scope| {
			let mut handles = Vec::with_capacity(workers);

			for worker_id in 0..workers {
				// Give each worker different parameters.
				let attempt = round
					.wrapping_mul(workers)
					.wrapping_add(worker_id);

				let seed = 2u64.wrapping_add(attempt as u64);
				let c = 1u64.wrapping_add(attempt as u64);

				let cancel_ref = &cancel;

				let handle = scope.spawn(move || {
					let d = pollard_rho(
						n,
						seed,
						c,
						cancel_ref,
						iteration_budget,
					);

					// Only publish a valid nontrivial factor.
					if let Some(d) = d {
						if d > 1 && d < n && n % d == 0 {
							cancel_ref.store(true, Ordering::Relaxed);
							return Some(d);
						}
					}

					None
				});

				handles.push(handle);
			}

			let mut found_factor = None;
			let mut worker_failed = false;

			// Always join every worker.
			for handle in handles {
				match handle.join() {
					Ok(Some(d)) => {
						if found_factor.is_none() {
							found_factor = Some(d);
						}
					}
					Ok(None) => {}
					Err(_) => {
						worker_failed = true;
					}
				}
			}

			if worker_failed {
				Err(FactorizationError::WorkerFailed)
			} else {
				Ok(found_factor)
			}
		})?;

		if let Some(d) = result {
			return Ok(d);
		}
	}

	Err(FactorizationError::BudgetExhausted)
}


fn strip_small_factors(n: &mut u64, out: &mut Vec<u64>) {
	const SMALL_PRIMES: [u64; 12] = [
		2, 3, 5, 7, 11, 13,
		17, 19, 23, 29, 31, 37,
	];

	for p in SMALL_PRIMES {
		while *n % p == 0 {
			out.push(p);
			*n /= p;
		}
	}
}


fn split(
    n: u64,
    out: &mut Vec<u64>,
    workers: usize,
    iteration_budget: usize,
    max_rounds: usize,
) -> Result<(), FactorizationError> {
    if n == 1 {
        return Ok(());
    }

    // Prime numbers are already factored.
    if is_prime(n) {
        out.push(n);
        return Ok(());
    }

    // Handle perfect squares.
    let root = n.isqrt();

    if root * root == n {
        split(root, out, workers, iteration_budget, max_rounds)?;
        split(root, out, workers, iteration_budget, max_rounds)?;
        return Ok(());
    }

    let d = find_factor_parallel(
        n,
        workers,
        iteration_budget,
        max_rounds,
    )?;

    split(d, out, workers, iteration_budget, max_rounds)?;
    split(n / d, out, workers, iteration_budget, max_rounds)?;

    Ok(())
}


pub fn factorize_with_config(
	n: u64,
	workers: usize,
	iteration_budget: usize,
	max_rounds: usize,
) -> Result<Vec<u64>, FactorizationError> {
	if n < 2 {
		return Err(FactorizationError::InvalidInput);
	}

	let mut remaining = n;
	let mut factors = Vec::new();

	// Remove small prime factors
	strip_small_factors(&mut remaining, &mut factors);

	// Factorize remains.
	split(
		remaining,
		&mut factors,
		workers,
		iteration_budget,
		max_rounds,
	)?;

	// Sort into ascending order.
	factors.sort_unstable();

	Ok(factors)
}

pub fn factorize(n: u64) -> Result<Vec<u64>, FactorizationError> {
	let workers = std::thread::available_parallelism()
		.map(|n| n.get())
		.unwrap_or(1)
		.min(4);

	factorize_with_config(n, workers, 1_000_000, 4)
}



#[test]
fn test_config_zero_workers() {
    let result = factorize_with_config(8051, 0, 100_000, 4);
    assert!(result.is_ok());
}

#[test]
fn test_config_small_budget() {
    assert_eq!(
        factorize_with_config(8051, 4, 1, 1),
        Err(FactorizationError::BudgetExhausted)
    );
}