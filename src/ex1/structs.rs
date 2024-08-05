
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple struct
struct ColorX (u8, u8, u8, u8);


struct Person {
    fname: String, 
    lname: String
}


impl Person {
    fn new(f: &str, l: &str) -> Person{
        Person {
            fname: f.to_string(),
            lname: l.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {} ", self.fname, self.lname)
    }
    fn set_last_name(&mut self, x: &str) {
        self.lname = x.to_string();
    }
    fn to_tuple(self) -> (String, String){
        (self.fname, self.lname)
    }
}


pub fn run(){

    let mut c = Color{red:100,green:200,blue:200};
    println!("{:?}", (c.red, c.green, c.blue));

    c.red=32;
    println!("{:?}", (c.red, c.green, c.blue)); 

    // tuple struct
    let mut d=ColorX(255, 0,0,23);
    println!("{:?}", (d.0, d.1, d.2, d.3)); 
    
    d.2=23;
    println!("{:?}", (d.0, d.1, d.2, d.3)); 


    // person 
    let mut p = Person::new("john", "doe");
    println!("Person {}", (p.full_name()));

    p.set_last_name("ert");
    println!("Person {}", p.full_name());
    
    println!("Person {:?}", p.to_tuple());

}