fn main() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Account {
        username: String,
        password: String,
    }

    impl Account {
        fn new(username: &str, password: &str) -> Self {
            Account {
                username: username.to_string(),
                password: password.to_string(),
            }
        }

        fn change_password(&mut self, new_password: &str) {
            self.password = new_password.to_string()
        }
    }

    let mut edar = Account::new("edar", "123");

    edar.change_password("asdkfasklfd");

    println!("{edar:#?}");
}
