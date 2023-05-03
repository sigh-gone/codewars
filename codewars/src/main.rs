fn main() {
    /*assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 11,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, false);*/

    //let a = find_next_square(625);
    //println!("{:?}", a);
    /*
    let small = increment_string("foobar00");

    println!("{:?}", small);
    let small = increment_string("foobar00999");

    //let small = increment_string("foobar01");

    println!("{:?}", small);
    println!("{:?}", num_as_roman(1966));
    */

    //testequal(3.0, 0.66, 1.5, 3);
    //testequal(30.0, 0.66, 1.5, 15);
    //testequal(40.0, 0.4, 10.0, 3);
    //testequal(10.0, 0.6, 10.0, -1);

    /*dotest(1, 10, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    dotest(1, 100, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
    dotest(10, 89, &[89]);
    dotest(10, 100, &[89]);
    dotest(90, 100, &[]);
    dotest(89, 135, &[89, 135]);*/

    /*assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );*/
    //examples();

    //basics_order_weight();
    sample_tests();
}

fn parse(code: &str) -> Vec<i32> {
    let mut retvec: Vec<i32> = vec![];
    let mut val = 0;
    code.chars().for_each(|c| match c {
        'i' => val = val + 1,
        'd' => val = val - 1,
        's' => val = val * val,
        'o' => retvec.push(val.clone()),
        _ => {}
    });
    retvec
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}
fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    a = a.iter().map(|x| x.pow(2)).collect();
    a.sort();
    b.sort();
    a == b
}

fn find_next_square(sq: u64) -> Option<u64> {
    /*
    //first try
    let mut index = sq;
    let upper_limit = f64::sqrt(index as f64) as i64;
    if upper_limit * upper_limit != sq as i64 {
        return None;
    } else {
        index += 1;
        loop {
            let upper_limit = f64::sqrt(index as f64).floor();
            if upper_limit * upper_limit == index as f64 {
                return Some(index);
            } else {
                index += 1;
            }
        }
    }*/

    //or better

    let root = (sq as f64).sqrt();
    if root != root.floor() {
        return None;
    }

    Some((root as u64 + 1).pow(2))
}

fn move_zeros(xs: &[u8]) -> Vec<u8> {
    /*  let mut ret_vec = Vec::new();
        let mut cnt = 0;
        for i in arr {
            if *i == 0 {
                cnt += 1;
            } else {
                ret_vec.push(*i);
            }
        }
        let mut zeros = vec![0; cnt];
        ret_vec.append(&mut zeros);
        ret_vec

    */
    let mut ys = Vec::with_capacity(xs.len());
    ys.extend(xs.iter().filter(|&&x| x != 0));
    ys.resize(xs.len(), 0);
    ys
}

fn count_bits(mut decimal: i64) -> u32 {
    // code here
    if decimal == 0 {
        //decimal as u32
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                //bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        // reverse the bits
        // bits.len() as u32
    }

    decimal.count_ones()
}

fn dig_pow(n: i64, p: i32) -> i64 {
    // someone elses code
    let r: i64 = n
        .to_string()
        .chars()
        .map(|c| (c as i64) - 48)
        .enumerate()
        .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
        .sum();

    match r % n {
        0 => r / n,
        _ => -1,
    }
}

/*

[]                                -->  "no one likes this"
["Peter"]                         -->  "Peter likes this"
["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"

 */

fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".into(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!(
            "{},  {} and {} others like this",
            names[0],
            names[1],
            names.len() - 2
        ),
    }

    //or

    /*

        match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }

     */
}

fn narcissistic(num: u64) -> bool {
    //times out
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .enumerate()
        .into_iter()
        .map(|(i, x)| x.to_digit(10).unwrap())
        .collect();
    let amount_of_digits = digits.len();

    let sum: u32 = digits
        .into_iter()
        .map(|x| x.pow(amount_of_digits as u32))
        .sum();

    sum as u64 == num
}

use core::num;
use std::collections::{HashMap, HashSet};

