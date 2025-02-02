fn main() {
    let mut edar = Account {
        username: "edard3v",
        password: "123",
    };

    edar.password = "asdfs";
    edar.reset_password("asdfsdf");

    println!("{edar:?}")
}

#[derive(Debug)]
#[expect(dead_code)]
struct Account {
    username: &'static str,
    password: &'static str,
}

impl Account {
    fn reset_password(&mut self, new_password: &'static str) {
        self.password = new_password
    }
}
