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


fn check_student(n:&String, db:&Vec<Student>) -> Result<(), String>{
    for s in db
    {
        if s.name == *n{
            return Ok(());
        }
    }
    Err(String::from("error"))
}

fn check_student_grade(n:&String, db:&Vec<Student>) -> Result<Option<u32>, String>{
    for s in db
    {
        if s.name == *n{
            return Ok(s.grade);
        }
    }
    Err(String::from("error"))
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
    let s2 = check_student(&n, &student_db);
    match s2{
        Ok(_) =>{
            let g2 = get_grade(&n, &student_db);
            if let Some(x) = g2{
                println!("result-if-let pattern grade is {}", x);        
            }
        }
        Err(x) => {
            println!("result-err pattern {}", x);        
        }
    }
    let g2 =get_grade(&n, &student_db);
    match g2{
        Some(x) => println!("mathc pattern grade is {}", x),
        _=>{},
    }

    if let Some(x) = g2{
        println!("if-let pattern grade is {}", x);
    }

}


pub fn run2(){
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
    let n = String::from("ef2gh");
    let s2 = check_student_grade(&n, &student_db);
    match s2{
        Ok(option_grade)=>{
            if let Some(grade) = option_grade
            {
                println!("result-if-let pattern grade is {grade}");
            }
        },
        Err(msg) => {
            println!("result-if-let pattern grade is {}", msg);
        }
    }
}

// enum Option<T>{
//     OK(T), 
//      Err(E)
// }