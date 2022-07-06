use std::collections::HashMap;

pub struct Machine {
    memory: HashMap<u64, u64>,
    bitmask: String
}

impl Machine {
    pub fn new() -> Machine {
        let vec_size: usize = (2 as usize).pow(36);
        let memory = HashMap::new();
        
        Machine {
            memory: memory,
            bitmask: String::from("")
        }
    }

    fn set_bit_n(&self, input: u64, index: usize) -> u64 {
        let bitmask: u64 = 1 << index;

        return input | bitmask;
    }
    fn unset_bit_n(&self, input: u64, index: usize) -> u64 {
        let bitmask: u64 = 1 << index;

        return input & !bitmask;
    }

    pub fn save(&mut self, value: u64, address: u64) {
        let modified_value = self.apply_bitmask(value);
        
        if self.memory.contains_key(&address) {
            self.memory.remove(&address);
        }
        
        self.memory.insert(address, modified_value);
    }

    fn apply_bitmask(&self, mut input: u64) -> u64 {
        let rev_bitmask: Vec<char> = self.bitmask.chars().rev().collect();
        for (index, c) in rev_bitmask.into_iter().enumerate() {
            match c {
                'X' => {
                    continue;
                }
                '1' => {
                    input = self.set_bit_n(input, index);
                }
                '0' => {
                    input = self.unset_bit_n(input, index);
                }
                _ => {
                    panic!("Bitmask error: {}", c);
                }
            }
        }

        input
    }
    pub fn parse_input(&mut self, input: &String) {
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split(" ").collect();

            match parts[0] {
                "mask" => {
                    println!("Set mask to: {}", parts[2]);

                    self.bitmask = String::from(parts[2].trim());
                }
                _ => {
                    let mem_parts: Vec<&str> = parts[0].split("[").collect();
                    if mem_parts.len() > 1 {
                        let address = mem_parts[1].trim().trim_end_matches("]");
                        println!("memset op: {} to {}", address, parts[2]);
                        let value: u64 = parts[2].parse().unwrap();
                        let address: u64 = address.parse().unwrap();
                        self.save(value, address);
                    }
                    else {
                        panic!("Unexpected input: {:?}", parts);
                    }
                }
            }
        }
    }

    pub fn mem_acc(&self) -> u128 {
        println!("mem_acc");
        let mut result: u128 = 0;
        for (_, value) in &self.memory {
            let value: u128 = (*value).into();
            result += value;
        }
        result
    }

}