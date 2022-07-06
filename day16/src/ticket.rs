#[derive(Debug)]
pub struct Ticket {
    pub numbers: Vec<usize>
}

impl Ticket {
    pub fn from_str(s: &str) -> Ticket{
        let mut numbers: Vec<usize> = Vec::new();
        for n in s.split(',') {
            let num: usize = n.parse().unwrap();
            numbers.push(num);
        }
        Ticket { numbers }
    }
}