struct Minister {
    name: String,
    ministry: String,
    geographical_zone: String,
}

fn main() {
    let mut ministers = Vec::new();

    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let geographical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Combine data into one vector
    for i in 0..names.len() {
        ministers.push(Minister {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geographical_zone: geographical_zones[i].to_string(),
        });
    }

    // Print the merged data
    println!("S/N\tNAME OF COMMISSIONER\tMINISTRY\tGEOPOLITICAL ZONE");
    for (i, minister) in ministers.iter().enumerate() {
        println!("{}\t{}\t{}\t{}", i + 1, minister.name, minister.ministry, minister.geographical_zone);
    }
}