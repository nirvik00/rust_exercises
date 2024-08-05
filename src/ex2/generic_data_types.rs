

struct Point<T, U>{
    x:T,
    y:U,
}
impl Point<i8, f32>{
    fn display(&self){
        println!("x= {}, y={}", &self.x, &self.y);
    }
}


pub fn run(){
    let p = Point{x:1, y:3.0};
    p.display();

    let a = Some(5);
    let b = 10;
    let c = a.unwrap() + b;
    println!("{:?}", c);
}