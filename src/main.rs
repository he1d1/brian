use std::cmp::Ordering;
use std::collections::{HashMap, LinkedList};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let brainfuck = match args.last() {
        Some(brainfuck) => brainfuck.chars().collect::<LinkedList<char>>(),
        None => {
            eprintln!("No brainfuck given.");
            std::process::exit(1);
        }
    };

    let mut current = brainfuck.front();
    let mut current_position = 0;

    let mut open_brackets: Vec<usize> = Vec::new();
    let mut closed_brackets: Vec<usize> = Vec::new();

    while current != None {
        match current.unwrap() {
            '[' => {
                open_brackets.push(current_position);
            }
            ']' => {
                closed_brackets.push(current_position);
            }
            _ => {}
        }
        current_position += 1;
        current = brainfuck.iter().nth(current_position);
    }

    if open_brackets.len() != closed_brackets.len() {
        eprintln!("Syntax error: Unmatched brackets, bracket opened at position {} has no closing bracket.", open_brackets[open_brackets.len() - 1]);
        std::process::exit(1);
    }

    let mut current = brainfuck.front();
    let mut current_position = 0;

    let mut memory = vec![0; 30000];
    let mut memory_position = 0;

    while current != None {
        match current.unwrap() {
            '>' => {
                memory_position += 1;
            }
            '<' => {
                memory_position -= 1;
            }
            '+' => {
                memory[memory_position] += 1;
            }
            '-' => {
                memory[memory_position] -= 1;
            }
            '.' => {
                print!("{}", memory[memory_position] as u8 as char);
            }
            ',' => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input.");
                memory[memory_position] =
                    input.trim().parse::<i32>().expect("Failed to parse input.");
            }
            '[' => {
                if memory[memory_position] == 0 {
                    current_position = closed_brackets[open_brackets
                        .iter()
                        .position(|&x| x == current_position)
                        .unwrap()];
                }
            }
            ']' => {
                if memory[memory_position] != 0 {
                    current_position = open_brackets[closed_brackets
                        .iter()
                        .position(|&x| x == current_position)
                        .unwrap()];
                }
            }
            _ => {
                eprintln!(
                    "Syntax error: Unknown character '{}' at position {}.",
                    current.unwrap(),
                    current_position
                );
                std::process::exit(1);
            }
        }
        current_position += 1;
        current = brainfuck.iter().nth(current_position);
    }
}
