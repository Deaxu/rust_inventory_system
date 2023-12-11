use std::collections::HashMap;
pub struct UserDatabase {
    users: HashMap<String, String>,
}

impl UserDatabase{
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(String::from("admin"), String::from("admin123"));
        users.insert(String::from("kullanici1"), String::from("sifre1"));
        users.insert(String::from("kullanici2"), String::from("sifre2"));

        UserDatabase { users }
    }

    pub fn get_password(&self, username: &str) -> Option<&String> {
        self.users.get(username)
    }
}