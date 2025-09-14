use std::fmt;
#[derive(Debug)]

//enum
enum Gem {
    Diamond,
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
fn main() {
    //Tuple
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
}
