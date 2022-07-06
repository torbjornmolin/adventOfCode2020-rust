
mod machine;
mod machinev2;

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    second(&content);
}

fn first(input: &String) {
    let mut machine = machine::Machine::new();
    machine.parse_input(input);
    println!("Result: {}", machine.mem_acc());
}

fn second(input: &String) {
    let mut machine = machinev2::MachineV2::new();
    machine.parse_input(input);
    println!("Result: {}", machine.mem_acc());
}