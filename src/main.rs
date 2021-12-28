fn main() {
    println!("┌────────────────────────────────────────────────────────────────┐");
    println!("│                  Velkommen til DM548 Tools!                    │");
    println!("│             Følgende værktøj er tilgængelige:                  │");
    println!("│      1. Boothes algorithme                                     │");
    println!("│      2. Find binary af et floating point number                │");
    println!("│      3. Konverter binær til floating point                     │");
    println!("│      4. Konverter IEEE 754 til decimal                         │");
    println!("│      5. Generer binærstreng ud fra tal og længe                │");
    println!("│      6. Konverter twos complement til heltal                   │");
    println!("│                                                                │");
    println!("│      0. Afslut programmet                                      │");
    println!("└────────────────────────────────────────────────────────────────┘");
    println!("Vælg venligst et værktøj: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Kunne ikke læse input");
    let input: u32 = input.trim().parse().expect("Kunne ikke parse input");
    match input {
        1 => boothe(),
        2 => floating_point(),
        3 => convert_bin_to_float(),
        4 => convert_ieee754(),
        5 => generate_bin(),
        6 => convert_twos_comp(),
        0 => println!("Tak for at bruge DM548 Tools!"),
        _ => println!("Ugyldigt værktøj"),
    }
    println!("Tak for at bruge DM548 Tools!");
}

