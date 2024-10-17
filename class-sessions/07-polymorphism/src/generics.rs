fn main() {
    dbg!(smallest_i32(100, 150));
    dbg!(smallest_f64(100.0, 150.0));
    dbg!(smallest_char('a', 'b'));
    dbg!(smallest(100, 150));

    let amit = Student {
        name: "Amit".to_string(),
        id: 1,
        gpa: 4.0,
    };

    let ayush = Student {
        name: "Ayush".to_string(),
        id: 2,
        gpa: 4.0,
    };

    dbg!(smallest(amit, ayush));
}

#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    id: u32,
    gpa: f32,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Student) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

fn smallest<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

fn smallest_i32(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn smallest_f64(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn smallest_char(a: char, b: char) -> char {
    if a < b {
        a
    } else {
        b
    }
}
