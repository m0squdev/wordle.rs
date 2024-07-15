use std::{ collections::HashMap, fs::read_to_string, io::stdin };
use rand::Rng;

const PATH: &str = "src/words.txt";

fn main()
{
    let file_content = read_to_string(PATH).expect("Error while reading file content");
    let mut words = Vec::new();
    let mut word_count = 0;
    for line in file_content.lines()
    {
        words.push(line);
        word_count += 1;
    }
    let word = words[rand::thread_rng().gen_range(0..word_count)];
    let mut letters: HashMap<char, Vec<u8>> = HashMap::new();
    for (counter, letter) in word.chars().enumerate() {
        letters.entry(letter).or_insert(Vec::new()).push(
            match u8::try_from(counter).ok()
            {
                Some(position) => position,
                None =>
                {
                    println!("Error while examinating your guess");
                    return;
                }
            }
        );
    }
    println!("Welcome to Wordle! Try to guess the word I'm thinking about in 6 attempts.");
    let mut input;
    for attempt in 1..=6
    {
        loop
        {
            input = String::new();  // Needed in loop because next line appends to the existing String
            stdin().read_line(&mut input).expect("Error while reading user input");
            input = input.trim().to_lowercase();
            if input.chars().count() != 5
            {
                println!("\r\x1B[A\x1B[31m      Your guess must be 5 letters long!\x1B[37m\r\x1B[A");
            }
            else if !words.contains(&input.as_str())
            {
                println!("\r\x1B[A\x1B[31m      Word doesn't exist!\x1B[37m{}\r\x1B[A", " ".repeat(16));
            }
            else
            {
                break;
            }
        }
        let mut letter_count: HashMap<char, u8> = HashMap::new();
        let mut position_is_green: HashMap<u8, bool> = HashMap::new();
        let mut position_input: u8 = 0;
        for letter in input.chars()
        {
            // Green letter
            if letters.contains_key(&letter) && letters[&letter].contains(&position_input)
            {
                *letter_count.entry(letter).or_insert(0) += 1;
                position_is_green.insert(position_input, true);
                
            }
            // Grey letter
            else
            {
                letter_count.entry(letter).or_insert(0);
                position_is_green.insert(position_input, false);
            }
            position_input += 1;
        }
        let mut position_is_yellow: HashMap<u8, bool> = HashMap::new();
        position_input = 0;
        for letter in input.chars()
        {
            if letters.contains_key(&letter) && !letters[&letter].contains(&position_input)
            {
                // Yellow letter
                if letter_count[&letter] <
                    (match u8::try_from(letters[&letter].len()).ok()
                    {
                        Some(position) => position,
                        None =>
                        {
                            println!("Error while examinating your guess");
                            return;
                        }
                    }
                    )
                {
                    *letter_count.entry(letter).or_insert(0) += 1;
                    position_is_yellow.insert(position_input, true);
                }
                // Grey letter
                else
                {
                    position_is_yellow.insert(position_input, false);
                }
            }
            // Grey letter
            else
            {
                position_is_yellow.insert(position_input, false);
            }
            position_input += 1;
        }
        print!("\r\x1B[A");
        let mut keys: Vec<&u8> = position_is_green.keys().collect();
        keys.sort();
        for key in keys
        {
            let value = position_is_green[&key];
            let character;
            match input.chars().nth(usize::from(*key))
            {
                Some(_character) => character = _character,
                None =>
                {
                    println!("Error while comparing your guess and the word");
                    return;
                }
            }
            if value && position_is_yellow[&key]
            {
                println!("Error while comparing your guess and the word");
                return;
            }
            else if value && !position_is_yellow[&key]
            {
                print!("\x1b[32m{character}");
            }
            else if !value && position_is_yellow[&key]
            {
                print!("\x1b[33m{character}");
            }
            else
            {
                print!("\x1b[90m{character}");
            }
        }
        println!("\x1b[37m (Attempt {attempt}/6){}", " ".repeat(20));
        if input == word
        {
            print!("\x1b[37m🎉You win! ");
            break;
        }
    }
    println!("The answer was: {word}.")
}
