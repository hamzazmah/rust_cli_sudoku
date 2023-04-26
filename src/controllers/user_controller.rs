pub mod user_controller {
    use std::fs::File;
    use std::io::BufReader;
    use serde_json;
    use crate::models::User;

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

    pub fn get_user(name: &str) -> User {
        let users = get_users();
        return users.iter().find(|u| u.name == name).unwrap_or(&User::new(name)).clone();         
    }
    
    pub fn add_update_user(user: &User) {
        let mut users = get_users();
        if !users.iter().any(|u| u.name == user.name) {
            users.push(user.clone());
            let file = File::create("users.json").expect("Unable to create file");
            serde_json::to_writer_pretty(file, &users).expect("Unable to write to file");
        } else if let Some(index) = users.iter().position(|u| u.name == user.name) {
            users[index] = user.clone();
            let file = File::create("users.json").expect("Unable to create file");
            serde_json::to_writer_pretty(file, &users).expect("Unable to write to file");
        }
    }
}