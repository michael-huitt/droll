use std::env; //for command line args

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) { //skip the first arg since we don't need it
        arg_parse(arg); 
    }

}

fn arg_parse(string: &String) -> Result<(u32, u32), String> {
    let char_vec: Vec<char> = string.chars().collect();
    let mut dflag = false;
    let mut sflag = false; 
    let mut die_quant = 0; 
    let mut die_faces = 0;

    for chr in char_vec.iter() {
        if !chr.is_ascii_digit() && (*chr != 'd' && *chr != '*') {
            return Err(format!("Invalid character: '{}'", chr)); 
        }
        
        if *chr == 'd' {
            if dflag == true {
                return Err(format!("Specified 'd' too many times!"));
            }
        
            else {
                dflag = true;
            }
        }
    }
    return Ok((die_quant, die_faces));
}
