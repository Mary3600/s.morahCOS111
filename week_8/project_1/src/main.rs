use std::collections::HashMap;

struct Staff {
    role: String,
    experience: u8,
}

fn get_aps_level(role: &str, experience: u8) -> &'static str {
    let mut aps_levels: HashMap<&str, Vec<(u8, u8, &str)>> = HashMap::new();
    
    aps_levels.insert("Paralegal", vec![(0, 2, "APS 1-2")]);
    aps_levels.insert("Junior Associate", vec![(3, 5, "APS 3-5")]);
    aps_levels.insert("Associate", vec![(5, 8, "APS 5-8")]);
    aps_levels.insert("Senior Associate 1-2", vec![(8, 10, "EL1 8-10")]);
    aps_levels.insert("Senior Associate 3-4", vec![(10, 13, "EL2 10-13")]);
    aps_levels.insert("Partner", vec![(13, 50, "SES")]);
    
    if let Some(levels) = aps_levels.get(role) {
        for &(min_exp, max_exp, aps) in levels {
            if experience >= min_exp && experience <= max_exp {
                return aps;
            }
        }
    }
    "Unknown"
}

fn main() {
    let staff = Staff {
        role: "Associate".to_string(),
        experience: 6,
    };
    
    let aps_level = get_aps_level(&staff.role, staff.experience);
    println!("The APS level for a {} with {} years of experience is: {}", staff.role, staff.experience, aps_level);
}
