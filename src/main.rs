fn hex_to_rgba(hex: String) -> (u32, u32, u32, f32) {
    let mut alpha = 1f32;
    let mut hex_on_lower = hex.clone().to_lowercase();

    match hex_on_lower.len() {
        3 | 4 => {
            let mut temp_hex = String::new();
            temp_hex.push(hex_on_lower.chars().nth(0).unwrap());
            temp_hex.push(hex_on_lower.chars().nth(0).unwrap());
            temp_hex.push(hex_on_lower.chars().nth(1).unwrap());
            temp_hex.push(hex_on_lower.chars().nth(1).unwrap());
            temp_hex.push(hex_on_lower.chars().nth(2).unwrap());
            temp_hex.push(hex_on_lower.chars().nth(2).unwrap());

            if hex_on_lower.len() == 4{
                let mut temp_alpha_hex = String::new();
                temp_alpha_hex.push(hex_on_lower.chars().nth(3).unwrap());
                temp_alpha_hex.push(hex_on_lower.chars().nth(3).unwrap());
                alpha = (u32::from_str_radix(&temp_alpha_hex, 16).unwrap()) as f32 / 255.0;    
            }
            hex_on_lower = temp_hex;
        },
        8 => {
            let mut temp_alpha_hex = String::new();
            temp_alpha_hex.push(hex_on_lower.chars().nth(6).unwrap());
            temp_alpha_hex.push(hex_on_lower.chars().nth(7).unwrap());
            alpha = (u32::from_str_radix(&temp_alpha_hex, 16).unwrap()) as f32 / 255.0;    
        }, 
        _ => {}
    }

    let decimal_val: u32 = u32::from_str_radix(&hex_on_lower, 16).unwrap();
    let red = (decimal_val >> 16) & 0xFF;
    let green = (decimal_val >> 8) & 0xFF;
    let blue = decimal_val & 0xFF;
    (red, green, blue, alpha)
}

fn main() {
    println!("F00 to rgb {:?}", hex_to_rgba(String::from("ffffffff")));
}
