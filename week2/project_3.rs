fn main(){
	// declare your variables

	let p:f64 = 510_000.00;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// Amount after 3 years 
	let a = p * (1.0 - (r / 100.0)) * t;

	/* depreciation
	let d = p - a;
	*/

	println!("The tv depreciated in value to {} after 3 years", a);
}