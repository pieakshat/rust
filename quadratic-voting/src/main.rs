use std::{collections::HashMap, hash::Hash, io};

use rand::Rng;

struct Voter {
    id: u64,
    name: String,
    credits: u64,
}

impl Voter {
    fn newVoter(name: String) -> Voter {
        Voter {
            id: rand::thread_rng().gen_range(1..=100), // random number generated for id
            name: name,
            credits: 100, // 100 credits assigned to each new user
        }
    }

    fn assignCredits(mut voter: Voter, amount: u64) {
        voter.credits += amount;
    }

    fn get_vote_cost(&self, num_votes: u64) -> u64 {
        num_votes * num_votes
    }
}

fn start_voting_system(options: Vec<String>, mut voters: Vec<Voter>) {
    let mut votes: HashMap<String, u32> = HashMap::new();

    for option in &options {
        votes.insert(option.clone(), 0);
    }

    for voter in &mut voters {
        println!("Voter: {} (Credits{})", voter.name, voter.credits);

        println!("which option would you like to vote for?");
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i + 1, option);
        }

        let mut option_input: String = String::new();
        io::stdin()
            .read_line(&mut option_input)
            .expect("failed to read the input");
        let option_index: usize = option_input
            .trim()
            .parse()
            .expect("please enter a valid number");

        println!("how many votes do you want to cast? (Cost will be votes squared)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read the input");
        let num_votes: u64 = input.trim().parse().expect("please enter a valid number");

        let cost = voter.get_vote_cost(num_votes);

        if cost > voter.credits {
            println!("voter does not have enough credits, cost would be {}", cost);
            continue;
        }

        voter.credits -= cost;
        if let Some(vote_count) = votes.get_mut(&options[option_index]) {
            *vote_count += num_votes as u32;
        }
    }

    println!("\nfinal vote results");
    for (option, count) in votes {
        println!("{}: {}", option, count);
    }
}

fn add_new_voters() -> Vec<Voter> {
    let mut input: String = String::new();
    let mut voters: Vec<Voter> = Vec::new();
    println!("enter the number of voters to add: ");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the input");
    let num_options: usize = input.trim().parse().expect("put a valid number");
    input.clear();

    for i in 1..=num_options {
        println!("enter the name of the new voter: ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("failed to read line");
        let mut new_voter: Voter = Voter::newVoter(name.trim().to_string());
        voters.push(new_voter);
        name.clear();
    }

    voters
}

fn collect_options() -> Vec<String> {
    let mut options: Vec<String> = Vec::new();
    let mut input: String = String::new();

    println!("enter the number of options: ");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the input");
    let num_options: usize = input.trim().parse().expect("please enter a valid number");
    input.clear();

    for i in 1..=num_options {
        println!("enter option {}", i);
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        options.push(input.trim().to_string());
        input.clear();
    }
    options
}

fn main() {
    let options: Vec<String> = collect_options();
    let voters: Vec<Voter> = add_new_voters();

    for voter in &voters {
        println!("{} {}: {}", voter.id, voter.name, voter.credits);
    }

    for option in &options {
        println!("{}", option);
    }

    start_voting_system(options, voters);
}
