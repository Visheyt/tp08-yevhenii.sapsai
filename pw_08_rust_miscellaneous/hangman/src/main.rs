use std::time::Duration;
use std::time::Instant;

const INPUT_MAX: u8 = 8;

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    word: String,
    found: bool,
    time: Duration,
}

fn main() {
    let mut p1 = get_player("player_1");
    let mut p2 = get_player("player_2");

    println!(
        "******************************************** {} plays",
        p1.name
    );
    let (found, time) = chrono_play(&p2.word);
    p1.found = found;
    p1.time = time;

    println!(
        "******************************************** {} plays",
        p2.name
    );
    let (found, time) = chrono_play(&p1.word);
    p2.found = found;
    p2.time = time;

    println!("******************************************** Result");
    println!(
        "{}: {}.{} seconds",
        p1.name,
        p1.time.as_secs(),
        p1.time.subsec_millis()
    );
    println!(
        "{}: {}.{} seconds",
        p2.name,
        p2.time.as_secs(),
        p2.time.subsec_millis()
    );
    println!("{}", get_winner(&p1, &p2));
}

fn get_player(file_path: &str) -> Player {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let main_vec: Vec<_> = contents.lines().collect();
    Player {
        name: main_vec[0].to_string(),
        word: main_vec[1].to_uppercase().to_string(),
        found: false,
        time: Duration::new(0, 0),
    }
}

fn get_dashed_word(word: &str, letters: &str) -> String {
    let mut s: String = String::new();
    for i in word.chars() {
        if letters.contains(i) {
            s.push(i);
        } else {
            s.push('-');
        }
    }

    s
}

fn get_letter() -> char {
    loop {
        println!("Enter a letter:");

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        let user_input = user_input.trim();

        if user_input.len() != 1 {
            continue;
        }

        if let Some(letter) = user_input.to_uppercase().pop() {
            break letter;
        }
    }
}
//Need pay attention on this segment
fn play(word: &str) -> bool {
    let mut letters: String = String::new();
    let mut input_count = INPUT_MAX;

    while input_count > 0 {
        let dashed_word = get_dashed_word(word, &letters);
        print!("{},{}\n", input_count, dashed_word);
        if dashed_word == word {
            return true;
        }
        let letter = get_letter();
        if !word.contains(letter) {
            input_count -= 1
        }
        if !letters.contains(letter) {
            letters.push(letter)
        }
    }
    return false;
}

fn chrono_play(word: &str) -> (bool, Duration) {
    let duration1 = Instant::now();
    let status = play(word);
    let duration2 = Instant::now();
    (status, duration2 - duration1)
}

fn get_winner(p1: &Player, p2: &Player) -> String {
    let mut winner_string: String = String::new();

    if p1.found == true && p2.found == false {
        winner_string = format!("The winner is {}", p1.name)
    } else if p2.found == true && p1.found == false {
        winner_string = format!("The winner is {}", p2.name)
    } else if p1.found == false && p2.found == false {
        winner_string = format!("No winner, two losers")
    } else if p1.found == true && p2.found == true {
        if p1.time == p2.time {
            winner_string = format!("No winner, no loser")
        } else if p1.time < p2.time {
            winner_string = format!("The winner is {}", p1.name)
        } else {
            winner_string = format!("The winner is {}", p2.name)
        }
    }

    winner_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_player() {
        let player = get_player("player_test");

        assert_eq!(
            player,
            Player {
                name: String::from("David"),
                word: String::from("RUST"),
                found: false,
                time: Duration::new(0, 0),
            }
        );
    }

    #[test]
    fn test_get_dashed_word() {
        assert_eq!(get_dashed_word("AZERTY", ""), "------");
        assert_eq!(get_dashed_word("TEST", "TAB"), "T--T");
        assert_eq!(get_dashed_word("LITERALIZATION", "AEIOU"), "-I-E-A-I-A-IO-");
        assert_eq!(get_dashed_word("LITERAL", "AEIOU"), "-I-E-A-");
    }

    #[test]
    fn test_get_winner() {
        let mut p1 = Player {
            name: String::from("David"),
            word: String::from("RUST"),
            found: false,
            time: Duration::new(0, 0),
        };

        let mut p2 = Player {
            name: String::from("Rolland"),
            word: String::from("LANGUAGE"),
            found: false,
            time: Duration::new(0, 0),
        };

        // Players 1 and 2 did not find the word.
        assert_eq!(get_winner(&p1, &p2), "No winner, two losers");

        // Player 1 did not find the word.
        // Player 2 found the word.
        p2.found = true;
        assert_eq!(get_winner(&p1, &p2), "The winner is Rolland");

        // Player 1 found the word.
        // Player 2 did not find the word.
        p1.found = true;
        p2.found = false;
        assert_eq!(get_winner(&p1, &p2), "The winner is David");

        // Players 1 and 2 found the word.

        // Same time.

        p2.found = true;
        assert_eq!(get_winner(&p1, &p2), "No winner, no loser");

        // Player 1 was faster.
        p2.time = Duration::new(5, 0);
        assert_eq!(get_winner(&p1, &p2), "The winner is David");

        // Player 2 was faster.
        p1.time = Duration::new(6, 0);
        assert_eq!(get_winner(&p1, &p2), "The winner is Rolland");
    }
}
