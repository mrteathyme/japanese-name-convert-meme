use std::env;

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();
    let mut result: Vec<&'static str> = vec![];
    for token in args {
        for char in token.chars() {
            if char == ' ' {
                result.push(" ");
                continue
            }
            result.push(convert(char).unwrap())

        }
    }
    println!("{:?}", result.concat());
}


fn convert(character: char) -> Option<&'static str> {
    match character {
        'a' | 'A' => Some("KA"),
        'b' | 'B' => Some("TU"),
        'c' | 'C' => Some("MI"),
        'd' | 'D' => Some("TE"),
        'e' | 'E' => Some("KU"),
        'f' | 'F' => Some("RU"),
        'g' | 'G' => Some("JI"),
        'h' | 'H' => Some("RE"),
        'i' | 'I' => Some("KI"),
        'j' | 'J' => Some("ZU"),
        'k' | 'K' => Some("ME"),
        'l' | 'L' => Some("TA"),
        'm' | 'M' => Some("RIN"),
        'n' | 'N' => Some("TO"),
        'o' | 'O' => Some("MO"),
        'p' | 'P' => Some("NO"),
        'q' | 'Q' => Some("KE"),
        'r' | 'R' => Some("SHI"),
        's' | 'S' => Some("SU"),
        't' | 'T' => Some("CHI"),
        'u' | 'U' => Some("DO"),
        'v' | 'V' => Some("RU"),
        'w' | 'W' => Some("MEI"),
        'x' | 'X' => Some("NA"),
        'y' | 'Y' => Some("FU"),
        'z' | 'Z' => Some("ZE"),

        _ => Some("")
    }
}