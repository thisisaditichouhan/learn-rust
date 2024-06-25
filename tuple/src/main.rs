fn main() {
    this_is_tuple();
}

fn this_is_tuple() {
    let emp_info: (&str, u16, &str) = ("Anna", 26, "XX Fairyland");
    let fname = emp_info.0;
    let age = emp_info.1;
    let address = emp_info.2;
    println!("First name: {} | Age: {} | Address: {}", fname, age, address);
}

