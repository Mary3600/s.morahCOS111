fn main() {
	//declare all my variables for amount

	 let ts:f64 = 450_000.00;
	 let ma:f64 = 1_500_000.00;
	 let hpa:f64 = 750_000.00;
	 let da:f64 = 2_850_000.00;
	 let aa:f64 = 250_000.00;

	 // declare my variables for quantity

	 let tq:f64 = 2.0;
	 let mq:f64 = 1.0;
	 let hpq:f64 = 3.0;
	 let dq:f64 = 3.0;
	 let aq:f64 = 1.0;

	 // calculate sum 

	 let sum_1:f64 = ts + ma + hpa + da + aa;
	 let sum_2:f64 = tq + mq + hpq + dq + aq;

	 // calculate my average

	 let average:f64 = sum_1/sum_2;

	 //print out the commands

	 println!("The total sales made is {},the amount sold {},and the average of sales made is {}", sum_1,sum_2,average);
}