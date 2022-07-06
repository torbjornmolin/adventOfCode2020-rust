#[derive(PartialEq, Debug)]
enum OperatorType {
    Plus,
    Multiply,
}
#[derive(PartialEq, Debug)]
enum Token {
    Operator(OperatorType),
    Number(i64),
    OpenParam,
    CloseParam,
    EOF
}

pub struct Calculator {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Calculator {
    pub fn from_str(input: &str) -> Calculator {
        Calculator {
            tokens: parse_to_tokens(input),
            cursor: 0
        }
    }

    pub fn run(&mut self) -> i64 {
           return self.expression();
    }

    fn primary(&mut self) -> i64 {
        println!("Primary");
        match self.get_token() {
            Token::Number(n) => {
                return *n;
            },
            Token::OpenParam => {
                let result = self.expression();
                let token = self.get_token();
                if *token != Token::CloseParam {
                    panic!("Syntax error, expected ), got {:?}.", *token);
                }
                return result;
            }
            _ => {
                panic!("Expected expression.");
            }
        }
    }

    fn expression(&mut self) -> i64 {
        println!("Expression");
        let mut left = self.factor() as i64;
        let mut t = self.get_token();
        loop {
            match t {
                Token::Operator(op) => {
                    match op {
                        OperatorType::Plus => {
                            self.put_back();
                            return left;
                        },
                        OperatorType::Multiply => {
                            left *= self.factor() as i64;
                        }                   
                    }
                    t = self.get_token();
                    continue;
                },
                _ => {
                    self.put_back();
                    return left;
                }
            }
        }
    }
    fn factor(&mut self) -> i64 {
        println!("Factor");
        let mut left = self.primary() as i64;
        let mut t = self.get_token();
        loop {
            match t {
                Token::Operator(op) => {
                    match op {
                        OperatorType::Plus => {
                            left += self.primary() as i64;
                        },
                        OperatorType::Multiply => {
                            self.put_back();
                            return left;
                        }                   
                    }
                    t = self.get_token();
                    continue;
                },
                _ => {
                    self.put_back();
                    return left;
                }
            }
        }
    }


    fn get_token(&mut self) -> &Token {
        if self.cursor >= self.tokens.len() {
            return &Token::EOF;
        }
        let token = &self.tokens[self.cursor];
        self.cursor += 1;
        println!("Reading token {:?}", token);
        token
    }

    fn put_back(&mut self) {
        if self.cursor > 1 {
            // println!("Putting back token {:?}", &self.tokens[self.cursor]);
            self.cursor -= 1;
        }
    }
}

    fn parse_to_tokens(input: &str) -> Vec<Token> {
        let mut result = Vec::<Token>::new();

        let bytes = input.chars().as_str().as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let char = bytes[i] as char;
            if char == '*' {
                result.push(Token::Operator(OperatorType::Multiply));
                
            }
            else if char == '+' {
                result.push(Token::Operator(OperatorType::Plus));
                
            }
            else if char == '(' {
                result.push(Token::OpenParam);
                
            }
            else if char == ')' {
                result.push(Token::CloseParam);
            }
            else if char.is_numeric() {
                let start_index = i;
                while i < bytes.len() && (bytes[i] as char).is_numeric()
                {
                    i += 1;
                }
                result.push(Token::Number(input[start_index..i].parse().unwrap()));
                continue;
            }
            else if char.is_whitespace() {
            }
            else {
                panic!("Unexpected token '{}' at {}", char, i);
            }
            i+=1;
        }

        result
    }

