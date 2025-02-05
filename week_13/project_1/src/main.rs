
use std::io;
use std::io::Read;

fn main() {
    println!("Wecome to Globacom Ltd");
    println!("What user are you?");
    println!("1. Administrator \n 2. Project manager \n 3. Employee \n 4. Customer \n 5. Vendor");

    let mut input = String::new(); //User type
    io::stdin().read_line(&mut input).expect("Kindly pick a number");
    let user_type:u32 = input.trim().parse().expect("Invalid entery");

    if user_type == 1
    {
        db_structure()
    }
    else if user_type == 2
    {
        project_table()
    }
    else if user_type == 3
    {
        staff_table()
    }
    else if user_type == 4
    {
        customer_table()
    }
    else if user_type == 5
    {
        data_plan_table()
    }
}

fn db_structure() {
    
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    
}
fn project_table() {
    
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    
}
fn staff_table() {
    
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    
}
fn customer_table() {
    
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    
}
fn data_plan_table() {
    
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    
}

/*
use std::io;
use std::io::Read;

fn main() {
    println!("Welcome to Globacom Ltd");
    println!("What user are you?");
    println!("1. Administrator\n2. Project manager\n3. Employee\n4. Customer\n5. Vendor");

    let mut input = String::new(); // User type
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let user_type: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid entry, please enter a valid number.");
            return;
        }
    };

    if user_type == 1 {
        db_structure();
    } else if user_type == 2 {
        project_table();
    } else if user_type == 3 {
        staff_table();
    } else if user_type == 4 {
        customer_table();
    } else if user_type == 5 {
        data_plan_table();
    } else {
        println!("Invalid choice, please enter a number between 1 and 5.");
    }
}

fn db_structure() {
    let mut file = std::fs::File::open("globacom_dbase.sql").expect("Error: Could not open 'globacom_dbase.sql'");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: Could not read file contents");
    print!("{}", contents);
}

fn project_table() {
    let mut file = std::fs::File::open("project_tb.sql").expect("Error: Could not open 'project_tb.sql'");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: Could not read file contents");
    print!("{}", contents);
}

fn staff_table() {
    let mut file = std::fs::File::open("staff_tb.sql").expect("Error: Could not open 'staff_tb.sql'");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: Could not read file contents");
    print!("{}", contents);
}

fn customer_table() {
    let mut file = std::fs::File::open("customer_tb.sql").expect("Error: Could not open 'customer_tb.sql'");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: Could not read file contents");
    print!("{}", contents);
}

fn data_plan_table() {
    let mut file = std::fs::File::open("dataplan_tb.sql").expect("Error: Could not open 'dataplan_tb.sql'");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: Could not read file contents");
    print!("{}", contents);
}
*/