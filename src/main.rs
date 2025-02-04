fn main() {
  trait Printable {
    fn print(&self, txt: &str) {
      println!("{txt}");
    }
  }

  struct Animal;
  impl Printable for Animal {}

  struct Account;
  impl Printable for Account {
    fn print(&self, txt: &str) {
      println!("***************{txt}***********")
    }
  }

  let x = Animal {};
  let y = Account {};

  x.print("Animal");
  y.print("Account");
}
