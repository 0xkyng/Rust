mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius should be positive"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert!(larger_circle.contains(&smaller_circle), "{}", true);
    }
}