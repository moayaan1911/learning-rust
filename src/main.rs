// fn main() {
//     // let num:u8 = 10;
//     // println!("Hello, China! {}", num);

//     // STRING
//     // let string_literal_static : &str = "Hello Everyone"; // stored in stack special memory, its static
//     // println!( "{}", string_literal_static);

//     // let mut string_literal_dynamic : String = String::from("Hello Everyone"); // String stored in heap, its dynamic
//     // string_literal_dynamic.push_str(", China!");
//     // println!( "{}", string_literal_dynamic);

//     // TUPLE
//     // let emp_value:(&str, u8) = ("Tom", 32);
//     // println!("Employee name is {} and age is {}", emp_value.0, emp_value.1);

//     // FUNCTIONS
//     // let number1: u8 = 10;
//     // let number2: u8 = 20;
//     // let result: u8 = sum(number1, number2);
//     // println!("Sum of {} and {} is {}", number1, number2, result);
// }

// fn sum(number1: u8, number2: u8) -> u8 {
//     return number1 + number2;
// }

// OWNERSHIP
fn main() {
    let s1: String = get_string();
    println!("The value of s1 is {}", s1);

    let s2: String = String::from("World");

    let s3: String = send_get_string(s2);
    // println!("The value of s2 is {}", s2); // THIS WILL THROW ERROR
    println!("The value of s3 is {}", s3);
}

fn get_string() -> String {
    let s: String = String::from("Hello");
    return s;
}

fn send_get_string(s: String) -> String {
    return s;
}
