// P.M. Okeke Sons Ltd Sales Record

fn main() {
	let toshiba: f64 = 2.00;
	let mac: f64 = 1.00;
	let hp: f64 = 3.00;
	let dell: f64 = 3.00;
	let acer: f64 = 1.00;

	let toshiba_a: f64 = 450_000.00;
	let mac_a: f64 = 1_500_000.00;
	let hp_a: f64 = 750_000.00;
	let dell_a: f64 = 2_850_000.00;
	let acer_a: f64 = 250_000.00;

	// Price of each product
	let toshiba_p: f64 = toshiba * toshiba_a;
	let mac_p: f64 = mac * mac_a;
	let hp_p: f64 = hp * hp_a;
	let dell_p: f64 = dell * dell_a;
	let acer_p: f64 = acer * acer_a;

	// Sum of quantity and amount
	let sum_amt: f64 = toshiba_p + mac_p + hp_p + dell_p +acer_p;
	let sum_qty: f64 = toshiba + mac + hp + dell + acer;

	//Average of the sales record
	let avg: f64 = sum_amt/sum_qty;

	// The Results
	println!("The Results of P.M. Okeke and Sons Ltd Sales Record");
	println!("The total number of products available is {}", sum_qty);
	println!("The Amount of the products is {}",sum_amt);
	println!("Finally, the average of the sales record is {}",avg );
	
}