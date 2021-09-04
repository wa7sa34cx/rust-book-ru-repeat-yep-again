use rand::Rng;
use std::io;

// Range for our game. It can be anything,
// but we implement default values
#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Default for Range {
    fn default() -> Self {
        Range { min: 1, max: 100 }
    }
}

// Our game settings
#[derive(Debug)]
struct Game {
    range: Range,
    number: u32,
}

impl Game {
    fn new() -> Self {
        // Get default range
        let range = Range::default();

        // Get random guess number
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(range.min..=range.max);

        Game { range, number }
    }
}

struct Speech {}

impl Speech {
    fn greeting() {
        println!("Let's play the Guessing Game 🤔\n");
    }

    fn guess(game: &Game) {
        println!(
            "Print a number between {} and {}:",
            game.range.min, game.range.max
        );
    }

    fn not_a_number() {
        println!("This is not a number 😱 try again:");
    }

    fn less() {
        println!("Too small ⬆️ try again:");
    }

    fn greater() {
        println!("Too big ⬇️ try again:");
    }

    fn equal() {
        println!("Congrats 🎉 you guessed it!");
    }
}

struct Guess {
    number: u32,
}

pub fn run() {
    // Start new game
    let game = Game::new();

    // Say greeting
    Speech::greeting();

    // Say guessing
    Speech::guess(&game);

    // Get input from user
    let mut input = String::new();
    // read input value to variable
    // io::stdin().read_line(&mut input).expect("Failed to read input");

    // println!("You guessed: {:?}", input);
}
