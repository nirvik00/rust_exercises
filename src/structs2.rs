struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn new(name: String, year: u32) -> Self {
        Self {
            owner:name.to_string(),
            year: year,
            fuel_level: 234.0,
            price: 123,
        }
    }

    fn display_car_info(&self) {
        println!(
            "owner : {}, year: {}, price: {}, fuel_level: {}",
            self.owner, self.year, self.price, self.fuel_level
        )
    }

    fn update_fuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }

    // associated method == static method
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }
}

pub fn run() {
    let mut my_car = Car {
        owner: "abcd".to_string(),
        year: 23,
        fuel_level: 234.0,
        price: 123,
    };

    {
        let _a = my_car.year;
        my_car.fuel_level = 323.0;
        let b = my_car.owner.clone();
        println!("owner {}", my_car.owner);
    }

    let car2 = Car {
        owner: "new_name".to_string(),
        ..my_car
    };

    // println!("owner {}", my_car.owner);
}

pub fn run2() {
    let p1 = (1, 3);
    let p2 = (4, 10, 12);

    // tuple structs
    struct pt2d(i32, i32);
    struct pt3d(i32, i32, i32);

    let p01 = pt2d(1, 2);
    let p02 = pt3d(1, 2, 4);

    // unit structs
    struct ABC;
}

pub fn run3() {
    let mut car = Car {
        owner: "abcd".to_string(),
        year: 23,
        fuel_level: 234.0,
        price: 123,
    };
    car.update_fuel(32.2);
    car.display_car_info();

    let sp = car.selling_price();
    println!("selling price = {} ", sp);

    Car::monthly_insurance();

    let car2 = Car::new("abcd".to_string(), 2020);
    car2.display_car_info();
    // let new_owner = car.sell();
}
