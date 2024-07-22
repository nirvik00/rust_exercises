
#[derive(Debug)]
enum WeekDay{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}


#[derive(Debug)]
enum TravelType{
    Car(f32),
    Train(f32),
    Aeroplane(f32),
}

impl TravelType{
    fn travel_allowance(&self) -> f32 {
        let allowance = match self{
            TravelType::Car(miles) => miles*2.0,
            TravelType::Train(miles) => miles*3.0,
            TravelType::Aeroplane(miles) => miles*5.0,
        };
        allowance
    }
}

#[derive(Debug)]
enum Value{
    Integer(i32),
    Float(f32),
}

pub fn run()
{
    let day = WeekDay::Monday;
    println!("day {:?}", day);

    let t = TravelType::Car(32.09);
    let x= t.travel_allowance();
    println!("travel allowance = {}", x);

    let v= vec![Value::Integer(3), Value::Float(43.0)];
    for i in v{
        match i {
            Value::Integer(num) => println!("int {:?}", i),
            Value::Float(num) => println!("float {:?}", i),
        }
    }


}
