use std::io;

fn main() {
    println!("Welcome to the geometry calculator.\nWhat are you trying to find today? \nKindly pick an option");

    let mut input = String::new(); // what to calculate
  
    println!("1.Area of trapezium \n2.Area of rhombus \n3.Area of parallelogram \n4.Area of cube \n5.Volume of cylinder");
    io::stdin().read_line(&mut input).expect("Please pick a number");
    let option:u32 = input.trim().parse().expect("Enter a number");

    if option == 1
    {
        trapezium()
    }
    else if option == 2
    {
        rhombus()
    }
    else if option == 3
    {
        parallelogram()
    }
    else if option == 4
    {
        cube()
    }
    else if option == 5
    {
        cylinder()
    }

}
fn trapezium(){
    let mut input1 = String::new(); //base1
    let mut input2 = String::new(); //base2
    let mut input3 = String::new(); //height

    println!("What is the length of the first base");
    io::stdin().read_line(&mut input1).expect("Enter the base");
    let base_1:f32 = input1.trim().parse().expect("Please enter an valid number");

    println!("What is the length of the second base");
    io::stdin().read_line(&mut input2).expect("Enter base");
    let base_2:f32 = input2.trim().parse().expect("Please enter a valid number");

    println!("What is the height of the trapezium");
    io::stdin().read_line(&mut input3).expect("Enter height");
    let height:f32 = input3.trim().parse().expect("Please enter a valid number");

    let area_of_trapezium = (height/2.0) * (base_1 + base_2); 
    println!("The area of the trapezium is {}", area_of_trapezium);
}
fn rhombus(){
    let mut input1 = String::new(); //diagonal1
    let mut input2 = String::new(); //diagonal2

    println!("What is the lenght of the first diagonal");
    io::stdin().read_line(&mut input1).expect("Enter diagonal");
    let diagonal_1:f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("What is the length of the second diagona");
    io::stdin().read_line(&mut input2).expect("Enter diagonal");
    let diagonal_2:f32 = input2.trim().parse().expect("Please enter a valid number");

    let area_of_rhombus = 0.5 * diagonal_1 * diagonal_2;
    println!("The area of the rhombus is {}", area_of_rhombus);
}
fn parallelogram(){
    let mut input1 = String::new(); //base
    let mut input2 = String::new(); //altitude

    println!("What is the base");
    io::stdin().read_line(&mut input1).expect("Enter base");
    let base:f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("What is the altitude");
    io::stdin().read_line(&mut input2).expect("Enter altitude");
    let altitude:f32 = input2.trim().parse().expect("Please enter a valid number");

    let area_of_parallelogram = base * altitude;
    println!("The area of parallelogram is {}", area_of_parallelogram);
}
fn cube(){
    let mut input1 = String::new(); // length of side

    println!("What is the length of the side");
    io::stdin().read_line(&mut input1).expect("Enter length");
    let length:f32 = input1.trim().parse().expect("Plwase enter a valid number");

    let area_of_cube = 6.0 * length.powf(2.0);
    println!("The area of cube is {}", area_of_cube);
}
fn cylinder(){
    let mut input1 = String::new(); // radius
    let mut input2 = String::new(); // height
    let pi:f32 = 3.14;

    println!("What is the radius");
    io::stdin().read_line(&mut input1).expect("Enter radius");
    let radius:f32 = input1.trim().parse().expect("Enter a valid number");

    println!("What is the height");
    io::stdin().read_line(&mut input2).expect("Enter height");
    let height:f32 = input2.trim().parse().expect("Enter a valid number");

    let volume_of_cylinder = pi * radius.powf(2.0) * height;
    println!("The Volume of cylinder is {}", volume_of_cylinder);
}
