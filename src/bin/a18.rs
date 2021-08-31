// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer {
    age: u32,
}

fn try_purchase(customer: &Customer, f: fn(u32) -> bool) -> Result<(), String> {

    if f(customer.age) {
        Err("customer must be at least purchase age".to_owned())
    } else {
        Ok(())
    }
}
fn main() {
    let age_check = |age: u32| age < 21;

    let ashley = Customer { age: 12 };
    let purchased = try_purchase(&ashley, age_check);
    println!("{:?}", purchased);
}
