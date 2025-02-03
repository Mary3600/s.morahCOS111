struct Developer {
    name: String,
    experience: u32,
}

fn find_most_experienced(devs: Vec<Developer>) -> Option<Developer> {
    devs.into_iter().max_by_key(|d| d.experience)
}

fn main() {
    let developers = vec![
        Developer {
            name: "Alice".to_string(),
            experience: 5,
        },
        Developer {
            name: "Bob".to_string(),
            experience: 10,
        },
        Developer {
            name: "Charlie".to_string(),
            experience: 7,
        },
    ];

    if let Some(most_experienced) = find_most_experienced(developers) {
        println!(
            "The most experienced developer is {} with {} years of experience.",
            most_experienced.name, most_experienced.experience
        );
    } else {
        println!("No developers found.");
    }
}
