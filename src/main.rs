use std::env; //For command line args

struct Roll {
    quant: u32,
    faces: u32,
    mult: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    for arg in args.iter().skip(1) { //Skip the first arg since we don't need it
        match arg_parse(arg) {
            Ok(roll) => {
                println!("It works!");    
            }, 
            
            Err(error) => println!("{}", error),
        } 
    }

}

fn char_vec_to_u32(char_vec: Vec<&char>) -> Result<u32, String> {
    let u32_str: String = char_vec.into_iter().collect();

    match u32_str.parse::<u32>() {
        Ok(int) => return Ok(int),
        Err(error) => return Err(format!("Conversion Error({})", error))
    }
}

fn arg_parse(string: &String) -> Result<Roll, String> {
    let char_vec: Vec<char> = string.chars().collect();
    let mut vec_quant = Vec::new(); let mut vec_faces = Vec::new(); let mut vec_mult = Vec::new();
    let mut dflag = false; let mut sflag = false; 

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
            
            _ => {

                if !chr.is_ascii_digit() && (*chr != 'd' && *chr != '*') {
                    return Err(format!("Invalid character: '{}'", chr)); 
                }

                else if chr.is_ascii_digit() {
                    if dflag == false {
                        vec_quant.push(chr);
                    }
                    
                    else if dflag == true && sflag == false {
                        vec_faces.push(chr);
                    }
                
                    else {
                        vec_mult.push(chr);
                    }
                }
            },
        } 
    }
   
    let mut roll = Roll {
        quant: char_vec_to_u32(vec_quant)?,
        faces: char_vec_to_u32(vec_faces)?,
        mult: 0,
    };

    if !vec_mult.is_empty() {
        roll.mult = char_vec_to_u32(vec_mult)?; 
    }

    return Ok(roll);
}