use regex::Regex;
fn increment_string(s: &str) -> String {
    let first_seperator = Regex::new(r"[0-9]+|[A-Za-z]+").expect("Invalid regex");
    let mut final_string = String::new();
    let mut index = 0;
    let length_of_matches = first_seperator.captures(s).unwrap().len();

    if length_of_matches < 3 {
        for cap in first_seperator.captures_iter(s) {
            let text1 = cap.get(0).unwrap().as_str();
            let mut z_cnt = 0;
            let mut num_string = String::new();
            match text1.parse::<i64>() {
                Ok(mut number) => {
                    for a in text1.chars() {
                        if a == '0' {
                            z_cnt += 1;
                        } else {
                            break;
                        }
                    }
                    if number == 0 {
                        z_cnt -= 1;
                    }
                    index = index + 1;
                    number = number + 1;

                    if number % 10 == 0 && number > 1 {
                        for _i in 0..z_cnt - 1 {
                            num_string.push_str("0");
                        }
                        num_string.push_str(&number.to_string())
                    } else if number % 10 != 0 {
                        for _i in 0..z_cnt {
                            num_string.push_str("0");
                        }
                        num_string.push_str(&number.to_string())
                    } else if number == 1 {
                        for _i in 0..z_cnt - 1 {
                            num_string.push_str("0");
                        }
                        num_string.push_str(&number.to_string())
                    } else {
                        num_string.push_str(&number.to_string())
                    }
                    final_string.push_str(&num_string)
                }
                Err(_) => {
                    final_string.push_str(text1);
                }
            };
        }

        if index == 0 || final_string.len() < 1 {
            final_string.push_str("1")
        }
    } else {
        final_string.push_str(&s.to_string());
        final_string.push_str("1");
    }
    final_string
}

fn num_as_roman(mut num: i32) -> String {
    let mut letters = String::new();
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for &(n, symbol) in symbols.iter() {
        while num >= n {
            letters.push_str(symbol);
            num -= n;
        }
    }
    letters
}
fn bouncing_ball(mut h: f64, bounce: f64, window: f64) -> i32 {
    // your code
    let mut bounces = 0;
    while h >= window {
        println!("{} {} {}", h, bounce, window);
        bounces += 1;
        h = h * bounce;
    }
    (bounces * 2) - 1
}
fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    // your code
    let mut ret_vec: Vec<(u64, u64)> = vec![];
    for i in m..n {
        for j in 0..i {
            if i % j == 0 {}
        }
    }
    vec![]
}

fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut ret_vec: Vec<u64> = (a..b + 1)
        .into_iter()
        .map(|a| {
            let number = a
                .to_string()
                .chars()
                .enumerate()
                .map(|(mut b, c)| {
                    b += 1;
                    let number = match c.to_digit(10) {
                        Some(digit) => digit.pow(b as u32) as u64,
                        None => 0,
                    };
                    number
                })
                .sum();
            println!("{} {:?}", a, number);
            if number == a {
                number
            } else {
                0
            }
        })
        .collect();

    ret_vec = ret_vec.into_iter().filter(|a| *a != 0).collect();

    ret_vec

    //or
    /*

             (a..=b)
         .filter(|n| {
             *n == n.to_string()
                 .chars()
                 .map(|c| c.to_string().parse::<u64>().unwrap())
                 .enumerate()
                 .map(|(i, v)| v.pow(i as u32 + 1))
                 .sum::<u64>()
         })
     .collect::<Vec<u64>>()

    */
}
fn dotest(a: u64, b: u64, expected: &[u64]) {
    let actual = sum_dig_pow(a, b);
    assert!(
        actual == expected,
        "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}"
    )
}

fn high(input: &str) -> String {
    let mut ret_word = (0, "");
    let input = input.to_ascii_lowercase();

    for word in input.split(" ") {
        let mut temp = 0;

        for char in word.chars() {
            match char {
                'a' => temp += 1,
                'b' => temp += 2,
                'c' => temp += 3,
                'd' => temp += 4,
                'e' => temp += 5,
                'f' => temp += 6,
                'g' => temp += 7,
                'h' => temp += 8,
                'i' => temp += 9,
                'j' => temp += 10,
                'k' => temp += 11,
                'l' => temp += 12,
                'm' => temp += 13,
                'n' => temp += 14,
                'o' => temp += 15,
                'p' => temp += 16,
                'q' => temp += 17,
                'r' => temp += 18,
                's' => temp += 19,
                't' => temp += 20,
                'u' => temp += 21,
                'v' => temp += 22,
                'w' => temp += 23,
                'x' => temp += 24,
                'y' => temp += 25,
                'z' => temp += 26,
                _ => {}
            }
        }
        if temp > ret_word.0 {
            ret_word.0 = temp;
            ret_word.1 = word;
        }
    }
    ret_word.1.to_string()
}

fn valid_parentheses(s: &str) -> bool {
    let mut opened = 0usize;
    for ch in s.chars() {
        match ch {
            '(' => opened += 1,
            ')' if opened > 0 => opened -= 1,
            ')' => return false,
            _ => (), // ignore
        }
    }
    opened == 0
}

