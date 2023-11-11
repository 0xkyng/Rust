use core::num;

fn main() {
    my_fn("This is my function");

    let result = basic_math(10, 20);
    let (multiplication, addition, subtraction) = basic_math(10, 20);

    // CODEBLOCK
    let full_name = {
        let firstname = "Isaac";
        let lastname = "Wanger";
        format!("{} {}", firstname, lastname)
    };
}

fn my_fn(s: &str) {
    println!("s");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    num1 * num2
}

// RETURNING MULTIPLE VALUES
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
 