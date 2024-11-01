fn main() {
	let p:i64 = 520_000_000;
	let r:i64 = 10;
	let t:i64 = 5;

	//Amount after compound interest
	let a = p * ( 1 + (r / 100)) * t;
	println!("The amount received after after 5 years is {}", a);
	//Compound interest received
	let ci = a - p;
	println!("The total compound interest received is {}", ci);

}