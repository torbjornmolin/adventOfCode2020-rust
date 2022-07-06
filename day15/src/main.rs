use std::collections::HashMap;

fn main() {
    //first(vec![20,9,11,0,1,2]);
    second(vec![20,9,11,0,1,2]);
    //first(vec![0,3,6]);
}

fn first(starting_numbers: Vec<u32>) {
    let mut turn = 0;
    let mut spoken_numbers: HashMap<u32, u32> = HashMap::new();
    let mut last_spoken: u32 = 0;
    for n in starting_numbers {
        spoken_numbers.insert(n, turn);
        turn += 1;
        last_spoken = n;
    }
    loop {
        println!("Last spoken: {} ", last_spoken);
        if turn == 2020 {
            break;
        }
        let this_turn_spoken: u32;
        let last_turn_no = turn - 1;
        // last number has been spoken before
        if spoken_numbers.contains_key(&last_spoken) {
            let last_time = spoken_numbers.get(&last_spoken).unwrap();
            
            this_turn_spoken = last_turn_no - last_time;

            // update entry for number spoken last round.
            spoken_numbers.remove(&last_spoken);
            spoken_numbers.insert(last_spoken, last_turn_no);
        } 
        // last number has not been spoken before
        else {
            spoken_numbers.insert(last_spoken, last_turn_no);
            this_turn_spoken = 0;
        }

        turn += 1;
        last_spoken = this_turn_spoken;
    }
}
fn second(starting_numbers: Vec<u32>) {
    let mut turn = 0;
    let mut spoken_numbers: HashMap<u32, u32> = HashMap::new();
    let mut last_spoken: u32 = 0;
    for n in starting_numbers {
        spoken_numbers.insert(n, turn);
        turn += 1;
        last_spoken = n;
    }
    loop {
        if turn % 1000000 == 0 {
            print!(".");
        }
        if turn == 30000000 {
            println!("Last spoken: {} ", last_spoken);
            break;
        }
        let this_turn_spoken: u32;
        let last_turn_no = turn - 1;
        // last number has been spoken before
        if spoken_numbers.contains_key(&last_spoken) {
            let last_time = spoken_numbers.get(&last_spoken).unwrap();
            
            this_turn_spoken = last_turn_no - last_time;

            // update entry for number spoken last round.
            spoken_numbers.remove(&last_spoken);
            spoken_numbers.insert(last_spoken, last_turn_no);
        } 
        // last number has not been spoken before
        else {
            spoken_numbers.insert(last_spoken, last_turn_no);
            this_turn_spoken = 0;
        }

        turn += 1;
        last_spoken = this_turn_spoken;
    }
}