fn main() {
    let encoded_message = caesar_cypher("middle-Outz",2);
    println!("Encoded Message : {:?}", encoded_message);
}

/// CAESAR'S CYPHER
/// INPUT middle-Outz   k=2
/// OUTPUT okffng-Qwvb
/// Original alphabet:      abcdefghijklmnopqrstuvwxyz
/// Alphabet rotated +2:    cdefghijklmnopqrstuvwxyzab
fn caesar_cypher(s: &str, k: i32) -> String {
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut j = 0;
    let mut code: Vec<char> = Vec::new();

    for (i, elem) in s.chars().enumerate() {
        let current_letter_index = indexof_char(elem, &alphabet);
        
        if current_letter_index == -1 {
            code.push(elem);
            continue;
        }

        let mut capitalized = false;
        capitalized = elem.is_uppercase();

        let encoded_letter_index = current_letter_index + k;

        j = match encoded_letter_index {
            0..=24 => encoded_letter_index,
            other => encoded_letter_index - 26
        };
        
        let new_char = alphabet[j as usize]; 

        if capitalized {
            code.push(new_char.to_uppercase().next().unwrap());
        } else {
            code.push(new_char);
        }
        
    }

    let code_string = code.iter().collect::<String>();
    return code_string;

}

fn indexof_char(searched_char: char, s: &[char] ) -> i32 {
    s.iter()
    .position(|&x| x.to_string() == searched_char.to_string().to_lowercase())
    .map(|n| n.try_into().unwrap())
    .unwrap_or(-1)
}