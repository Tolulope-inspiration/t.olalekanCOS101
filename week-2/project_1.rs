// Compound interest calculator

fn main () {
	let p:f32 = 520_000_000.0;
	let r:f32 = 10.0;
	let n:f32 = 5.0;

	// Finding amount
	let a = p * (1.0 + (r / 100.0)).powf(n);
		println!("Amount is {}",a);

	// Compound interest formular
	let ci = a - p;
		println!("compound interest is {}",ci );
}