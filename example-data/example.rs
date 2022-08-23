// Example sankey generator template
// Put this in data/ to run this file and generate an SVG

use sankey::Sankey;
use super::{helpers::{Band, apply_tax}, style::*};

pub fn generate(sankey: &mut Sankey) {
	let salary = sankey.node(Some(50000.0), Some("Salary".into()), Some(INCOME_NODE_COLOR.into()));
	let bonus = sankey.node(Some(5000.0), Some("Bonus".into()), Some(INCOME_NODE_COLOR.into()));
	let employer = sankey.node(None, Some("Employer".into()), Some(EMPLOYER_NODE_COLOR.into()));
	let government = sankey.node(None, Some("Government".into()), Some(GOVERNMENT_NODE_COLOR.into()));
	
	let income = sankey.node(None, Some("Income".into()), Some(INCOME_NODE_COLOR.into()));
	sankey.edge(salary, income, sankey.remaining_output(salary), None, Some(INCOME_EDGE_COLOR.into()));
	sankey.edge(bonus, income, sankey.remaining_output(bonus), None, Some(INCOME_EDGE_COLOR.into()));
	
	let pension = sankey.node(None, Some("Pension".into()), Some(SAVING_NODE_COLOR.into()));
	sankey.edge(income, pension, sankey.required_output(income) * 0.1, None, Some(SAVING_EDGE_COLOR.into()));
	sankey.edge(employer, pension, sankey.current_input(pension), None, Some(EMPLOYER_EDGE_COLOR.into()));
	
	let taxable = sankey.node(None, Some("Taxable".into()), Some(INCOME_NODE_COLOR.into()));
	sankey.edge(income, taxable, sankey.remaining_output(income), None, Some(INCOME_EDGE_COLOR.into()));
	
	let tax = sankey.node(None, Some("Tax".into()), Some(TAX_NODE_COLOR.into()));
	sankey.edge(taxable, tax, apply_tax(sankey.required_output(taxable), &TAX_BANDS), None, Some(TAX_EDGE_COLOR.into()));
	
	let national_insurance = sankey.node(Some(apply_tax(sankey.required_output(taxable), &NATIONAL_INSURANCE_BANDS)), Some("National Insurance".into()), Some(TAX_NODE_COLOR.into()));
	sankey.edge(taxable, national_insurance, apply_tax(sankey.required_output(taxable), &NATIONAL_INSURANCE_CONTRIBUTION_BANDS), None, Some(TAX_EDGE_COLOR.into()));
	sankey.edge(employer, national_insurance, sankey.remaining_input(national_insurance), None, Some(EMPLOYER_EDGE_COLOR.into()));
	
	let student_loan = sankey.node(None, Some("Student Loan".into()), Some(TAX_NODE_COLOR.into()));
	sankey.edge(taxable, student_loan, apply_tax(sankey.required_output(taxable), &STUDENT_LOAN_BANDS), None, Some(TAX_EDGE_COLOR.into()));
	
	let rent = sankey.node(Some(8400.0), Some("Rent".into()), Some(BILLS_NODE_COLOR.into()));
	sankey.edge(taxable, rent, sankey.required_input(rent), None, Some(BILLS_EDGE_COLOR.into()));
	
	let bills = sankey.node(Some(1000.0), Some("Bills".into()), Some(BILLS_NODE_COLOR.into()));
	sankey.edge(taxable, bills, sankey.required_input(bills), None, Some(BILLS_EDGE_COLOR.into()));
	
	let food = sankey.node(Some(2500.0), Some("Food".into()), Some(SPENDING_NODE_COLOR.into()));
	sankey.edge(taxable, food, sankey.required_input(food), None, Some(SPENDING_EDGE_COLOR.into()));
	
	let travel = sankey.node(Some(5000.0), Some("Travel".into()), Some(SPENDING_NODE_COLOR.into()));
	sankey.edge(taxable, travel, sankey.required_input(travel), None, Some(SPENDING_EDGE_COLOR.into()));
	
	let other = sankey.node(Some(2000.0), Some("Other".into()), Some(SPENDING_NODE_COLOR.into()));
	sankey.edge(taxable, other, sankey.required_input(other), None, Some(SPENDING_EDGE_COLOR.into()));
	
	if sankey.remaining_output(taxable) > 0.0 {
		let lisa = sankey.node(None, Some("LISA".into()), Some(SAVING_NODE_COLOR.into()));
		sankey.edge(taxable, lisa, f64::min(sankey.remaining_output(taxable), 4000.0), None, Some(SAVING_EDGE_COLOR.into()));
		sankey.edge(government, lisa, sankey.current_input(lisa) * 0.25, None, Some(GOVERNMENT_EDGE_COLOR.into()));
	}
	
	if sankey.remaining_output(taxable) > 0.0 {
		let isa = sankey.node(None, Some("ISA".into()), Some(SAVING_NODE_COLOR.into()));
		sankey.edge(taxable, isa, f64::min(sankey.remaining_output(taxable), 16000.0), None, Some(SAVING_EDGE_COLOR.into()));
	}
	
	if sankey.remaining_output(taxable) > 0.0 {
		let savings = sankey.node(None, Some("Savings".into()), Some(SAVING_NODE_COLOR.into()));
		sankey.edge(taxable, savings, sankey.remaining_output(taxable), None, Some(SAVING_EDGE_COLOR.into()));
	}
}

const TAX_BANDS: [Band; 6] = [
	Band { max: 12570.0, rate: 0.0 },
	Band { max: 50270.0, rate: 0.2 },
	Band { max: 100000.0, rate: 0.4 },
	Band { max: 125140.0, rate: 0.6 },
	Band { max: 150000.0, rate: 0.4 },
	Band { max: f64::INFINITY, rate: 0.45 },
];

const NATIONAL_INSURANCE_CONTRIBUTION_BANDS: [Band; 3] = [
	Band { max: 12570.0, rate: 0.0 },
	Band { max: 50270.0, rate: 0.1325 },
	Band { max: f64::INFINITY, rate: 0.0325 },
];

const NATIONAL_INSURANCE_BANDS: [Band; 2] = [
	Band { max: 9100.0, rate: 0.0 },
	Band { max: f64::INFINITY, rate: 0.1505 },
];

const STUDENT_LOAN_BANDS: [Band; 2] = [
	Band { max: 27295.0, rate: 0.0 },
	Band { max: f64::INFINITY, rate: 0.09 },
];
