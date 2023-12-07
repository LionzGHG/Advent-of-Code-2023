use self::day2::{Lexer, Token, Game};


pub mod day2 {
    // 12 red, 13 green, 14 blue
    // add id's of the possible games

    use std::fs;

    #[derive(Debug,Clone)]
    pub struct Game { pub id: String, pub colors: Vec<i32> }

    #[derive(Clone,Debug)]
    pub struct Token { pub kind: String, pub value: String }
    impl Token { 
        pub fn new(kind: &str, value: &str) -> Token {
            Token { kind: kind.to_string(), value: value.to_string() }
    }}

    pub struct Lexer { text: String, idx: usize, tokens: Vec<Token>, ch: char, token: Option<Token> }
    impl Lexer {
        pub fn new(text: String) -> Lexer {
            Lexer { text: text.clone(), idx: 0, tokens: Vec::new(), ch: text.chars().nth(0).unwrap(), token: None }
        }
        pub fn next(&mut self) {
            self.idx += 1;
            if self.idx < self.text.len() {
                self.ch = self.text.chars().nth(self.idx).unwrap();
        }}
        pub fn extract_id(&mut self) -> Token {
            while !self.ch.is_ascii_digit() {
                self.next();
            }
            let ch = self.ch.clone();
            Token::new("id", ch.to_string().as_str())
        }
        pub fn extract_color(&mut self) -> Token {
            println!("found ascii digit");
            let mut num: String = String::new();
            let mut color: String = String::new();
            num.push(self.ch);
            self.next();
            
            self.next();
            while self.ch != ',' || self.ch != ';' || self.ch != '\n' {
                if self.ch == ',' || self.ch == ';' || self.ch == '\n' { break };
                color.push(self.ch);
                self.next();
            }
            Token::new(color.as_str(),num.as_str())
        }
        pub fn tokenize(&mut self) -> Vec<Token> {
            while self.idx < self.text.len() {
                if self.ch == 'G' {
                    self.token = Some(self.extract_id());
                    self.next();
                }
                else if self.ch.is_ascii_digit() {
                    self.token = Some(self.extract_color());
                    self.next();
                }
                else {
                    self.next();
                    continue;
                }

                match self.token.clone() {
                    Some(n) => self.tokens.push(n),
                    None => panic!("Invalid Token"),
                }
            }
            return self.tokens.clone();
        }

        pub fn do_all(&mut self, input: String) {
            input.chars().
            let tokenizer: Lexer = Lexer::new(input);

        }

        pub fn evaluate(&mut self) -> Game {

            let mut game: Game = Game { id: String::new(), colors: Vec::new() };

            let mut greens: i32 = 0;
            let mut reds: i32 = 0;
            let mut blues: i32 = 0;

            for token in self.tokens.clone() {

                if token.kind == "id" {
                    game.id = token.value.clone();
                }

                else if token.kind == "green" {
                    greens += token.value.parse::<i32>().unwrap();
                }

                else if token.kind == "red" {
                    reds += token.value.parse::<i32>().unwrap();
                }

                else if token.kind == "blue" {
                    blues += token.value.parse::<i32>().unwrap();
                }
            }

            game.colors.push(reds);
            game.colors.push(greens);
            game.colors.push(blues);

            game
        }
    }
}

pub fn run() {
    let mut lexer: Lexer = Lexer::new("Game 1: 3 blue, 7 green, 10 red; 4 green, 4 red; 1 green, 7 blue, 5 red; 8 blue, 10 red; 7 blue, 19 red, 1 green\n".to_string());
    let tokens: Vec<Token> = lexer.tokenize();

    for i in tokens {
        println!("{:?}", i);
    }

    for i in lexer.evaluate().colors {
        println!("{i}");
    }

}