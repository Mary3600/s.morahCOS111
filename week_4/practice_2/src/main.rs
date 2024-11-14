// program to calculate area of triangle given 3 sides
use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter first edge of triangle: ");
   io::stdin().read_line(&mut input1).expect("Invalid entry");
   let a:f32 = input1.trim().parse().expect("Not a a valid number");

   println!("Enter secomd edge of triangle: ");
   io::stdin().read_line(&mut input2).expect("Invalid entry");
   let b:f32 = input2.trim().parse().expect("Not a valid numer");

   println!("Enter third edge of triangle: ");
   io::stdin().read_line(&mut input3).expect("Invalid entry");
   let c:f32 = input3.trim().parse().expect("Not a valid number");

   let s:f32 = (a * b *c) / 2.0;
   let mut area:f32 = s * (s - a) * (s - b) * (s - c);
   let area = (&mut area).sqrt();

   println!("The area of the triangle is {}", area);

}
