enum Role {
  Admin,
  Manager,
  User,
  Guest
}

fn main() {
  let role = Role::Guest;

  let can_access_file = match role {
    Role::Admin => true,
    _=> false,
  };

  println!("Can access file: {:?}", can_access_file);
}