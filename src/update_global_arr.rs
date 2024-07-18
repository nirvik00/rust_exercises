#[derive(Debug)]
pub struct Person {
    pub lname: String,
    pub fname: String,
    pub age: u8,
}

#[derive(Debug)]
pub struct Cube {
    pub l: f32,
    pub w: f32,
    pub h: f32,
    pub id: String,
}
impl Cube {
    pub fn new(l_: f32, w_: f32, h_: f32, id_: &str) -> Cube {
        Cube {
            l: l_,
            w: w_,
            h: h_,
            id: String::from(id_),
        }
    }
    pub fn vol(&self) -> f32 {
        self.l * self.w * self.h
    }

    pub fn ar(&self) -> f32 {
        2_f32 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    pub fn display(&self) -> String {
        let a = &self.id.to_string();
        let b = &self.l.to_string();
        let c = &self.w.to_string();
        let d = &self.h.to_string();
        let x = format!("{} {} {} {}", a, b, c, d);
        x
    }
}

pub fn run(s: &mut String, x: &mut i32) {
    println!("{:?}", (&s, &x));
    s.push_str(", world!");
    *x += 1;
    *x *= 89274;
}

pub fn per(p: &mut Person) {
    p.age = 19;
}
