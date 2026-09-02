//Value of depreciation of TV after 3 years
fn main() {
	let p: f64 = 210_000.00;
	let r: f64 = 5.00;
	let n: f64 = 3.00;

	let amount = p * (1.0 - (r/100.00)).powf(n);
	println!("The amount is N{}", amount);
	
}