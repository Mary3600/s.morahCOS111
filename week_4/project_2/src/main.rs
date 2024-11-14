use std::io;

fn main() {
    println!("\nKindly answer the followng question in order to allow us determine your incentive for the year");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nWhat is your name?");
    io::stdin().read_line(&mut input1).expect("Invalid input");

    println!("\nEnter your age:");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let age:u32 = input2.trim().parse().expect("Not a number");

    println!("\nYou are experienced (True or False) - ");
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let experience:bool = input3.trim().parse().expect("Invalid boolean");

    if experience == true && age >= 40
    {
        println!("\n{}, your annual incentive is N1_560_000", input1);
    }
    else if experience == true && age >= 30 && age < 40
    {
        println!("\n{}, your annual incentive is N1_480_000", input1);
    }
    else if experience == true && age <= 29
    {
        println!("\n{}, your annual inentive is N1_300_000", input1);
    }
    else{
        println!("\n{}, your yearly incentive is N100_000", input1);
    }
}
