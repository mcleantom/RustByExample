trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

trait Barks {
    fn bark(&self) -> String;
}

trait BarkingStudent: CompSciStudent + Barks {}

fn comp_sci_student_greeting(
    student: &dyn BarkingStudent
) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My git user name is {}, {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
        student.bark()
    )
}

fn main() {}