//program to find roots of a quadratic equation

use std::io;

fn main() {

    println!("\nDetermine the roots and type of root of your quadratic equation");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\ninput a: ");
    io::stdin().read_line(&mut input1).expect("Invald input");
    let a:f32 = input1.trim().parse().expect("Not a avalid number");

    println!("\ninput b: ");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\ninput3: ");
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

     let d:f32 = b.powf(2.0)- (4.0 *a*c);
    
    if d > 0.0 
    {
        println!("The eqaution has real roots");
    }
    else if d < 0.0
    {
        println!("The equation has no real roots");
    }
    else if d == 0.0 
    {
        println!("The equation has equal roots");
    }

    let x_1:f32 = -b + d.sqrt() / (2.0 * a);
    let x_2:f32 = -b - d.sqrt() / (2.0 * a);
    println!("The roots of the equation are {}, {}", x_1, x_2);
}
