// practice of super trait
trait Person{
    fn name(&self) -> String;
}

trait Student: Person{
    fn university(&self) -> String;
}

trait Programmer{
    fn fav_language(&self) -> String;
}

trait CsStudent: Student + Programmer{
    fn git_username(&self) -> String;
}

fn cs_student_greeting(student: &dyn CsStudent) -> String{
    format!{
        "My name is {} and I attend {}. My favourite language is {}, and my git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    }
}

struct CeStudent{
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for CeStudent{
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CeStudent{
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CeStudent{
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CsStudent for CeStudent{
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main(){
    let student = CeStudent{
        name: "Asher".to_string(),
        university: "UW".to_string(),
        fav_language: "Python".to_string(),
        git_username: String::from("Cadian"),
    };

    println!("{}", cs_student_greeting(&student));
}

