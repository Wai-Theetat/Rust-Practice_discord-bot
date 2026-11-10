pub fn to_superscript(n: u32) -> String {
	const SUPER: [char; 10] = [
		'⁰', '¹', '²', '³', '⁴',
		'⁵', '⁶', '⁷', '⁸', '⁹',
	];

	n.to_string()
		.chars()
		.map(|c| SUPER[c.to_digit(10).unwrap() as usize])
		.collect()
}

pub fn group_factors(factors: &[u64]) -> Vec<(u64, u32)> {
	let mut groups: Vec<(u64, u32)> = Vec::new();

	for &factor in factors {
		if let Some(last) = groups.last_mut() {
			if last.0 == factor {
				last.1 += 1;
				continue;
			}
		}

		groups.push((factor, 1));
	}

	groups
}


pub fn format_factorization(n: u64, factors: &[u64]) -> String {
	let groups = group_factors(factors);

	if groups.len() == 1 && groups[0].1 == 1 {
		return format!("{n} is prime.");
	}

	let formatted: Vec<String> = groups
		.iter()
		.map(|&(factor, exponent)| {
			if exponent == 1 {
				factor.to_string()
			} else {
				format!("{}{}", factor, to_superscript(exponent))
			}
		})
		.collect();

	format!("Factor of {n} is {}", formatted.join(" × "))
}





#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_superscript() {
		assert_eq!(to_superscript(0), "⁰");
		assert_eq!(to_superscript(123), "¹²³");
		assert_eq!(to_superscript(100), "¹⁰⁰");
	}

	#[test]
	fn test_group_factors() {
		assert_eq!(
			group_factors(&[2, 2, 2, 3, 3, 5]),
			vec![(2, 3), (3, 2), (5, 1)]
		);

		assert_eq!(
			group_factors(&[7]),
			vec![(7, 1)]
		);

		assert_eq!(
			group_factors(&[]),
			Vec::<(u64, u32)>::new()
		);
	}

	#[test]
	fn test_format_prime() {
		assert_eq!(
			format_factorization(13, &[13]),
			"13 is prime."
		);
	}

	#[test]
	fn test_format_factorization() {
		assert_eq!(
			format_factorization(8, &[2, 2, 2]),
			"Factor of 8 is 2³"
		);

		assert_eq!(
			format_factorization(12, &[2, 2, 3]),
			"Factor of 12 is 2² × 3"
		);

		assert_eq!(
			format_factorization(30, &[2, 3, 5]),
			"Factor of 30 is 2 × 3 × 5"
		);
	}
}
