use core::str::SplitWhitespace;
use std::io;

fn get_winner(area: [[u8; 3]; 3]) -> Option<u8> {
    for y in 0..3 {
        if area[y][0] != 0 && area[y][0] == area[y][1] && area[y][0] == area[y][2] {
            println!("1");
            return Some(area[y][0])
        }
    }

    for x in 0..3 {
        if area[0][x] != 0 && area[0][x] == area[1][x] && area[0][x] == area[2][x] {
            println!("2");
            return Some(area[0][x])
        }
    }

    if area[0][0] != 0 && area[0][0] == area[1][1] && area[1][1] == area[2][2] {
        println!("3");
        return Some(area[0][0])
    }

    if area[0][2] != 0 && area[0][2] == area[1][1] && area[1][1] == area[2][0] {
        println!("4");
        return Some(area[0][2])
    }


    let mut empty_counter = 0;
    for y in 0..3 {
        for x in 0..3 {
            if area[y][x] == 0 {
                empty_counter += 1;
            }
        }
    }
    if empty_counter == 0 {
        return Some(3);
    }


    return None
}

fn draw(area: [[u8; 3]; 3]) {
    println!("| {} | {} | {} |", ws(area[0][0]), ws(area[0][1]), ws(area[0][2]));
    println!("| {} | {} | {} |", ws(area[1][0]), ws(area[1][1]), ws(area[1][2]));
    println!("| {} | {} | {} |", ws(area[2][0]), ws(area[2][1]), ws(area[2][2]));
}

fn ws(walker: u8) -> String {
    match walker {
        0 => "-".to_string(),
        1 => "X".to_string(),
        2 => "O".to_string(),
        _ => "?".to_string()
    }
}

fn get_move() -> (u8, u8) {
    println!("Enter two space-separated numbers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut numbers = input.trim().split_whitespace();

    let x: u8 = extract_next_number(&mut numbers, "x".to_string());

    let y: u8 = extract_next_number(&mut numbers, "y".to_string());

    return (x - 1, y - 1)
}

fn extract_next_number(numbers: &mut SplitWhitespace, name: String) -> u8 {
    match numbers.next() {
        Some(num) => match num.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for {}", name);
                0 // Or handle the error differently
            }
        },
        None => {
            println!("Missing input for {}", name);
            0
        }
    }
}

fn add(area: &mut [[u8; 3]; 3], walker: u8, x: u8, y: u8) {
    area[x as usize][y as usize] = walker;
}

fn main() {
    println!("Hello, world!");

    let mut walker: u8 = 2;
    let mut area = [[0u8; 3]; 3];
    draw(area);
    while {
        match get_winner(area) {
            Some(winner) => { 
                if winner == 3 {
                    println!("Draw")
                } else {
                    println!("Winner - {}", winner)
                }
                false
            },
            None => true,
        }
    } {
        walker = if walker == 1 { 2 } else { 1 };
        let (x, y) = get_move();
        add(&mut area, walker, x, y);
        draw(area);
    }
}
