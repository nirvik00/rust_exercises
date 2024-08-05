struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(n:&String, db:&Vec<Student>) -> Option<u32>{

    for s in db
    {
        if s.name == *n{
            return s.grade;
        }
    }
    None
}

pub fn run() 
{
    let student_db= vec![
        Student{
            name:"abcd".to_string(),
            grade:Some(32),
        },
        Student{
            name:"efgh".to_string(),
            grade:Some(89),
        },
        Student{
            name:"ijkl".to_string(),
            grade:None,
        }
    ];

    let n = String::from("efgh");
    let x= get_grade(&n, &student_db);
    // match x{
    //     Some(x) => println!("grade is {}", x),
    //     _=>{},
    // }

    if let Some(g) = x {
        println!("grade is {}", g);
    }

}

// enum Option<T>{
//     None, 
//     Some(T)
// }