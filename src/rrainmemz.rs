use std::io;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    Plus,
    Minus,
    LeftParen,
    RightParen,
    PointerRight,
    PointerLeft,
    Output,
    Input,
}

pub struct Rrainmemz {
    code: Vec<TokenType>,
    memory: Vec<u8>,
    pointer: i32,
}

impl Rrainmemz {
    pub fn new(code: String) -> Rrainmemz {
        let code: Vec<TokenType> = Self::parse_code(code);
        Rrainmemz {
            code,
            pointer: 0,
            memory: vec![0; 30000],
        }
    }

    fn plus(&mut self) {
        if self.memory[self.pointer as usize] == 255 {
            self.memory[self.pointer as usize] = 0;
        } else {
            self.memory[self.pointer as usize] += 1;
        }
    }

    fn minus(&mut self) {
        if self.memory[self.pointer as usize] == 0 {
            self.memory[self.pointer as usize] = 255;
        } else {
            self.memory[self.pointer as usize] -= 1;
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Failed to read input");
        self.memory[self.pointer as usize] = input.as_bytes()[0];
    }

    fn output(&mut self) -> String {
        print!("{}", self.memory[self.pointer as usize] as u8 as char);
        (self.memory[self.pointer as usize] as char).to_string()
    }

    fn move_right(&mut self) {
        if self.pointer == 29999 {
            self.pointer = 0;
        }
        self.pointer += 1;
    }

    fn move_left(&mut self) {
        if self.pointer == 0 {
            self.pointer = 29999;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn parse_code(code: String) -> Vec<TokenType> {
        let code: Vec<&str> = code.split_whitespace().collect();
        let mut parsed_code = Vec::new();
        for i in 0..code.len() {
            match code[i] {
                "sigma" => parsed_code.push(TokenType::Plus),
                "ligma" => parsed_code.push(TokenType::Minus),
                "sideeye" => parsed_code.push(TokenType::PointerRight),
                "amogus" => parsed_code.push(TokenType::PointerLeft),
                "npc" => parsed_code.push(TokenType::Output),
                "goofy" => parsed_code.push(TokenType::Input),
                "skedaadle" => parsed_code.push(TokenType::LeftParen),
                "skedoodle" => parsed_code.push(TokenType::RightParen),
                _ => {}
            }
        }
        parsed_code
    }

    pub fn run(&mut self) -> Result<String, String> {
        let mut i = 0;
        let mut output = String::new();
        while i < self.code.len() {
            match self.code[i] {
                TokenType::Plus => self.plus(),
                TokenType::Minus => self.minus(),
                TokenType::PointerRight => self.move_right(),
                TokenType::PointerLeft => self.move_left(),
                TokenType::Output => output += &self.output(),
                TokenType::Input => self.input(),
                TokenType::LeftParen => {
                    if self.memory[self.pointer as usize] == 0 {
                        let mut layers = 0;
                        loop {
                            if self.code[i] == TokenType::RightParen {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i += 1;
                            if self.code[i] == TokenType::LeftParen {
                                layers += 1
                            }
                        }
                    }
                }
                TokenType::RightParen => {
                    if self.memory[self.pointer as usize] != 0 {
                        let mut layers = 0;
                        loop {
                            if self.code[i] == TokenType::LeftParen {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i -= 1;
                            if self.code[i] == TokenType::RightParen {
                                layers += 1
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapmemory() {
        let program = String::from("<-.>---.+-.>-.");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), Ok(String::from("ÿýýÿ")))
    }

    #[test]
    fn test_wraparound() {
        let program = String::from("-.>>-.<<.");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), Ok(String::from("ÿÿÿ")));
    }
    #[test]
    fn test_invalid_character() {
        let program = String::from("nothing lol");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), Ok(String::new()));
    }

    #[test]
    fn test_loop() {
        let program = String::from("[[-.+]]++[-]-.");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), Ok(String::from("ÿ")));
    }
}
