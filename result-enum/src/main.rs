struct Student {
    name: String,
    grade: Option<u32>,
}

fn check_student_get_grade(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

fn main() {
    // Initialization of student database
    let student_db = vec![
        Student {
            name: String::from("Joe"),
            grade: Some(90),
        },
        Student {
            name: String::from("Doe"),
            grade: Some(75),
        },
    ];

    // Search for the grade of a specific student
    let student_name = String::from("Joe");
    let student_status = check_student_get_grade(&student_name, &student_db);

    // Match on the result and handle accordingly
    match student_status {
        Ok(option_grade) => {
            if let Some(grade) = option_grade {
                println!("Grade is: {}", grade);
            }
        }
        Err(error_msg) => println!("{}", error_msg),
    }
}
