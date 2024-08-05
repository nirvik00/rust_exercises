use std::fmt;

struct Structure(i32);
impl fmt::Display for Structure{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "out: {}", self.0)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate()
        {
            if count!=0{
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
            
        }
        write!(f, "]")
    }
}


pub fn run(){
    println!("1e4 is {} and 2e-4 is {}", 1e4, 2e-4);

    let s=Structure(1);
    println!("{}", s);

    let l = List(vec![1, 2, 3]);
    println!("{}", l);

}


