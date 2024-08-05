struct Rectangle{
    pub x:f32,
    pub y:f32,
}
impl Rectangle{
    fn get_area(&self) -> f32{
        &self.x * &self.y
    }
    
}

impl Rectangle{
    fn width(&self) -> bool{
        let x2:f32 = 0.0;
        &self.x > &x2
    }
}


pub fn run()
{
    let r = Rectangle{x:1.2, y:1.2};
    let a = r.get_area();
    println!("area = {:?}", a);
}