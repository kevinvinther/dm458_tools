fn main() {
    // Take floating point number input from stdin
    // println!("Enter a floating point number (0. ...): ");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input: f32 = input.trim().parse().unwrap();

    // // Print floating point number to stdout
    // println!("{}", find_floating_point_number(input));

    // Get input from user
    println!("Enter a IEEE 754: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
        
    input.retain(|c| !c.is_whitespace());

    println!("{}", convert_ieee754_to_decimal(input));
}

fn find_floating_point_number(num: f32) -> String {
    let mut binary = String::new();
    binary.push_str("0.");
    let mut sum: f32 = 0.0; 


    for i in 1..32 {
        if (1_f32/2_f32.powf(i as f32)) + sum < num {
            sum += 1_f32/2_f32.powf(i as f32);
            binary.push_str("1");
        } else if (1_f32/2_f32.powf(i as f32)) + sum == num {
            binary.push_str("1");
            break;
        } else {
            binary.push_str("0");
        }
    }
    binary
}

fn convert_binary_to_floating_point(binary: String) -> f32 {
    let mut sum: f32 = 0.0;
    let mut i: usize = 1;
    for c in binary.chars() {
        if c == '1' {
            sum += 1_f32/2_f32.powf(i as f32);
        }
        i += 1;
    }
    sum
}

fn convert_ieee754_to_decimal(binary: String) -> f32 {
    if binary.len() != 32 {
        panic!("Invalid binary number, has to be 32 bits correct IEEE 754 format\nTry removing all spaces\nExample input: 00111111000001100110011001100110\nYou inputted:  {}",  binary);
    }
    let sign = &binary[0..1];
    println!("Sign bit: {}", sign);
    let exponent = &binary[1..9];
    println!("Exponent: {}", exponent);
    let mantissa = &binary[9..32];
    println!("Mantissa: {}", mantissa);

    let exponent_decimal = isize::from_str_radix(exponent, 2).unwrap();
    println!("Exponent decimal: {}", exponent_decimal);
    let mantissa_decimal = convert_binary_to_floating_point(mantissa.to_string());
    println!("Mantissa decimal = {}/2^23", isize::from_str_radix(mantissa, 2).unwrap());
    println!("Mantissa decimal: {}", mantissa_decimal);

    let f = 1_f32 + mantissa_decimal;
    println!("f = 1 + {}/2^32 = 1 + {} = {}", isize::from_str_radix(mantissa, 2).unwrap(), mantissa_decimal, f);
    let e = exponent_decimal - 127;
    println!("e = {} - 127 = {}", exponent_decimal, e);
    let base: f32 = -1.0;
    let v = base.powf(sign.parse::<isize>().unwrap() as f32) * 2_f32.powf(e as f32) * f; 
    println!("v = (-1)^s * 2^e * f");
    println!("v = (-1)^{} * 2^{} * {} = {}", sign, e, f, v);


    println!("\nAttention: 
        \n\tIf the exponent is all 1s, and the mantissa is all 0s the number is ±∞
        \n\tIf the exponent is all 1s, and the matissa contains something other than all 0's we get NaN
        \n\tIf everything is 0, then the number is 0");
    v
}
