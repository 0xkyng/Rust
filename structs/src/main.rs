struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn display_car_info(&self) {
        println!("Owner: {}, Year: {}, Price: {}", self.owner, self.year, self.price);
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    //  In Rust, when a method takes self as a parameter, it indicates that the method is consuming the object it is called on, meaning it takes ownership of that object.
    fn sell(self) -> Self {
        self
    }

    // Associated function
    fn monthly_insurance() -> u32 {
        123
    } 

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year:year,
            fuel_level: 0.0,
            price: 0,
        }
    }
}

fn main() {
    let mut my_car:Car = Car { 
    owner:String::from("ABC"), 
    year: 2020, 
    fuel_level: 0.0,
    price: 5_000
   };


   let car_year = my_car.year;
   my_car.fuel_level = 30.0;
   let new_car_owner = my_car.owner.clone();

   // create a new instance of the Car struct, named another_car,
   let another_car:Car = Car {
        owner: "new_name".to_string(),
        // You can use ".." to include other fields from `my_car` if needed
        ..my_car
   };

   // TUPLE STRUCTS
   struct point_2D(i32, i32);
   struct  point3D(i32,i32, i32);

   let point1 = point_2D(1,2);
   //slet point2 = point3D(4, 5, 10);


   my_car.display_car_info();
   my_car.refuel(10.5);
   let new_owner = my_car.sell();

   let new_car = Car::new("izik".to_string(), 2024);

}


