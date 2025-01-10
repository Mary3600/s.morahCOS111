use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    let mut students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE10202020".to_string(),
            department: "Electrical".to_string(),
            level: 400,
        },
        Student {
            name: "Blane Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Display student details
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for student in &students {
        println!("{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level);
    }

    // Save student details to CSV file
    let mut file = File::create("students.csv").expect("Failed to create file");
    file.write_all(b"Student Name,Matric. Number,Department,Level\n").expect("Failed to write to file");

    for student in &students {
        let line = format!("{},{},{},{}\n", student.name, student.matric_number, student.department, student.level);
        file.write_all(line.as_bytes()).expect("Failed to write to file");
    }
}