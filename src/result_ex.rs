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


fn check_student(n:&String, db:&Vec<Student>) -> Result{

    Ok(())
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

}

// enum Option<T>{
//     None, 
//     Some(T)
// }