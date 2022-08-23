pub struct Band {
	pub max: f64,
	pub rate: f64,
}

pub fn apply_tax(mut income: f64, bands: &[Band]) -> f64 {
	let mut tax = 0.0;
	for &Band { max, rate } in bands {
		tax += rate * f64::min(income, max);
		income -= max;
		if income <= 0.0 {
			break;
		}
	}
	tax
}
