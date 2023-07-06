use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ChorusMember {
    id: i32,
    name: String,
    first_name: String,
    last_name: String,
    full_name: String,
    nickname: Option<String>,
    username: String,
    formal_name: String,
    initials: String,
    status_id: i32,
    status: String,
    is_archived: bool,
    section_id: i32,
    section: String,
    section_split: Option<String>,
    full_section: String,
    cloudinary_image_id: Option<String>,
    website: Option<String>,
    occupation: Option<String>,
    facebook: Option<String>,
    twitter: Option<String>,
    linked_in: Option<String>,
    instagram: Option<String>,
    about_me: Option<String>,
    fun_fact: Option<String>,
    user_id: i32,
    group_names: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    street_address: Option<String>,
    street_address_2: Option<String>,
    city: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
}

fn main() {
    let singing_sections: HashSet<String> = HashSet::from([
        "T1".to_string(),
        "T2".to_string(),
        "B1".to_string(),
        "B2".to_string(),
    ]);
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let raw_json = fs::read_to_string(file_path)
        .expect("Failed to read file");
    let chorus_members: Vec<ChorusMember> = serde_json::from_str(&raw_json).unwrap();

    let mut grouped_chorus_members: HashMap<String, Vec<&ChorusMember>> = HashMap::new();

    for chorister in &chorus_members {
        if !singing_sections.contains(&chorister.section) {
            continue;
        }
        if !grouped_chorus_members.contains_key(&chorister.section) {
            let section: Vec<&ChorusMember> = Vec::new();
            grouped_chorus_members.insert(chorister.section.clone(), section);
        }
        if let Some(section) = grouped_chorus_members.get_mut(&chorister.section) {
            section.push(chorister);
        }
    }

    let sections: Vec<String> = grouped_chorus_members.into_keys().collect();
    println!("Found sections: {:?}", sections)
}
