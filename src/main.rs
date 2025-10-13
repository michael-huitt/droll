use std::env; //For command line args

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) { //Skip the first arg since we don't need it
        match arg_parse(arg) {
            Ok((die_quant, die_faces)) => println!("Die: {}\nFaces: {}", die_quant, die_faces),
            Err(error) => println!("Error: {}", error),
        } 
    }

}

fn arg_parse(string: &String) -> Result<(u32, u32), String> {
    let char_vec: Vec<char> = string.chars().collect();
    let mut dflag = false;
    let mut sflag = false; 

    for chr in char_vec.iter() {
        if !chr.is_ascii_digit() && (*chr != 'd' && *chr != '*') {
            return Err(format!("Invalid character: '{}'", chr)); 
        }
        
        match *chr {
            'd' => {
                if dflag == true {
                    return Err(format!("Specified 'd' too many times!"));
                }
            
                else {
                    dflag = true;
                }
            },
            
            '*' => {
                if sflag == true {
                    return Err(format!("Specified '*' too many times!"));
                }
            
                else {
                    sflag = true;
                }
            }, 
            
            _ => break,
        } 
    }

    let mut die_quant = 0; 
    let mut die_faces = 0;
    
    return Ok((die_quant, die_faces));
}
