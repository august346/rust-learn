fn linear(a: f32, b: f32) -> Option<f32> {
    // ax + b = 0 -> ax = -b
    if b == 0.0 {
        return Some(0.0)
    }
    else if a == 0.0 {
        return None
    }
    return Some(-b / a)
}

fn quadratic(a: f32, b: f32, c: f32) -> Option<Vec<f32>> {
    let mut result = Vec::with_capacity(2);

    if a == 0.0 {
        match linear(b, c) {
            Some(val) => { result.push(val)},
            None => { }
        }
    } else {
        let discriminant = b*b - 4.0 * a * c;

        if discriminant < 0.0 { println!("discriminant negative! D = {}", discriminant) }

        if discriminant == 0.0 {
            result.push( -1.0 * (b / (2.0 * a)))
        }

        if discriminant > 0.0 {
            result.push( (-1.0 * b + discriminant.sqrt()) / (2.0 * a) );
            result.push( (-1.0 * b - discriminant.sqrt()) / (2.0 * a) );
        }
    }

    return Some(result);
}


fn main() {
    println!("Hello, world!\n\n\n");

    let a: f32 = 1.0;
    let b: f32 = 4.0;
    let c: f32 = -5.0;
    let mut msg;

    let sign_b_to_print = if b < 0.0 { "-" } else { "+" };
    let b_to_print = if b < 0.0 { -b } else { b };

    let sign_c_to_print = if c < 0.0 { "-" } else { "+" };
    let c_to_print = if c < 0.0 { -c } else { c };

    let equation = format!("{}x {} {} = 0", a, sign_b_to_print, b_to_print);
    match linear(a, b) {
        Some(val) => {msg = format!("{}; x = {}", equation, val)},
        None => {msg = format!("{}; x = no solution", equation)},
    }
    println!("{}\n", msg);

    let equation = format!("{}x^2 {} {}x {} {} = 0", a, sign_b_to_print, b_to_print, sign_c_to_print, c_to_print);
    match quadratic(a, b, c) {
        Some(val) =>  {msg = format!("{}; x = {:?}", equation, val)},
        None => {msg = format!("{}; x = no solution", equation)},
    }
    println!("{}", msg);
}
