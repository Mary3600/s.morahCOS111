use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Filed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <='9'
    {
        println!("Character '{}' ia a digit",ch);
    }
    else
    {
        println!("Character '{}' is not a digit",ch);
    }
}

fn main(){
    // calling function
    println!("Welcome! This program checkswheter a character variablecontains a digit or not");
    checker()
}