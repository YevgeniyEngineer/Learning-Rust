extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    is_active: bool,
    preferences: HashMap<String, String>,
}

fn main() -> serde_json::Result<()> {
    // Create an instance of a user
    let mut user_preferences = HashMap::new();
    user_preferences.insert("theme".to_string(), "dark".to_string());
    user_preferences.insert("language".to_string(), "en".to_string());

    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        email: "johndoes@example.com".to_string(),
        is_active: true,
        preferences: user_preferences,
    };

    // Serialize the User to a JSON string
    let serialized = serde_json::to_string(&user)?;
    println!("Serialized User: {}", serialized);

    // Deserialize the JSON string back to a User instance
    let deserialized: User = serde_json::from_str(&serialized)?;
    println!("Deserialized User: {:#?}", deserialized);

    Ok(())
}
