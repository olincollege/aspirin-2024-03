pub trait Speak {
    fn speak(&self);
}

struct Rohan {}

struct Cherry {}

impl Speak for Rohan {
    fn speak(&self) {
        println!("I don't know");
    }
}

impl Speak for Cherry {
    fn speak(&self) {
        println!("Uhhhhhhh");
    }
}



struct Students {
    first_student: Box<dyn Speak>,
    second_student: Box<dyn Speak>,
}

fn make_students_speak(is_cherry_first: bool) {
    let first_student: Box<dyn Speak>;
    let second_student: Box<dyn Speak>;

    match is_cherry_first {
        true => {
            first_student = Box::new(Cherry {});
            second_student = Box::new(Rohan {})
        }
        false => {
            first_student = Box::new(Rohan {});
            second_student = Box::new(Cherry {})
        }
    };

    let students = Students {
        first_student,
        second_student,
    };

    students.first_student.speak();
    students.second_student.speak();
}

// struct Students<T, U>
// where
//     T: Speak,
//     U: Speak,
// {
//     first_student: T,
//     second_student: U,
// }

// fn make_students_speak() {
//     let students = Students {
//         first_student: Cherry {},
//         second_student: Rohan {},
//     };

//     students.first_student.speak();
//     students.second_student.speak();
// }
