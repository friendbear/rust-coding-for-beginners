// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    RitchhenStaff,
    AssemblyTech,
}

#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
#[derive(Debug)]
struct Employee {
    position: Position,
    status: Status,
}

fn try_access(employee: &Employee) -> Result<(), String> {

    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}
fn main() {

    let manager = Employee {
        position: Position::Manager,
        status: Status::Terminated,
    };

    if let Err(e) = try_access(&manager) {
        println!("Access denied: {:?}", e);
    } else {
        println!("Access allowed: {:?}", &manager);
    }
}
