use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, i: i32, s: Student) -> Result<(), String> {
        if self.students.contains_key(&i) {
            return Err("key already exists".to_string());
        }
        self.students.insert(1, s);
        Ok(())
    }

    fn get_student(&mut self, i: i32) -> bool{
        if self.students.contains_key(&i) {
            return true;
        } else {
            return false;
        }
    }

    fn display(&mut self) {
        for (i, n) in &self.students {
            let name = &n.name;
            println!("{i}, {name}");
        }
    }
}

pub fn main() {
    let s: Student = Student {
        id: 12,
        name: "a".to_string(),
        grade: "A".to_string(),
    };

    let mut sm = StudentManager::new();
    let r = sm.add_student(1, s);
    match r {
        Ok(()) => {
            println!("successfully added student");
        }
        Err(e) => {
            println!("unable to add student");
        }
    }
    
    sm.display();

    let t = sm.get_student(1);
    if t==true{
        println!("student exists");
    }
}
