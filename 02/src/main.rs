const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

#[derive(PartialEq, Debug)]
struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u8,
    possible: bool,
    hands: Vec<Hand>,
    min_red: u8,
    min_green: u8,
    min_blue: u8,
    power: u32,
}

/// Get the game id from the line.
fn get_game_id(line: &str) -> u8 {
    // Simplify the split and get the first element:
    let v = line.split(':').next().expect("no game id");
    let id: u8 = v
        .trim_start_matches("Game ")
        .parse()
        .expect("failed to parse to u8.");
    id
}

/// Get Hand from the string.
fn get_hand(text: &str) -> Hand {
    let draw = text.split(", ").collect::<Vec<&str>>();
    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;
    for color in draw {
        let color_trim = color.trim();
        if color_trim.contains("red") {
            red = color_trim
                .trim_end_matches(" red")
                .parse()
                .unwrap_or_else(|_| panic!("failed to get red: {}", color))
        }
        if color_trim.contains("green") {
            green = color_trim
                .trim_end_matches(" green")
                .parse()
                .unwrap_or_else(|_| panic!("failed to get green: {}", color))
        }
        if color_trim.contains("blue") {
            blue = color_trim
                .trim_end_matches(" blue")
                .parse()
                .unwrap_or_else(|_| panic!("failed to get blue: {}", color))
        }
    }
    Hand {
        red: red,
        green: green,
        blue: blue,
    }
}

/// Get hands from the line.
fn get_hands(line: &str) -> Vec<Hand> {
    // Simplify the split and get the second element:
    let hands_str = line.split(":").collect::<Vec<&str>>()[1];
    let hands = hands_str.split("; ").collect::<Vec<&str>>();
    let mut hands_vec: Vec<Hand> = Vec::new();
    for hand in hands {
        hands_vec.push(get_hand(hand))
    }
    hands_vec
}

/// Determine if hand was possible
fn get_possible(hand: &Hand) -> bool {
    if hand.red > RED {
        return false;
    } else if hand.green > GREEN {
        return false;
    } else if hand.blue > BLUE {
        return false;
    }
    true
}

fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let mut sum: u32 = 0;
    let mut games: Vec<Game> = Vec::new();
    for line in file_contents.lines() {
        let id = get_game_id(line);
        let hands = get_hands(line);
        let mut possible: bool = true;
        for hand in &hands {
            if !get_possible(&hand) {
                possible = false;
                break;
            }
        }
        games.push(Game {
            id: id,
            possible: possible,
            hands: hands,
            min_red: u8::default(),
            min_green: u8::default(),
            min_blue: u8::default(),
            power: u32::default(),
        });
    }
    for game in games {
        if game.possible {
            sum = sum + game.id as u32;
        }
    }
    sum
}

fn get_min_red(hands: &Vec<Hand>) -> u8 {
    let mut min: u8 = 0;
    for hand in hands {
        if hand.red > min {
            min = hand.red;
        }
    }
    min
}

fn get_min_green(hands: &Vec<Hand>) -> u8 {
    let mut min: u8 = 0;
    for hand in hands {
        if hand.green > min {
            min = hand.green;
        }
    }
    min
}

fn get_min_blue(hands: &Vec<Hand>) -> u8 {
    let mut min: u8 = 0;
    for hand in hands {
        if hand.blue > min {
            min = hand.blue;
        }
    }
    min
}

fn get_power(red: u8, green: u8, blue: u8) -> u32 {
    red as u32 * green as u32 * blue as u32
}

fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let mut power: u32 = 0;
    let mut games: Vec<Game> = Vec::new();
    for line in file_contents.lines() {
        let id = get_game_id(line);
        let hands = get_hands(line);
        let min_red = get_min_red(&hands);
        let min_green = get_min_green(&hands);
        let min_blue = get_min_blue(&hands);
        let power = get_power(min_red, min_green, min_blue);
        games.push(Game {
            id: id,
            possible: true,
            hands: hands,
            min_red: min_red,
            min_green: min_green,
            min_blue: min_blue,
            power: power,
        });
    }
    for game in games {
        power = power + game.power;
    }
    power
}

fn main() {
    println!("part1_result: {}", part1("input.txt"));
    println!("part2_result: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_id() {
        assert_eq!(
            get_game_id("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            3
        );
    }

    #[test]
    fn test_get_hand() {
        assert_eq!(
            get_hand("8 green, 6 blue, 20 red"),
            Hand {
                red: 20,
                green: 8,
                blue: 6
            }
        );
    }

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 8);
    }

    #[test]
    fn part2_example01() {
        assert_eq!(part2("example.txt"), 2286);
    }
}
