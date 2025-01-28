#[derive(Debug)]
#[allow(dead_code)]
pub struct Account {
    username: String,
    password: String,
}

impl Account {
    pub fn new(username: &str, password: &str) -> Self {
        Account {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn change_password(&mut self, new_password: &str) {
        self.password = new_password.to_string()
    }
}
