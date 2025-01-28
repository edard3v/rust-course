mod account;
use account::Account;

fn main() {
    let mut edar = Account::new("edar", "123");

    edar.change_password("asdkfasklfd");

    println!("{edar:#?}");
}
