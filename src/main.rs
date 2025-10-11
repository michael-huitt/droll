use std::env; //for command line args

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        arg_parse(arg); 
    }

}

fn arg_parse(string: &String) {
   let char_vec: Vec<char> = string.chars().collect();

   for chr in char_vec {
        println!("{}", chr); 
   }

}
