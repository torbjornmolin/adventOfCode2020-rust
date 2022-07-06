use std::collections::HashMap;

pub struct MachineV2 {
    memory: HashMap<u64, u64>,
    bitmask: String
}

impl MachineV2 {
    pub fn new() -> MachineV2 {
        let vec_size: usize = (2 as usize).pow(36);
        let memory = HashMap::new();
        
        MachineV2 {
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
        
        for location in self.get_addresses(address)
        {
            println!("Writing {} to {}", value, location);
            if self.memory.contains_key(&location) {
                self.memory.remove(&location);
            }
            
            self.memory.insert(location, value);
        }
    }

    fn get_addresses(&self, address: u64) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for bitmask in self.get_bitmask_variants() {
            println!("Appplying {} to {}", bitmask, address);
            let address = self.apply_bitmask(address, bitmask);
            println!("Resulting address: {}", address);
            result.push(address);
        }
        return result;
    }
    fn get_bitmask_variants(&self) -> Vec<String> {
        println!("Getting variants for {}", self.bitmask);
        let mut result: Vec<String> = Vec::new();
        let bitmask_x_count = self.bitmask.matches("X").count();
        // loop over bitmask variants
        for i in 0..(2 as u64).pow(bitmask_x_count as u32) {
            let mut current_bitmask = self.bitmask.clone();
            let bitmask_chars: Vec<char> = self.bitmask.chars().collect();
            let mut x_index = 0;

            for (index, c) in bitmask_chars.into_iter().enumerate() {
                match c {
                    'X' => {
                        if (i >> x_index & 1) == 1 { // if i shifted x_index to right is 1
                            current_bitmask.replace_range(index..index+1, "1");
                        } else {
                            current_bitmask.replace_range(index..index+1, "F");
                        }
                        x_index += 1;
                    }
                    '1' => {
                        continue;
                    }
                    '0' => {
                         continue;
                    }
                    _ => {
                        panic!("Bitmask error: {}", c);
                    }
                }
            }
            if (current_bitmask.contains("X")) {
                panic!("X still present: {}", current_bitmask);
            }

            // println!("Adding variant: {}", current_bitmask);
            result.push(current_bitmask);
        }

        return result;
    }

    fn apply_bitmask(&self, input: u64, bitmask: String) -> u64 {
        let mut result = input;
        let rev_bitmask: Vec<char> = bitmask.chars().rev().collect();
        for (index, c) in rev_bitmask.into_iter().enumerate() {
            match c {
                '1' => {
                    result = self.set_bit_n(result, index);
                }
                '0' => {
                     // unchanged
                }
                'F' => {
                    // force zero
                    result = self.unset_bit_n(result, index);   
                }
                _ => {
                    panic!("Bitmask error: {}", c);
                }
            }
        }
        result
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