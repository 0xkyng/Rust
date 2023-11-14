fn main() {
   // --------------------------------------
   //          Ownership in Functions
   // --------------------------------------

   let vec_1 = vec![1,2,2];
   takes_ownership(vec_1.clone());
   println!("vec 1 is: {:?}", vec_1);

   let vec_2 = gives_ownership();
   println!("Vec 2 is: {:?}", vec_2);

   let vec_3 = takes_and_gives_ownership(vec_2);
   //println!("vec 2 is: {:?}", vec_2);
   println!("vec 3 is: {:?}", vec_3)
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec);
}
// Moving ownership out of the funtion
fn gives_ownership() -> Vec<i32> {
    vec![4,5,6]
}

// Combine giving & Taking Ownership
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}
