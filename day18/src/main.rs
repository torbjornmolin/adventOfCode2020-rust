use std::fs;

mod calculator;


fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Result of first: {}", first(&content));
}


fn first(content: &str) -> i64{
    let mut result = 0;
    for line in content.lines()
    {
        let mut calc = calculator::Calculator::from_str(line);
        result += calc.run();

    }
    result
}
