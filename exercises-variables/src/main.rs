
fn main() {
    // 1.  In this exercise, you will create two tuples, p1 and p2, representing points on a Cartesian plane. Each tuple will contain two values, one for the x-axis and another for the y-axis. Your task is to write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

    //Note: To calculate the absolute value, use the abs() function. The compiler may show an error message stating "ambiguous numeric type" when using this function. To resolve this, ensure that you specify the value as f64 by writing "as f64" in front of it. For example, (-3.5 as f64).abs() will result in a value of 3.5.

    //Declare two tuples, p1 and p2, to represent the coordinates of the two points. Initialize p1 with the values (4.0, 3.0) and p2 with the values (5.0, 4.5).

    // Declare and initialize the tuples
    let p1 = (4.0, 3.0);
    let p2 = (5.0, 4.5);

    // Calculate the absolute differences
    let x_diff = (p1.0 - p2.0 as f64).abs();
    let y_diff = (p2.1 - p2.1 as f64).abs();

    println!("Absolute difference in x valuees {}", x_diff);
    println!("Absolute difference in y values {}", y_diff);


    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    2;
    // In this question, you will solve the same problem as discussed in Question 1, but using arrays.

    // First, declare two arrays called p1 and p2. These arrays will have a length of 2 and a type of f64. Each array represents a point, with the x-axis value in the first index (position) and the y-axis value in the second index (position).

    // Next, write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

    //Note: Use the abs() function to determine the absolute value of the difference. In this case, you don't need to explicitly specify the types as f64 for all the values because the compiler can infer the types correctly. The compiler is smart enough to detect the types being passed to the abs() function, so you won't encounter any issue

    // Write a statement to compute the distance between p1 and p2 using the given formula. To calculate the square root, use the sqrt() function, passing in the expression inside the parentheses. The syntax for computing the square root is number.sqrt().

    let p1 = [4.0, 3.0];
    let p2 = [5.0, 4.5];

    let x_diff = (p1[0] - p2[0] as f64).abs();
    let y_diff = (p1[1] - p2[1] as f64).abs();

    // Calculate the distance between p1 and p2 using the Euclidean distance formula
    let distance = f64::sqrt(x_diff.powi(2) + y_diff.powi(2));

    // Display the results
    println!("Absolute difference in x values {}", x_diff);
    println!("Absolute difference in y values {}", y_diff);
    println!("Distance between p1 and p2: {}", distance);

}



