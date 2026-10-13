use std::io;

struct Parser<'a> {
    tokens: Vec<&'a str>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<&'a str> {
        let token = self.tokens.get(self.pos).copied();
        self.pos += usize::from(token.is_some());

        token
    }

    fn parse_factor(&mut self) -> f64 {
        self.next().unwrap().parse::<f64>().unwrap()
    }

    fn parse_term(&mut self) -> f64 {
        let mut val = self.parse_factor();

        while matches!(self.peek(), Some("*" | "/")) {
            let op = self.next().unwrap();
            let rhs = self.parse_factor();

            match op {
                "*" => val *= rhs,
                "/" => val /= rhs,
                _ => unreachable!(),
            }
        }

        val
    }

    fn parse_expression(&mut self) -> f64 {
        let mut val = self.parse_term();

        while matches!(self.peek(), Some("+" | "-")) {
            let op = self.next().unwrap();
            let rhs = self.parse_term();

            match op {
                "+" => val += rhs,
                "-" => val -= rhs,
                _ => unreachable!(),
            }
        }

        val
    }
}

fn main() {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Input Error");

        input.retain(|c| !c.is_whitespace());

        // parsing
        let mut tokens = vec![];
        let mut last_index = 0;

        for (i, op) in input.match_indices(&['+', '-', '*', '/'][..]) {
            tokens.push(&input[last_index..i]);
            tokens.push(op);

            last_index = i + op.len();
        }
        tokens.push(&input[last_index..]);

        // calculate
        let mut parser = Parser { tokens, pos: 0 };
        let result = parser.parse_expression();

        println!("result: {}", result);
    }
}

fn assert_f64(actual: f64, expected: f64) {
    const EPSILON: f64 = 1e-10;

    assert!(
        actual == expected || (actual - expected).abs() < EPSILON,
        "expected: {expected}, actual: {actual}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let mut parser = Parser {
            tokens: vec!["2", "+", "3"],
            pos: 0,
        };

        assert_f64(parser.parse_expression(), 5.0);
        assert_eq!(parser.pos, parser.tokens.len());
    }

    #[test]
    fn mixed() {
        let mut parser = Parser {
            tokens: vec!["1", "-", "2", "+", "6", "/", "2", "*", "5"],
            pos: 0,
        };

        assert_f64(parser.parse_expression(), 14.0);
        assert_eq!(parser.pos, parser.tokens.len());
    }
}
