// P.M. Okeke Sons Ltd Sales Record

fn main() {
	let toshiba = 2.00;
	let mac = 1.00;
	let hp = 3.00;
	let dell = 3.00;
	let acer = 1.00;

	let toshiba_a = 450_000.00;
	let mac_a = 1_500_000.00;
	let hp_a = 750_000.00;
	let dell_a = 2_850_000.00;
	let acer_a = 250_000.00;

	// Price of each product
	let toshiba_p = toshiba * toshiba_a;
	let mac_p = mac * mac_a;
	let hp_p = hp * hp_a;
	let dell_p = dell * dell_a;
	let acer_p = acer * acer_a;

	// Sum of quantity and amount
	let sum_amt = toshiba_p + mac_p + hp_p + dell_p +acer_p;
	let sum_qty = toshiba + mac + hp + dell + acer;

	//Average of the sales record
	let avg = sum_amt/sum_qty;

	// The Results
	println!("The Results of P.M. Okeke and Sons Ltd Sales Record");
	println!("The total number of products available is {}", sum_qty);
	println!("The Amount of the products is {}",sum_amt);
	println!("Finally, the average of the sales record is {}",avg );
	
}