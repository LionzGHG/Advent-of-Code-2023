
pub mod day1 {
    // first digit combined with last digit to form single two-diget-num

    use std::fs;

    pub fn solve_part_one() {
        let input: String = fs::read_to_string("day1input.txt").unwrap();
        let mut calibs: Vec<String> = Vec::new();

        let mut it = input.lines().peekable();
        while let Some(n) = it.next() {
            let nums: Vec<char> = n.chars().filter(|x| x.is_ascii_digit()).collect();
            calibs.push(format!("{}{}", nums.first().unwrap(), nums.last().unwrap())); 
        }

        let res: i32 = calibs.iter().map(|x| x.parse::<i32>().unwrap()).sum();
        println!("{res}");
    }

    pub fn solve_part_two() {
        let input: String = fs::read_to_string("day1input.txt").unwrap();
        let mut calibs: Vec<String> = Vec::new();
        
        let word_input: Vec<String> = input.split('\n').map(|l| {
            l.to_string()
            .replace("zero", "zero0zero")
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
        }).collect();

        let mut new_input: String = String::new();
        for word in word_input {
            new_input.push_str(&word);
            new_input.push_str("\n");
        }

        let mut it = new_input.lines().peekable();
        while let Some(n) = it.next() {
            let nums: Vec<char> = n.chars().filter(|x| x.is_ascii_digit()).collect();
            calibs.push(format!("{}{}", nums.first().unwrap(), nums.last().unwrap())); 
        }

        let res: i32 = calibs.iter().map(|x| x.parse::<i32>().unwrap()).sum();
        println!("{res}");
    }
}