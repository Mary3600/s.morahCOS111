use std::io;

fn main(){

	println!("\nKindly fill in the following information about your siblings.");

	let mut input1 = String::new(); // no_of_siblings
	let mut input2 = String::new(); //first name
	let mut input3 = String::new(); //age of sibling
	let mut input4 = String::new(); //gender of sibling
	let mut input5 = String::new(); // country of residence
	let mut input6 = String::new(); // relationship status
	let mut input7 = String::new(); //children
	let mut input8 = String::new(); // number of children
	let mut input9 = String::new(); // number of children
	let mut input10 = String::new(); //child's age

	println!("\nHow many siblings do you have?");
	io::stdin().read_line(&mut input1).expect("Invalid input");
	let no_of_siblings:u32 = input1.trim().parse().expect("Not a number");

	println!("\nFor each of your siblings fill in the following information");

	for x in 0..no_of_siblings {

		println!("\nWhat is his/hers first name?");
		io::stdin().read_line(&mut input2).expect("Not a valid string");

		println!("\nWhat's his/her age?");
		io::stdin().read_line(&mut input3).expect("Invalid input");
		let age:u32 = input3.trim().parse().expect("Not a number");

		println!("\nWhat's the gender of the sibling above? (Male or Female");
		io::stdin().read_line(&mut input4).expect("Invalid input");

		println!("Which country does the following sibling reside in?");
		io::stdin().read_line(&mut input5).expect("Invalid inpuut");
		let country_of_residence = input5.trim().parse().expect("invalid input");

		if age >= 18 
	{
		println!("Is the sibling married(1), single(2), or in a relationship(3)?");
		io::stdin().read_line(&mut input6).expect("Invalid input");
		let relationship_status =input6.trim().parse().expect("Invalid input");
		let married = 1
		let single = 2
		let relatinship = 3

		  if relationship_status == 1
		  {
		  	println!("They have children? (True or false)");
		  	io::stdin().read_line(&mut input7).expect("invalid input");
		  	
		  	println!("How many children?");
		  	io::stdin().read_line(&mut input8).expect("Invalid input");
		  	let no_of_children = input8.trim().parse().expect("Please enter number of children");

		  	  for x in 0..no_of_children {

		  	  	println!("What is the child's name?");
		  	  	io::stdin().read_line(&mut input9).expect("input wrong");

		  	  	println!("how old is the child?");
		  	  	io::stdin().read_line(&mut input10);


		  	  }
		  }
	}

	}
	
}