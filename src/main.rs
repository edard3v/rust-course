fn main() {
    #[derive(Debug)]
    #[expect(dead_code)]
    enum Role {
        Admin(String),
        Client { id: u32, display: String },
        Boss(u32, String),
    }

    impl Role {
        fn print(&self) {
            match self {
                Role::Admin(display) => println!("{display}"),
                Role::Client { id, display } => println!("{id} {display}"),
                Role::Boss(id, display) => println!("{id} {display}"),
            }
        }
    }

    let boss = Role::Boss(1, "BOSS".to_string());
    boss.print();
}
