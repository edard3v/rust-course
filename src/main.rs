fn main() {
  struct Animal;
  impl Animal {
    fn print(&self) {
      println!("Animal");
    }
  }

  struct Account;
  impl Account {
    fn print(&self) {
      println!("Account");
    }
  }

  let x = Animal {};
  let y = Account {};

  x.print();
  y.print();
}
