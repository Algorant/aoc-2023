
#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}


impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let mut round = Self { red: 0, green: 0, blue: 0 };
        let colors = s.split(", ").collect::<Vec<_>>();
        for color in colors {
            let mut iter = color.split_whitespace();
            let count = iter.next().unwrap().parse().unwrap();
            let color = iter.next().unwrap();
            match color {
                "red" => round.red = count,
                "green" => round.green = count,
                "blue" => round.blue = count,
                _ => unreachable!(),
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

impl Game {
    fn power(&self) -> usize {
        let red = self.rounds.iter().map(|r| r.red).max();
        let green = self.rounds.iter().map(|r| r.green).max();
        let blue = self.rounds.iter().map(|r| r.blue).max();
        red.unwrap() * green.unwrap() * blue.unwrap()
    }
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
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();
    let games = input.lines().map(Game::from).collect::<Vec<_>>();

    let sum = games
        .iter()
        .filter(|g| g.rounds.iter().all(|r| r.is_valid()))
        .map(|g| g.id)
        .sum::<usize>();
    println!("p1: {}", sum);

    // Part 2
    let sum_power = games.iter().map(|g| g.power()).sum::<usize>();
    println!("p2: {}", sum_power);

}
