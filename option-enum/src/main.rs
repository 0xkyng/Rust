
struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}
fn main() {
    let student_db = vec![
        Student {
            name: String::from("Isaac"),
            grade: Some(97),
        },
        Student {
            name: String::from("Pelz"),
            grade:Some(90),
        },
        Student{
            name: String::from("Chalz"),
            grade: None,
        }
    ];

    let student_name = String::from("Isaac");
    let student_grade = get_grade(&student_name, &student_db);

    // match student_grade {
    //     Some(grade) => println!("Grade is:{grade}"),
    //     None => {}
    // }

    if let Some(grade) = student_grade {
        println!("Gradee is:{grade}");
    }
}
