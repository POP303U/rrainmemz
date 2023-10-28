use std::io::*;

#[derive(PartialEq, Clone, Copy, Debug)]
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
    pointer: usize,
}

impl Rrainmemz {
    pub fn new(code: String) -> Rrainmemz {
        Rrainmemz {
            code: Self::parse_code(code),
            pointer: 0,
            memory: vec![0; 30000],
        }
    }

    fn plus(&mut self) {
        match self.memory[self.pointer] {
            255 => self.memory[self.pointer] = 0,
            _ => self.memory[self.pointer] += 1,
        }
    }

    fn minus(&mut self) {
        match self.memory[self.pointer] {
            0 => self.memory[self.pointer] = 255,
            _ => self.memory[self.pointer] -= 1,
        }
    }

    fn input(&mut self) {
        let mut buffer = [0];
        stdin()
            .read_exact(&mut buffer)
            .expect("ERROR: Failed to read input");
        self.memory[self.pointer] = buffer[0];
    }

    fn output(&mut self) -> String {
        print!("{}", self.memory[self.pointer] as u8 as char);
        (self.memory[self.pointer] as char).to_string()
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
        let code: Vec<&str> = code.split(',').collect();
        let mut parsed_code = Vec::new();
        for i in 0..code.len() {
            match code[i].trim() {
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

    pub fn run(&mut self) -> String {
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
                    if self.memory[self.pointer] == 0 {
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
                    if self.memory[self.pointer] != 0 {
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
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapmemory_8bit() {
        let program = String::from("ligma, npc, sigma, sigma, npc");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), String::from("ÿ\u{1}"))
    }

    #[test]
    fn test_wraparound_cells() {
        let program = String::from("amogus, amogus, sideeye, sideeye, sideeye");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), String::from(""));
    }
    #[test]
    fn test_invalid_tokens() {
        let program = String::from("nothing lols");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), String::from(""));
    }

    #[test]
    fn test_loop() {
        let program = String::from("sigma, skedaadle, sigma, skedoodle");
        let mut rrainmemz = Rrainmemz::new(program);
        assert_eq!(rrainmemz.run(), String::from(""));
    }
}
