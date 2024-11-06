use std::string::String;
use std::str;

pub fn print_symbol(amount: u32, symbol: char){
    for i in 0..amount{
        if i == (amount-1){
            println!("{}", symbol);
        }else{
            print!("{}", symbol);
        }
    }  
}

pub fn print_with_spaces(print_length:u32, text_string:&str){
    let s:String  = text_string.trim().to_owned();
    let l:u32 = s.len().try_into().unwrap(); 

    if print_length <= l {
        println!("{}", text_string)
    }else{
        let n:f32 = ((print_length - l)/2) as f32;
        let start_n:u32 = n.floor() as u32;
        let end_n:u32 = n.ceil() as u32;
        let mut s2 = String::new();

        for _ in 0..start_n{
            s2.push(' ');    
        }

        s2.push_str(&s);

        for _ in 0..end_n{
            s2.push(' ');    
        }
        println!("{}", s2)
    }

}

pub fn print_new_lines(amount: u32){
    for _ in 0..amount{
        println!("");
    }
}