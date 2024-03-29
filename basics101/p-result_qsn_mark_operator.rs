// Topic: Result & the question mark operator
//
// Summary:
//   This small program simulates unlocking a door using digital keycards
//   backed by a database. Many errors can occur when working with a database,
//   making the question mark operator the perfect thing to use to keep
//   the code managable.


#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        // In a production application, a database connection error is likely to occur here.
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(String::from("employee not found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;
    // Above expression with ? is same as below
    // let db = match Database::connect() {
    //   Ok(db) => db,
    //   Err(e) => return Err(e),
    // }

    let employee = db.find_employee(employee_name)?;
    let keycard = db.get_keycard(&employee)?;

    if keycard.access_level >= location.required_access_level() {
      Ok(AuthorizationStatus::Allow)
    } else {
      Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    // Anita is trying to access the Warehouse, which requires access level 500.
    // Her keycard has access level 1000, which should be allowed.
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    // Brody is trying to access the Office, which requires access level 800.
    // His keycard has access level 500, which should be denied.
    let brody_authorized = authorize("Brody", ProtectedLocation::Office);
    // Catherine is trying to access the Warehouse, which requires access level 500.
    // She doesn't have a keycard, so this should be an error.
    let catherine_authorized = authorize("Catherine", ProtectedLocation::Warehouse);

    println!("{anita_authorized:?}");
    println!("{brody_authorized:?}");
    println!("{catherine_authorized:?}");
}
