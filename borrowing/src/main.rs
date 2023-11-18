fn main() {
    // |-------------------------------|
    // |       Borrowing               |
    // |-------------------------------|
    let mut vec_1 = vec![1, 2, 3];
    let ref_1 = &vec_1;
    borrows_vec(ref_1);
    println!("Vec 1 is: {:?}", vec_1);

    let re2 = &mut vec_1; 
    mutably_borrows_vec(re2);

    let mut numbers = vec![1, 2, 3];
    let slice = get_slice(&mut numbers);
    numbers.push(4);
    println!("Slice: {:?}",slice);



    // |---------------------------------|
    // |            Dereferencing        |
    // |---------------------------------|
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let value = *ref_1;
    *ref_1 = 13;
    println!("Some_data is:{some_data}, value:{value}");


    let s1 = String::from("Hello");
    let s2 = &s1; 
    let s3 = &s2;     
    let s4 = &s3;

    ***s4 == "Hello".to_string();

    // The variable s2 is single reference. The variable s3 is a second order reference and the variable s4 is a third order reference. Since adding each star will deref one of the reference so therefore we will use three stars to deref the actual contents pointed to by s4. 

     
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("Vec is: {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn get_slice(numbers: &mut Vec<i32>) -> Vec<i32> {
    numbers[0..2].to_vec()
}

