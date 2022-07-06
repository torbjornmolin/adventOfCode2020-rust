use std::collections::HashSet;
use std::fs;
use rule::Rule;
use ticket::Ticket;

mod rule;
mod ticket;

enum ParserState {
    Rules,
    MyTicket,
    NearbyTickets
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    first(&content);
    second(&content);
}

fn second(content: &String) {
    let mut parse_state = ParserState::Rules;
    let mut rules: Vec<Rule> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();
    let mut my_ticket_line: &str = "";
    for line in content.lines() {
        if line.starts_with("your ticket") {
            parse_state = ParserState::MyTicket;
            continue;
        }
        if line.starts_with("nearby tickets") {
            parse_state = ParserState::NearbyTickets;
            continue;
        }
        if line.is_empty() {
            continue;
        }

        match parse_state {
            ParserState::MyTicket => {
                my_ticket_line = line;
            }
            ParserState::NearbyTickets => {
                let ticket = Ticket::from_str(line);
                tickets.push(ticket);
            }
            ParserState::Rules => {
                let rule = Rule::from_string(line);
                rules.push(rule);
            }
        }
    }

    let mut valid_tickets: Vec<Ticket> = get_valid_tickets(tickets, &rules);
    valid_tickets.push(Ticket::from_str(my_ticket_line));

    let my_ticket = Ticket::from_str(my_ticket_line);
    let mut found_indicies: HashSet<usize> = HashSet::new();
    let mut result = 1;
    for _ in 1..1000
    {
        for rule in &rules {
            match matches_test(rule, &valid_tickets, &mut found_indicies) {
                Some(value) => {
                    if rule.name.starts_with("departure") {
                        result *= my_ticket.numbers[value];
                    }
                }
                None => {}
            }
        }
    }

    println!("Sum: {}", result);
}

fn matches_test<'a>(rule: &'a Rule, tickets: &[Ticket], found_indicies: &mut HashSet<usize>) -> Option<usize> {    
    let end_index = tickets.first().unwrap().numbers.len();
    let mut match_index = 0;

    let mut match_count = 0;
    for column_index in 0..end_index {
        if found_indicies.contains(&column_index) {
            continue;
        }
        let mut found = true;
        for ticket in tickets {
            if rule.matches(ticket.numbers[column_index]) {
                continue;
            }
            else {
                found = false;
                break;
            }
        }
        if found {
            match_count += 1;
            match_index = column_index;
        }
    }
    if match_count == 1 {
        found_indicies.insert(match_index);
        
        println!("Found index {} for rule {:?}", match_index, rule);
        Some(match_index)
    }
    else {
        None
    }
    
}

fn get_valid_tickets(tickets: Vec<Ticket>, rules: &[Rule]) -> Vec<Ticket> {
    let mut valid_tickets: Vec<Ticket> = Vec::new();
    for ticket in tickets {
        let mut valid = true;
        for n in &ticket.numbers {
            if !matches_any(rules, *n) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_tickets.push(ticket);
        }
    }
    valid_tickets
}

fn matches_any(rules: &[Rule], num: usize) -> bool {
    for rule in rules {
        if rule.matches(num) {
            return true;
        }
    }
    false
}

fn first(content: &str) {
    let mut parse_state = ParserState::Rules;
    let mut rules: Vec<Rule> = Vec::new();
    let mut tickets: Vec<Ticket> = Vec::new();

    for line in content.lines() {
        if line.starts_with("your ticket") {
            parse_state = ParserState::MyTicket;
            continue;
        }
        if line.starts_with("nearby tickets") {
            parse_state = ParserState::NearbyTickets;
            continue;
        }
        if line.is_empty() {
            continue;
        }

        match parse_state {
            ParserState::MyTicket => {
                continue;
            }
            ParserState::NearbyTickets => {
                let ticket = Ticket::from_str(line);
                tickets.push(ticket);
            }
            ParserState::Rules => {
                let rule = Rule::from_string(line);
                rules.push(rule);
            }
        }
    }
    let mut accumulator: u64 = 0;

    for ticket in tickets {
        for n in ticket.numbers {
            if !matches_any(&rules, n) {
                accumulator += n as u64;
            }
        }
    }
    println!("Accumulator: {}", accumulator);
}