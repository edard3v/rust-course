fn main() {
    #[derive(Debug)]
    #[expect(dead_code)]
    enum Role {
        Admin = 0,
        Client = 99,
        Boss,
    }

    fn print_role(role: Role) {
        println!("{:?}", role as u32)
    }

    print_role(Role::Admin);
}
