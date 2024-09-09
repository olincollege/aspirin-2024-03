#[allow(dead_code)]
enum Shape {
    Circle {
        radius: u16,
    },
    Square {
        side_length: u16,
    },
    Rectangle {
        width: u16,
        height: u16,
    },
    Trapezoid {
        base_one_length: u16,
        base_two_length: u16,
        height: u16,
    },
    Polygon {
        perimeter: u16,
        apothem_length: u16,
    },
}

enum Option {
    Some(u32),
    None,
}

enum Result{
    Ok(u32),
    Err()
}

fn main() {
    let optional_number = Option::Some(30);

    let explicit_number = match optional_number {
        Some(number) => number,
        None => 30
    }
}

#[allow(dead_code)]
fn get_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle { radius } => std::f32::consts::PI * radius as f32,
        Shape::Square { side_length } => side_length.into(),
        Shape::Rectangle { width, height } => (width * height).into(),
        Shape::Trapezoid {
            base_one_length,
            base_two_length,
            height,
        } => ((base_one_length / base_two_length) * height).into(),
        Shape::Polygon {
            perimeter,
            apothem_length,
        } => 0.5 * (perimeter * apothem_length) as f32,
    }
}

#[allow(dead_code)]
fn foo() -> i32 {
    let mut v = [1, 2, 3];
    for i in 0..v.len() {
        v[i] += 1;
    }
    v.into_iter().sum()
}