fn boothe() {
    // Read bit string length
    println!("Enter bit string length: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let bit_string_length = input.trim().parse::<usize>().unwrap();

    // Read two numbers
    println!("Enter two numbers (whitespace seperated): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut q = numbers[0];
    let m = numbers[1];
    let mut q_1 = '0';
    let mut cycle = 0; 
    let mut a = 0;

    println!("Numbers in decimal: {} {}", numbers[0], numbers[1]);
    println!("Numbers in binary: {} {}", generate_binary(q, bit_string_length), generate_binary(m, bit_string_length));

    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} |", cycle, generate_binary(a, bit_string_length), generate_binary(q, bit_string_length), q_1, generate_binary(m, bit_string_length));
    for i in 0..bit_string_length {
        // let q_2 be the last digit of q plus q_1
        let mut q_2: String = generate_binary(q, bit_string_length).chars().nth(bit_string_length-1).unwrap().to_string();
        q_2.push(q_1);

        cycle = i as i32 + 1; 
        if q_2 == "10" {
            let (_temp, temp_a, temp_q, temp_q_1) = subtraction(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        } else if q_2 == "01" {
            let (_temp, temp_a, temp_q, temp_q_1) = addition(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        } else {
            println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = {}", cycle, generate_binary(a, bit_string_length), generate_binary(q, bit_string_length), q_1, generate_binary(m, bit_string_length), q_2);
            let (_temp, temp_a, temp_q, temp_q_1) = shift(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        }
    }
    println!("Final result: {}{}", generate_binary(a, bit_string_length), generate_binary(q, bit_string_length));
    println!("Final result: {}", numbers[0] * numbers[1]);
}

fn floating_point() {
    println!("Enter floating point number (starting with 0.): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<f32>().unwrap();
    find_floating_point_number(input);
}

fn convert_bin_to_float() {
    println!("Enter binary number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<String>().unwrap();
    convert_binary_to_floating_point(input);
}

fn convert_ieee754() {
    println!("Enter IEEE 754 number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<String>().unwrap();
    input.retain(|c| !c.is_whitespace());
    println!("{}",convert_ieee754_to_decimal(input));
}

fn generate_bin() {
    println!("Enter number: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();
    println!("Enter bit string length: ");
    let mut bit_string_length = String::new();
    std::io::stdin().read_line(&mut bit_string_length).unwrap();
    let bit_string_length = bit_string_length.trim().parse::<usize>().unwrap();
    println!("{}", generate_binary(input, bit_string_length));
}

fn convert_twos_comp() {
    println!("Enter binary string: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = &input.trim()[..];
    println!("Enter bit string length: ");
    let mut bit_string_length = String::new();
    std::io::stdin().read_line(&mut bit_string_length).unwrap();
    let bit_string_length = bit_string_length.trim().parse::<usize>().unwrap();
    println!("{}", twos_complement_to_i32(input, bit_string_length));
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


/// Generate binary string of length len from num
///
/// # Arguments
/// * `num` - number to convert to binary
/// * `len` - length of binary string
///
/// # Returns
/// * binary string
///
/// # Examples
/// ```
/// assert_eq!(generate_binary(23, 32), "00000000000000000000000000010111");
/// ```
fn generate_binary(num: i32, len: usize) -> String {
    let mut b = format!("{:032b}", num);
    b.replace_range(..b.len() - len, "");
    b.to_owned()
}

fn addition(a: i32, q: i32, q_1: char, m: i32, cycle: i32, len: usize) -> (i32, i32, i32, char) {
    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = 01", cycle, generate_binary(a, len), generate_binary(q, len), q_1, generate_binary(m, len));
    println!("|         |   + {} | Q = {} | Q_1 = {} | M = {} | Add {} to {}", generate_binary(m, len), generate_binary(q, len), q_1, generate_binary(m, len), m, generate_binary(a, len));
    println!("|         |     {} | Q = {} | Q_1 = {} | M = {} | ", generate_binary(a+m, len), generate_binary(q, len), q_1, generate_binary(m, len));
    shift(a+m, q, q_1, m, cycle, len)
}

/// Subtract two binary numbers and print the process
///
/// # Arguments
/// * `a` - first number
/// * `q` - second number
/// * `q_1` - leftover digit
/// * `m` - third number
/// * `cycle` - number of cycles
/// * `len` - length of bit string
///
/// # Returns
/// * i32: subtraction result
/// * i32: a, first number
/// * i32: q, second number
/// * i32: q_1, leftover digit
fn subtraction(a: i32, q: i32, q_1: char, m: i32, cycle: i32, len: usize) -> (i32, i32, i32, char) {
    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = 10", cycle, generate_binary(a, len), generate_binary(q, len), q_1, generate_binary(m, len));
    println!("|         |   + {} | Q = {} | Q_1 = {} | M = {} | Subtract {} from {}", generate_binary(-m, len), generate_binary(q, len), q_1, generate_binary(m, len), m, generate_binary(a, len));
    println!("|         |     {} | Q = {} | Q_1 = {} | M = {} | ", generate_binary(a-m, len), generate_binary(q, len), q_1, generate_binary(m, len));
    shift(a-m, q, q_1, m, cycle, len)
}

fn shift(a: i32, q: i32, q_1: char, m: i32, _cycle: i32, len: usize) -> (i32, i32, i32, char) {
    let mut shifted_value = generate_binary(a, len);
    shifted_value.push_str(&generate_binary(q, len));
    shifted_value.push(q_1);
    let first: char = shifted_value[..].chars().nth(0).unwrap();
    if first == '1' {
        shifted_value = '1'.to_string() + &shifted_value[..];
    } else if first == '0' {
        shifted_value = '0'.to_string() + &shifted_value[..];
    } else {
        println!("Error: {}", first);
    }
    println!("|         | A = {} | Q = {} | Q_1 = {} | M = {} | Shift", &shifted_value[0..len], &shifted_value[len..len*2], &shifted_value[len*2..(len*2)+1], generate_binary(m, len));
    (0, twos_complement_to_i32(&shifted_value[0..len], len), twos_complement_to_i32(&shifted_value[len..len*2], len), shifted_value[len*2..len*2+1].chars().nth(0).unwrap())
}

fn twos_complement_to_i32(num: &str, len: usize) -> i32 {
    let mut result = 0; 
    if num.to_string().as_bytes()[0] == 49 {
        result = -2_i32.pow(len as u32 - 1);
    }
    for (i, char) in num.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if char == '1' {
            result += 2_i32.pow((len - i - 1).try_into().unwrap());
        }
    }
    result
}


