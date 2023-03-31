use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::io::Write;

enum Coin {
    Heads,
    Tails,
}

impl Coin {
    fn flip() -> Self {
        let random_number = rand::thread_rng().gen_range(0..2);

        if random_number == 0 {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Coin::Heads => "Heads",
            Coin::Tails => "Tails",
        }
    }
}

fn main() {
    let mut stdout = std::io::stdout();

    println!("Welcome to the Coin Flip CLI utility!");

    enable_raw_mode().unwrap();

    loop {
        stdout
            .execute(Clear(ClearType::CurrentLine))
            .unwrap()
            .execute(cursor::MoveToColumn(0))
            .unwrap()
            .execute(Print("Press 'f' to flip the coin or 'q' to quit: "))
            .unwrap();

        let input = match event::read().unwrap() {
            Event::Key(key_event) => key_event.code,
            _ => continue,
        };

        match input {
            KeyCode::Char('f') | KeyCode::Char('F') => {
                let coin = Coin::flip();
                stdout
                    .execute(Clear(ClearType::CurrentLine))
                    .unwrap()
                    .execute(cursor::MoveToColumn(0))
                    .unwrap();
                println!("Result: {}", coin.to_string());
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                println!("\nGoodbye!");
                break;
            }
            _ => {
                print!(
                    "\r{}Invalid input. Please try again.{}",
                    SetForegroundColor(Color::Red),
                    ResetColor
                );
                stdout.flush().unwrap();
            }
        }
    }

    disable_raw_mode().unwrap();
}
