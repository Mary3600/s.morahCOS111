/*use std::io;

fn main() {
    println!("Welcome to our resturant");
    println!("The items available on our menu today are:");
    println!("      ITEM \t                         PRICE");

    println!("P= Poundo potato/Edinkaiko soup \t N3200");
    println!("F= Fried Rice & Chicken \t         N3000");
    println!("A= Amala & Ewedu Soup \t                 N2500");
    println!("E= Eba & Egusi soup \t                 N2000");
    println!("W = White Rice & Stew \t                 N2500");

    let mut food_choice = String::new();
    println!("What will you like to eat today?");
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");
    
    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read line");

    let quantity: i32 = quantity.trim().parse().expect("Invalid quantity");

    let price = match & food_choice.trim().to_uppercase().as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => 0,
    };

    let mut total_cost = price * quantity;

    if total_cost > 10000 {
        total_cost -= total_cost * 0.05; // Apply 5% discount
    }

    println!("Total cost: N{}", total_cost);
}
*/
use std::io;

fn main() {
    println!("Welcome to our resturant");
    println!("The items available on our menu today are:");
    println!("      ITEM \t                         PRICE");

    println!("P= Poundo potato/Edinkaiko soup \t N3200");
    println!("F= Fried Rice & Chicken \t         N3000");
    println!("A= Amala & Ewedu Soup \t                 N2500");
    println!("E= Eba & Egusi soup \t                 N2000");
    println!("W = White Rice & Stew \t                 N2500");

    let mut food_choice = String::new();
    println!("What would you like to eat today?:");
    io::stdin().read_line(&mut food_choice).expect("Failed to read line");

    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read line");

    let quantity: f32 = quantity.trim().parse().expect("Invalid quantity");

    let price = match food_choice.trim().to_uppercase().as_str() {
        "P" => 3200.00,
        "F" => 3000.00,
        "A" => 2500.00,
        "E" => 2000.00,
        "W" => 2500.00,
        _ => 0.00,
    };

    let mut total_cost:f32 = price * quantity;

    if total_cost > 10000.0 {
        total_cost -= total_cost * 0.05; // Apply 5% discount
    }

    println!("Total cost: N{}", total_cost);
}