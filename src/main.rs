use std::{
    env,
    fs::{self},
    io,
};

#[derive(PartialEq, Clone, Copy)]
enum TokenType {
    Plus,
    Minus,
    LeftParen,
    RightParen,
    PointerRight,
    PointerLeft,
    Output,
    Input,
}

struct Rrainmemz {
    memory: Vec<u8>,
    code: Vec<TokenType>,
    pointer: i32,
}

impl Rrainmemz {
    fn new(code: Vec<TokenType>) -> Rrainmemz {
        Rrainmemz {
            memory: vec![0; 30000],
            code,
            pointer: 0,
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

    fn output(&mut self) {
        print!("{}", self.memory[self.pointer as usize] as u8 as char);
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

    fn parse_code(code: Vec<u8>) -> Vec<TokenType> {
        let mut parsed_code = Vec::new();
        for i in 0..code.len() {
            match code[i] {
                b'+' => parsed_code.push(TokenType::Plus),
                b'-' => parsed_code.push(TokenType::Minus),
                b'>' => parsed_code.push(TokenType::PointerRight),
                b'<' => parsed_code.push(TokenType::PointerLeft),
                b'.' => parsed_code.push(TokenType::Output),
                b',' => parsed_code.push(TokenType::Input),
                b'[' => parsed_code.push(TokenType::LeftParen),
                b']' => parsed_code.push(TokenType::RightParen),
                _ => eprintln!("invalid token: {}", code[i] as char),
            }
        }
        parsed_code
    }

    fn run(&mut self) {
        let mut i: usize = 0;
        while i < self.code.len() {
            let current_token: TokenType = self.code[i];
            match current_token {
                TokenType::Plus => self.plus(),
                TokenType::Minus => self.minus(),
                TokenType::PointerRight => self.move_right(),
                TokenType::PointerLeft => self.move_left(),
                TokenType::Output => self.output(),
                TokenType::Input => self.input(),
                TokenType::LeftParen => {
                    if self.memory[self.pointer as usize] == 0 {
                        let mut layers = 0;
                        loop {
                            if current_token == TokenType::RightParen {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i += 1;
                            if current_token == TokenType::LeftParen {
                                layers += 1
                            }
                        }
                    }
                }
                TokenType::RightParen => {
                    if self.memory[self.pointer as usize] != 0 {
                        let mut layers = 0;
                        loop {
                            if current_token == TokenType::LeftParen {
                                if layers == 0 {
                                    break;
                                }
                                layers -= 1
                            }
                            i -= 1;
                            if current_token == TokenType::RightParen {
                                layers += 1
                            }
                        }
                    }
                }
            }
            println!("{}", i);
            i += 1;
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)?;

    println!("File contents: \n{}", file_content);

    let parsed_code = Rrainmemz::parse_code(file_content.into());
    let mut rrainmemz = Rrainmemz::new(parsed_code);
    rrainmemz.run();

    Ok(())
}
