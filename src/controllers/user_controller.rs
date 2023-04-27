/// This code defines a module named `user_controller` that contains functions for managing user data.
/// It imports several modules from the standard library (`std::fs::File`, `std::io::BufReader`) and the
/// `serde_json` crate for working with JSON data and importing exporting JSON data. It also imports the `User` struct from the `models`
/// module.
pub mod user_controller {
    use std::fs::File;
    use std::io::BufReader;
    use serde_json;
    use crate::models::User;

    /// This function reads the `users.json` file and returns a `Vec<User>` containing the users in the file 'users.json'.
    fn get_users() -> Vec<User> {
        let file = match File::open("users.json") {
            Ok(f) => f,
            Err(_) => {
                let mut new_file = File::create("users.json").expect("Unable to create file");
                serde_json::to_writer_pretty(&mut new_file, &Vec::<User>::new()).expect("Unable to write to file");
                File::open("users.json").expect("Unable to open file")
            }
        };
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    }

    /// This function returns a `User` with the given name if it exists in the `users.json` file. Otherwise, it returns a new `User` with the given name.
    pub fn get_user(name: &str) -> User {
        let users = get_users();
        return users.iter().find(|u| u.name == name).unwrap_or(&User::new(name)).clone();         
    }
    
    /// This function adds a `User` to the `users.json` file if it does not already exist. If it does exist, it updates the `User` with the updated user object.
    pub fn add_update_user(user: &User) {
        let mut users = get_users();
        // If the user does not exist, add it to the users vector
        if !users.iter().any(|u| u.name == user.name) {
            users.push(user.clone());
            // Get or Create the file 'users.json'
            let file = File::create("users.json").expect("Unable to create file");
            serde_json::to_writer_pretty(file, &users).expect("Unable to write to file");
        } else if let Some(index) = users.iter().position(|u| u.name == user.name) { // If the user exists, update it
            users[index] = user.clone();
            let file = File::create("users.json").expect("Unable to create file");
            serde_json::to_writer_pretty(file, &users).expect("Unable to write to file");
        }
    }
}