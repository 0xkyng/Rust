fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("Here is the value of x: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x is: {x}");

    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // TUPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup: (i32, f64, u16) = (100, 5.5, 3);
    let tup = (200, 1.5, 88);

    // To get the individual values out of a tuple
    // use pattern matching to destructure a tuple value

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // We can also access a tuple element directly
    // by using a period (.) followed by the index of the value we want to access

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    //STRINGS
    let mut growable_string = String::from("Hello ma");
    growable_string.push('n');
    println!("The value of growable string is: {growable_string}");
    // Adding more than one character to a string
    growable_string.push_str(", Good evening");
    println!("The new growable string is: {}", growable_string);

    // BASIC OPERATIONS ON STRINGS
    println!(
        "Basic Functions on strings,
        is_Empty():{},
        length():{},
        Bytes():{},
        Contains use():{}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str(" ");
    let length = growable_string.trim().len();

    // CONVERTING NUMBERS TO STRING
    let number = 6;
    let num_str = number.to_string();
    println!();

    // CREATE AN EMPTY STRING
    let empty_string = String::new();
    println!("The lenth of the empty string is: {}", empty_string.len());

    // STRING CONCATENATION
    let first_name = "Isaac".to_string();
    let last_name = "Wanger".to_string();
    let full_name = format!("My first name is {} and my last name is {}", first_name, last_name);
    println!("{}", full_name);
    let concat = format!("{}{}", first_name, last_name);

    ///////////////////////////////////////////////////////////
    

    // TUPLES
     
    let my_information = ("salary", 5000);
    println!(" My {} is equal to {}", my_information.0,my_information.1);

    println!("This is another way to print a whole tuple {:?}", my_information);

    // DESTRUCTURING TUPLES
    //1.
    let (salary, salary_value) = my_information;

    //2.
    let salary = my_information.0;
    let salary_value = my_information.1;

    // NESTED TUPLE
    let nested_tuple = (4,5.0,(3,2), "Hello");
    let destructure_third_element = nested_tuple.2.0;

    //xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    
    // ARRAYS
    let mut first_array  = [1,2,3,4,5];

    println!("{}", first_array[0]);
    println!("{:?}", first_array);

    first_array[3] = 10;

    let array_with_same_elements = [0;10];

    // SLICES
    let mut number_array = [4,5,6,8,9];

    let sub_array = &number_array[0..3];
    println!("The subset values of the array are {:?}", sub_array);
}