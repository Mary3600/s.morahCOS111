/*Rust code to read height of person,and
determine if they're tall, dwarf or average height*/

use std::io;

fn main()
{
    let mut input = String::new();

    println!("\nEnter your height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are an averaged height person");
    }
    else if height >170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height <150.0 && height >100.0
    {
        println!("You are a dwarf");
    }
    else {
        println!("Omor your height is abnormal");
    }
    
}
