
pub mod day2 {
    // 12 red, 13 green, 14 blue
    // add id's of the possible games

    use std::fs;

    #[derive(Clone, Debug)]
    pub struct Token<'a> { pub kind: &'a str, pub value: &'a str }
    impl<'a> Token<'a> {
        pub fn new(kind: &'a str, value: &'a str) -> Token<'a> {
            Token { kind, value }
    }}

    pub struct Lexer<'a> {text: String, idx: usize, tokens: Vec<Token<'a>>, pub ch: char, token: Option<Token<'a>> }
    impl<'a> Lexer<'a> {
        pub fn new(text: String) -> Lexer<'a> {
            Lexer { text: text.clone(), idx: 0, tokens: Vec::new(), ch: text.chars().nth(0).unwrap(), token: None }
        }
        fn next(&mut self) {
            self.idx += 1;
            if self.idx < self.text.len() {
                self.ch = self.text.chars().nth(self.idx).unwrap();
        }}
        fn extract_name(&mut self) -> Token<'_> {
            self.next(); // skip "Game"
            let ch = self.ch.clone();
            Token::new("id", format!("{}",ch.clone()).as_str())
        }

}



    

}