fn zeros(mut n: u64) -> u64 {
    let mut count = 0;

    let mut i = 5;
    while n / i >= 1 {
        count += n / i;
        i *= 5;
    }
    count

    //or

    /*

    println!("N -> {}", n);
     if n == 0 { 0 }
     else { n / 5 + zeros(n / 5) }

      */
}

fn alphabet_position(text: &str) -> String {
    // Code here...
    /*let letters : Vec<(usize, char)>= "abcdefghijklmnopqrstuwxyz"
            .to_string()
            .chars()
            .enumerate()
            .map(|(a, b)| (a + 1, b))
            .collect();
    */
    let mut ret_word = String::new();
    let input = text.to_ascii_lowercase();

    for word in input.split(" ") {
        println!("{:?}", word);

        for char in word.chars() {
            match char {
                'a' => ret_word.push_str("1 "),
                'b' => ret_word.push_str("2 "),
                'c' => ret_word.push_str("3 "),
                'd' => ret_word.push_str("4 "),
                'e' => ret_word.push_str("5 "),
                'f' => ret_word.push_str("6 "),
                'g' => ret_word.push_str("7 "),
                'h' => ret_word.push_str("8 "),
                'i' => ret_word.push_str("9 "),
                'j' => ret_word.push_str("10 "),
                'k' => ret_word.push_str("11 "),
                'l' => ret_word.push_str("12 "),
                'm' => ret_word.push_str("13 "),
                'n' => ret_word.push_str("14 "),
                'o' => ret_word.push_str("15 "),
                'p' => ret_word.push_str("16 "),
                'q' => ret_word.push_str("17 "),
                'r' => ret_word.push_str("18 "),
                's' => ret_word.push_str("19 "),
                't' => ret_word.push_str("20 "),
                'u' => ret_word.push_str("21 "),
                'v' => ret_word.push_str("22 "),
                'w' => ret_word.push_str("23 "),
                'x' => ret_word.push_str("24 "),
                'y' => ret_word.push_str("25 "),
                'z' => ret_word.push_str("26 "),
                _ => {}
            }
        }
    }
    ret_word.trim().to_string()

    /*
           //or

           text.to_lowercase()
               .chars()
               .filter(|c| c >= &'a' && c <= &'z')
               .map(|c| (c as u32 - 96).to_string())
               .collect::<Vec<String>>()
               .join(" ")



    */
}
pub fn range_extraction(a: &[i32]) -> String {
    // Your solution here
    let mut lowest = 0;
    let mut highest = 0;
    let mut last = 0;
    let mut ret_string = String::new();
    let mut temp_string = String::new();
    for i in a.iter() {
        if highest < *i {
            highest = *i
        }
        if lowest > *i {
            lowest = *i
        }
        if *i != last + 1 {
            ret_string.push_str(&format!("{}-{}", lowest, highest));
            temp_string = "".to_string();
        } else {
            temp_string.push_str(&format!(", {}", i))
        }

        last = *i;
    }

    ret_string.push_str(&temp_string);
    ret_string
}

fn is_perfect_power(x: u64) -> Option<(u64, u32)> {
    let mut k: u32;
    let mut n: u64 = 2;
    let mut y: u64;

    while n * n <= x {
        if x % n == 0 {
            y = x;
            k = 0;
            while y % n == 0 {
                y /= n;
                k += 1;
            }
            if y == 1 {
                return Some((n, k));
            }
        }
        n += 1;
    }

    return None;
}

fn int32_to_ip(int: u32) -> String {
    //Ipv4Addr::from(int).to_string()
    let a = (int & 0xFF000000) >> 24;
    let b = (int & 0x00FF0000) >> 16;
    let c = (int & 0x0000FF00) >> 8;
    let d = (int & 0x000000FF);

    format!("{}.{}.{}.{}", a, b, c, d)
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // your code
    let mut complements = HashSet::new();
    for &x in ints {
        let comp = s - x;
        match complements.get(&comp) {
            Some(&y) => return Some((y, x)),
            None => complements.insert(x),
        };
    }
    None
}

