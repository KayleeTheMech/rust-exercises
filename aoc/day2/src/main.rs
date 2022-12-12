use std::{fs, path::Path};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn value(self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
}

enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    fn value(self) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
}

fn parse_recommendation(input: &str) -> Option<(Choice, Choice, RoundResult)> {
    let recommendation: Vec<&str> = input.split(" ").collect();
    let theirs = match recommendation.first() {
        Some(&"A") => Some(Choice::Rock),
        Some(&"B") => Some(Choice::Paper),
        Some(&"C") => Some(Choice::Scissor),
        _ => None,
    };

    let our_wrong_idea = match recommendation.last() {
        Some(&"X") => Some(Choice::Rock),
        Some(&"Y") => Some(Choice::Paper),
        Some(&"Z") => Some(Choice::Scissor),
        _ => return None,
    };

    let actual_recommendation = match recommendation.last() {
        Some(&"X") => Some(RoundResult::Loss),
        Some(&"Y") => Some(RoundResult::Draw),
        Some(&"Z") => Some(RoundResult::Win),
        _ => return None,
    };
    Some((
        theirs.unwrap(),
        our_wrong_idea.unwrap(),
        actual_recommendation.unwrap(),
    ))
}

fn implied_choice(other_choice: Choice, recommended_outcome: RoundResult) -> Choice {
    match (other_choice, recommended_outcome) {
        (Choice::Rock, RoundResult::Loss) => Choice::Scissor,
        (Choice::Rock, RoundResult::Win) => Choice::Paper,
        (Choice::Paper, RoundResult::Loss) => Choice::Rock,
        (Choice::Paper, RoundResult::Win) => Choice::Scissor,
        (Choice::Scissor, RoundResult::Loss) => Choice::Paper,
        (Choice::Scissor, RoundResult::Win) => Choice::Rock,
        _ => other_choice, // all win and loss conditions handled, we should aim for draw
    }
}

struct Round {
    ours: Choice,
    theirs: Choice,
}

impl Round {
    fn result(self) -> RoundResult {
        match (&self.ours, &self.theirs) {
            (Choice::Paper, Choice::Scissor) => RoundResult::Loss,
            (Choice::Paper, Choice::Rock) => RoundResult::Win,
            (Choice::Rock, Choice::Paper) => RoundResult::Loss,
            (Choice::Rock, Choice::Scissor) => RoundResult::Win,
            (Choice::Scissor, Choice::Rock) => RoundResult::Loss,
            (Choice::Scissor, Choice::Paper) => RoundResult::Win,
            _ => RoundResult::Draw,
        }
    }
    fn points(self) -> u32 {
        self.ours.value() + self.result().value()
    }

    fn new(ours: Choice, theirs: Choice) -> Round {
        Round { ours, theirs }
    }
}

fn main() {
    let filepath = Path::new("./input.txt");
    let filestring = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let content: Vec<&str> = filestring.split("\n").collect();
    let mut our_wrong_idea_value: u32 = 0;
    let mut their_actual_recommendation_value: u32 = 0;

    for recommendation_string in content {
        println!("{}\n", recommendation_string);
        match parse_recommendation(recommendation_string) {
            Some((theirs, our_wrong_idea, actual_recommendation)) => {
                our_wrong_idea_value += Round::new(our_wrong_idea, theirs).points();
                let recommended_choice = implied_choice(theirs, actual_recommendation);
                their_actual_recommendation_value +=
                    Round::new(recommended_choice, theirs).points();
            }
            None => println!("Error parsing 1 round."),
        }
    }
    println!(
        "Wrong score according to misunderstanding: {:?}",
        our_wrong_idea_value
    );
    println!(
        "Score according to recommendation: {:?}",
        their_actual_recommendation_value
    );
}
