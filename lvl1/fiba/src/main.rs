fn fiba(numbers: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128;

    for _ in 0..numbers {
        c = a + b;
        a = b;
        b = c;
    }

    return a;
}

fn fiba_recursion(numbers: u128) -> u128 {
    if numbers < 2 {
        return numbers
    }

    return fiba_recursion(numbers-1) + fiba_recursion(numbers-2)
}

fn main() {
    println!("Hello, world!");

    for i in 0..186 {
        println!("{} - {}", i, fiba(i));
    }
    println!("End fiba!");

    for i in 0..40 {
        println!("{} - {}", i, fiba_recursion(i));
    }
    println!("End fiba_recursion!");
}
