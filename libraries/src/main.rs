use libraries::{Customer,Category, Order, Product};
fn main() {
    let product = Product::new(1, String::from("Laptop"), 499.99, Category::Electronics);

    let customer = Customer::new(1, String::from("Isaac"), String::from("codekng@gmail.com"));

    let order = Order::new(1,product, customer, 20);
    println!("Total cost of the order is: ${}", order.total_bill())
}
