
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Round {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}


impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let mut round = Self { red: 0, green: 0, blue: 0 };
        let colors = s.split(", ").collect::<Vec<_>>();
        for color in colors {
            let mut iter = color.split_whitespace();
            let count = iter.next().unwrap().parse().unwrap();
            let color = Color::from(iter.next().unwrap());
            match color {
                Color::Red => round.red = count,
                Color::Green => round.green = count,
                Color::Blue => round.blue = count,
            }
        }
        round
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut iter = s.split(": ");
        let id = iter
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .unwrap();
        let rounds = iter.next().unwrap().split("; ").map(Round::from).collect();

        Self { id, rounds }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let games = input.lines().map(Game::from).collect::<Vec<_>>();
    // let max_values = Round {
    //     red: 12,
    //     green: 13,
    //     blue: 14,
    // };

    let mut sum = 0;
    'game: for game in games {
        for round in game.rounds {
            if !round.is_valid()  {
                continue 'game;
            }
        }
        sum += game.id;
    }
    println!("p1: {}", sum);

}
