use std::io::stdin;

mod tests;

fn main() {
    let mut algorithm = Algorithm::default();

    loop {
        println!("Enter your expression: ");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to get user input");

        let result = algorithm.execute(input.as_str());

        if let Err(e) = result {
            println!("{:?}", e);
        }

        algorithm.print_queue();
        println!("Result: {}", algorithm.calculate_result());
        println!("----------------");
    }
}

#[derive(Debug)]
enum Output {
    Operator(char),
    Number(i32),
}

#[derive(Debug)]
enum AlgorithmError {
    UnableToParseInt,
    OperatorStackError,
}

#[derive(PartialEq)]
enum AssociatedWith {
    Left,
    Right,
}

struct Algorithm {
    operator_stack: Vec<char>,
    queue: Vec<Output>,
}

impl Algorithm {
    pub fn execute(&mut self, input: &str) -> Result<i32, AlgorithmError> {
        let characters: Vec<char> = input.chars().collect();

        let mut ch: usize = 0;
        while characters.len() > ch {
            // Skip over whitespaces
            if characters[ch].is_whitespace() {
                ch += 1;
                continue;
            }

            // Check if integer
            if characters[ch].is_numeric() {
                let from = ch;

                // Keep reading until we got the full digit
                while ch < characters.len() - 1 && characters[ch + 1].is_numeric() {
                    ch += 1;
                }

                // Convert to a digit
                let chars: Vec<&char> = characters[from..ch + 1].iter().collect();
                let digit = String::from_iter(chars)
                    .parse::<i32>()
                    .or(Err(AlgorithmError::UnableToParseInt))?;

                // Add digit to output
                self.queue.push(Output::Number(digit));
            }

            // Check for operators
            if "^*/+-".contains(characters[ch]) {
                let o1 = characters[ch];
                let mutable_o2 = self.operator_stack.last();

                // Handle precedence
                if let Some(mut o2) = mutable_o2 {
                    // Check if current operators precedence is lower or equals to that of the next operator (based on what's it's associated with)
                    while "^*/+-".contains(*o2)
                        && ((get_precedence(o1).1 == AssociatedWith::Left
                            && get_precedence(o1).0 <= get_precedence(*o2).0)
                            || get_precedence(o1).1 == AssociatedWith::Right
                                && get_precedence(o1).0 < get_precedence(*o2).0)
                    {
                        let op = self
                            .operator_stack
                            .clone()
                            .pop()
                            .ok_or(AlgorithmError::OperatorStackError)?;
                        self.queue.push(Output::Operator(op));
                        self.operator_stack.pop();

                        let v = self.operator_stack.last();

                        if let Some(v) = v {
                            o2 = v;
                        } else {
                            break;
                        }
                    }
                }

                self.operator_stack.push(o1);
            }

            if characters[ch] == '(' {
                self.operator_stack.push(characters[ch]);
            }

            if characters[ch] == ')' {
                while *self.operator_stack.last().unwrap_or(&'(') != '(' {
                    let op = self.operator_stack.pop();

                    if let Some(op) = op {
                        self.queue.push(Output::Operator(op))
                    }
                }

                self.operator_stack.pop();
            }

            ch += 1;
        }

        // Add the remaining operators to the output
        while !self.operator_stack.is_empty() {
            self.queue
                .push(Output::Operator(self.operator_stack.pop().unwrap()));
        }

        Ok(self.calculate_result())
    }

    pub fn print_queue(&self) {
        print!("RPN: ");
        for item in &self.queue {
            match item {
                Output::Operator(char) => print!("{} ", char),
                Output::Number(number) => print!("{} ", number),
            }
        }

        println!();
    }

    pub fn calculate_result(&self) -> i32 {
        let mut numbers: Vec<i32> = Vec::new();

        for item in &self.queue {
            match item {
                Output::Number(number) => {
                    numbers.insert(0, *number);
                }
                Output::Operator(op) => {
                    let right = numbers.remove(0);
                    let left = numbers.remove(0);

                    match op {
                        '^' => {
                            numbers.insert(0, left.pow(right as u32));
                        }
                        '/' => numbers.insert(0, left / right),
                        '*' => numbers.insert(0, left * right),
                        '+' => numbers.insert(0, left + right),
                        '-' => numbers.insert(0, left - right),
                        _ => panic!("Unknown operator!"),
                    };
                }
            }
        }

        *numbers.first().unwrap()
    }
}

// Helper functions

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm {
            operator_stack: vec![],
            queue: vec![],
        }
    }
}

fn get_precedence(operator: char) -> (i32, AssociatedWith) {
    match operator {
        '^' => (4, AssociatedWith::Right),
        '/' => (3, AssociatedWith::Left),
        '*' => (3, AssociatedWith::Left),
        '+' => (2, AssociatedWith::Left),
        '-' => (2, AssociatedWith::Left),
        _ => {
            panic!("Unknown operator: '{}'!", operator)
        }
    }
}
