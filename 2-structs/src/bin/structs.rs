struct Student {
    first_name: String,
    last_name: String
}

fn main() {
    let mut student = Student{first_name: "Brett".to_string(), last_name: "Shollenberger".to_string()};

    student.last_name = "Cassette".to_string();

    println!("student is {} {}", student.first_name, student.last_name);
}
