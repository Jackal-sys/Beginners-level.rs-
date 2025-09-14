use std::{collections::HashMap, fmt, io};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive, ToPrimitive, Eq, PartialEq, Hash, Clone, Copy)]
enum Gem {
    Diamond = 1,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "Diamond"),
            Gem::Sapphire => write!(f, "Sapphire"),
            Gem::Ruby => write!(f, "Ruby"),
            Gem::Topaz => write!(f, "Topaz"),
            Gem::Onyx => write!(f, "Onyx"),
        }
    }
}

fn game(arr: &mut [[u8; 5]; 5]) -> Vec<Gem> {
    let mut found: Vec<Gem> = Vec::new();

    while found.len() < 5 {
        println!("Search an X,Y position (Example - 3 1):");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line");
            continue;
        }

        let part: Vec<&str> = input.trim().split_whitespace().collect();
        if part.len() != 2 {
            println!("Two numbers required");
            continue;
        }

        let (x, y) = match (part[0].parse::<usize>(), part[1].parse::<usize>()) {
            (Ok(x), Ok(y)) if x < 5 && y < 5 => (x, y),
            _ => {
                println!("Invalid input: must be numbers between 0 and 4");
                continue;
            }
        };

        let data = arr[x][y];
        let gem = match Gem::from_u8(data) {
            Some(gem) => gem,
            None => {
                println!("No gem found in this position");
                continue;
            }
        };

        found.push(gem);
        arr[x][y] = 0; // remove gem after finding

        println!("You found: {:?}", found);
    }

    found
}

fn main() {
    // Show gem values
    let gems = [
        (Gem::Diamond, 25.00),
        (Gem::Sapphire, 90.00),
        (Gem::Ruby, 40.00),
        (Gem::Topaz, 60.00),
        (Gem::Onyx, 30.00),
    ];

    for gem in gems {
        println!("This {} is worth: {}", gem.0, gem.1);
    }

    // Create 5x5 grid with gem placements
    let mut arr = [[0; 5]; 5];
    arr[3][1] = 1;
    arr[1][4] = 2;
    arr[4][1] = 3;
    arr[1][2] = 4;
    arr[1][0] = 5;

    for row in &arr {
        println!("{row:?}");
    }

    // Run game
    let found: Vec<Gem> = game(&mut arr);

    // Value map
    let mut gem_values: HashMap<Gem, f64> = HashMap::new();
    gem_values.insert(Gem::Diamond, 1000.00);
    gem_values.insert(Gem::Sapphire, 2000.00);
    gem_values.insert(Gem::Ruby, 3000.00);
    gem_values.insert(Gem::Topaz, 4000.10);
    gem_values.insert(Gem::Onyx, 5000.50);

    // Sum the total value
    let mut sum = 0.0;
    for gem in &found {
        sum += gem_values[gem];
    }

    println!("The total Gem value is: {}", sum);
}
