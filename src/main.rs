use rand::Rng;
use std::io;

// Password generator
//
// Generate a new password made of letters (upper and lower case), numbers,
// special characters, with at least one of each

fn main() {
    println!(
        "Here's your password sir: {}",
        generate_random_string(get_length())
    );
}

fn generate_random_string(length: u32) -> String {
    static NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    static LOWERCASE_LETTERS: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    static UPPERCASE_LETTERS: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    static SPECIAL_CHARACTERS: [char; 43] = [
        '!', '@', '#', '$', '%', '¨', '&', '*', '(', ')', '-', '_', '=', '+', '§', '{', '}', '[',
        ']', 'ª', 'º', '^', '/', ':', ';', '.', ',', '*', '\\', '|', '?', '°', '¹', '²', '³', '£',
        '¢', '¬', '´', '`', '>', '<', '~',
    ];

    let mut password = String::new();

    let charset = String::from(
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWX\
        YZ!@#$%¨&*()-_=+§{}[]ªº^/:;.*\\|?°¹²³£¢¬´`><~",
    );

    let char_vec: Vec<char> = charset.chars().collect();

    for i in 0..length {
        let random: usize = rand::thread_rng().gen_range(0..char_vec.len());
        password.push(char_vec[random]);
        if i == ((length - 1) - 4) {
            let random: usize = rand::thread_rng().gen_range(0..NUMBERS.len());
            password.push(NUMBERS[random]);
            let random: usize = rand::thread_rng().gen_range(0..LOWERCASE_LETTERS.len());
            password.push(LOWERCASE_LETTERS[random]);
            let random: usize = rand::thread_rng().gen_range(0..UPPERCASE_LETTERS.len());
            password.push(UPPERCASE_LETTERS[random]);
            let random: usize = rand::thread_rng().gen_range(0..SPECIAL_CHARACTERS.len());
            password.push(SPECIAL_CHARACTERS[random]);
            break;
        }
    }

    assert_eq!(password.contains(SPECIAL_CHARACTERS), true);
    assert_eq!(password.contains(UPPERCASE_LETTERS), true);
    assert_eq!(password.contains(LOWERCASE_LETTERS), true);
    assert_eq!(password.contains(NUMBERS), true);

    password
}

fn get_length() -> u32 {
    const MAX_LENGTH: u32 = 100;
    const MIN_LENGHT: u32 = 8;

    let length: u32 = loop {
        print!("Lenght of your password: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line");

        match input.trim().parse() {
            Ok(input) => {
                if input < MIN_LENGHT || input > MAX_LENGTH {
                    eprintln!("Choose a range between {} - {}", MIN_LENGHT, MAX_LENGTH);
                    continue;
                }
                break input;
            }
            Err(_) => {
                eprintln!("Invalid input, only integers. Try again");
                continue;
            }
        }
    };

    length
}