fn josephus_survivor(n: i32, k: i32) -> i32 {
    // your code here
    if n == 1 {
        return 1;
    } else {
        return (josephus_survivor(n - 1, k) + k - 1) % n + 1;
    }

    //or
    /*

        let mut survivors = (1..=n).collect::<Vec<_>>();
    while survivors.len() > 1 {
        let rotate_by = k as usize % survivors.len();
        survivors.rotate_left(rotate_by);
        survivors.pop();
    }
    survivors[0]

     */
}
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{0:02X}{1:02X}{2:02X}",
        r.min(255).max(0),
        g.min(255).max(0),
        b.min(255).max(0)
    )
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();
    let mut input = input.into_iter();
    let mut output = vec![];
    let mut data = [0u8; 5000];
    let mut cp = 0;
    let mut dp = 0;
    while cp < code.len() {
        match code[cp] {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => data[dp] = data[dp].wrapping_add(1),
            b'-' => data[dp] = data[dp].wrapping_sub(1),
            b'.' => output.push(data[dp]),
            b',' => data[dp] = input.next().expect("input"),
            b'[' if data[dp] == 0 => cp += jump(code[cp..].iter()),
            b']' if data[dp] != 0 => cp -= jump(code[0..cp + 1].iter().rev()),
            _ => {}
        }
        cp += 1;
    }
    output
}

fn jump<'a, I: 'a>(code: I) -> usize
where
    I: Iterator<Item = &'a u8>,
{
    let mut n = 0;
    for (i, &c) in code.enumerate() {
        if c == b'[' {
            n += 1;
        }
        if c == b']' {
            n -= 1;
        }
        if n == 0 {
            return i;
        }
    }
    unreachable!();
}

pub fn decode_bits(encoded: &str) -> String {
    println!("{:?}", encoded);
    let mut ret_string = "".to_string();
    let unit = get_unit(encoded);
    let regex = format!(
        "1{{{}}}|0{{{}}}|0{{{}}}|1{{{}}}|0{{{}}}",
        unit * 3,
        unit * 7,
        unit * 3,
        unit,
        unit
    );
    println!("{:?}", regex);
    let re = Regex::new(&regex).unwrap();
    for cap in re.find_iter(encoded) {
        let some: &str = cap.into();
        if unit == some.len() && some.contains("1") {
            ret_string.push_str(".")
        } else if unit * 3 == some.len() && some.contains("1") {
            ret_string.push_str("-")
        } else if unit * 7 == some.len() && some.contains("0") {
            ret_string.push_str("/")
        } else if unit * 3 == some.len() && some.contains("0") {
            ret_string.push_str(" ")
        }
    }
    println!("{:?}", ret_string);
    ret_string
}

pub fn get_unit(encoded: &str) -> usize {
    let mut min = 1000000;
    for split in encoded.split('0') {
        if min > split.len() && split.len() > 0 {
            min = split.len();
            println!("{:?}", split.len());
        }
    }
    min
}

pub fn decode_morse(encoded: &str) -> String {
    let mut ret_string = "".to_string();
    for word in encoded.split("/") {
        for letter in word.split(" ") {
            /*if let Some(le) = morse_code.get(letter) {
                ret_string.push_str(&morse_code(le));
            }*/
        }
        ret_string.push_str(" ");
    }
    ret_string.trim().to_string()
}

pub fn morse_code(letter: &str) -> String {
    let mut book_reviews: HashMap<String, String> = HashMap::new();

    "".into()
}

pub fn match_letter(letter: &str) -> String {
    "".into()
}

fn examples() {
    //assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
    let stuff = decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011");
    println!("{:?}", stuff);
    //decode_morse(".--");
}

fn order_weight(s: &str) -> String {
    let mut s = s.split(" ").collect::<Vec<_>>();
    s.sort_unstable();
    s.sort_by_key(|n| n.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
    s.join(" ")
}

fn testings(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

fn basics_order_weight() {
    testings("103 123 4444 99 2000", "2000 103 123 4444 99");
    testings(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
fn valid_isbn10(isbn: &str) -> bool {
    let mut sum = 0;

    for (i, char) in isbn.char_indices() {
        match char.to_digit(10) {
            Some(x) => {
                let mo = i + 1;
                sum = sum + x.pow((mo).try_into().unwrap());
                println!("{:?}", x);
                println!("{:?}", sum);
            }
            None => {}
        }
    }

    sum % 11 != 0
}

fn dotest1(isbn: &str, expected: bool) {
    let actual = valid_isbn10(isbn);
    assert!(
        actual == expected,
        "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
    )
}

fn sample_tests() {
    dotest1("1112223339", true);
    dotest1("048665088X", true);
    dotest1("1293000000", true);
    dotest1("1234554321", true);
    dotest1("1234512345", false);
    dotest1("1293", false);
    dotest1("X123456788", false);
    dotest1("ABCDEFGHIJ", false);
    dotest1("XXXXXXXXXX", false);
    dotest1("123456789T", false);
}
