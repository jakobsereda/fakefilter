use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufReader},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(default)]
    relationships_following: Vec<Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(default)]
    title: String,
    #[serde(default)]
    media_list_data: Vec<serde_json::Value>,
    string_list_data: Vec<ListItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListItem {
    href: String,
    value: String,
    timestamp: i64,
}

fn main() -> io::Result<()> {
    let following_json = File::open("data/connections/followers_and_following/following.json")?;
    let followers_json = File::open("data/connections/followers_and_following/followers_1.json")?;

    let following_bufr = BufReader::new(following_json);
    let followers_bufr = BufReader::new(followers_json);

    let following_root: Root = serde_json::from_reader(following_bufr)?;
    let following: Vec<Profile> = following_root.relationships_following;
    let followers: Vec<Profile> = serde_json::from_reader(followers_bufr)?;

    let followers_set: HashSet<String> = followers
        .iter()
        .flat_map(|p| p.string_list_data.iter().map(|i| i.value.clone()))
        .collect();

    let not_following: Vec<&Profile> = following
        .iter()
        .filter(|p| {
            p.string_list_data
                .iter()
                .all(|i| !followers_set.contains(&i.value))
        })
        .collect();

    println!("==== FAKE FILTER ====");
    let mut i = 1;
    for profile in not_following {
        for item in &profile.string_list_data {
            println!("{}. {}", i, item.value);
            i += 1;
        }
    }

    Ok(())
}
