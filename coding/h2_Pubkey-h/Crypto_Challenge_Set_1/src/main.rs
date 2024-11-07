use std::io;

mod utils;

//////////////////////////
/// VARIABLES
/////////////////////////

const PRINT_L: u32 = 50;
const NUM_OPTIONS: u32 = 4;

fn main() {
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_symbol(PRINT_L,'*');
    utils::print_with_spaces(PRINT_L, "Cryptopals: Challenge Set 1");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
    print_program_option(false);

    loop {
        let mut option_selected_s  = String::new();

        println!("Please input your choice ( Option number ) :");

        io::stdin()
            .read_line(&mut option_selected_s)
            .expect("Failed to read line");


        let mut option_selected_n: u32 = match option_selected_s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option_selected_n <= NUM_OPTIONS {

            println!("You have selected option : {option_selected_n}");

            match option_selected_n {
                0 => exit_program(),
                1 => challenge_1(),
                2 => challenge_2(),
                3 => challenge_3(),
                4 => challenge_4(),
                _ => continue,
            }
        }else {
            utils::print_new_lines(2);
            println!("You have select : {option_selected_n}, which is not an option in this program. Please select again!");
        }
        print_program_option(false);
    }
}

fn print_program_option(clear_screen_b:bool){
    if clear_screen_b{
        utils::clear_c();    
    }

    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Which challenge would you like to run?");
    utils::print_new_lines(1);
    println!("Option 1: Challenge 1 = Convert hex to base64.");
    println!("Option 2: Challenge 2 = Fixed XOR.");
    println!("Option 3: Challenge 3 = Single-byte XOR cipher.");
    println!("Option 4: Challenge 4 = Detect single-character XOR.");
    println!("Option 0: Exit program.");
    utils::print_new_lines(2);
}

fn challenge_1(){
    let mut challenge_hex_s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 1 : Convert hex to base64.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
    println!("Convert a hexadecimal string to base64.");
    println!("Would you like use the default hexadecimal value or input you're own? ");
    println!("Option 1: use default hexadecimal value : {}", challenge_hex_s);
    println!("Option 2: input you're own hexadecimal value");
    println!("Option 2: input you're own hexadecimal value");
    println!("Please input your choice: 1 for option 1, 2 for option 2, 0 to exit the program.");
}

fn challenge_2(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 2 = Fixed XOR.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);    
}

fn challenge_3(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 3 = Single-byte XOR cipher.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
}

fn challenge_4(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_with_spaces(PRINT_L, "Challenge 4 = Detect single-character XOR.");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);    
}

fn exit_program(){
    utils::clear_c();
    utils::print_new_lines(2);
    utils::print_symbol(PRINT_L,'*');
    utils::print_with_spaces(PRINT_L, "Thank you for using this program!");
    utils::print_symbol(PRINT_L,'*');
    utils::print_new_lines(2);
    std::process::exit(0);
}