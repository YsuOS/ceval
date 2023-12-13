use std::env::args;

fn main() {
    let args = args();
    let val = args.skip(1).next().unwrap();
    let num = if val.len() > 1 && &val[..2] == "0x" {
        u64::from_str_radix(&val[2..], 16).expect("Unexpected value")
    } else {
        val.parse::<u64>().expect("Unexpected value")
    };
    print_num(num);
}

fn print_num(num: u64) {
    println!("hexdecimal: {:x}", num);
    println!("   decimal: {}", num);
    println!("     octal: {:o}", num);
    println!("    binary: {:064b}", num);
    let bs = get_bits_set(num);
    println!("  bits set: {}", bs);
}

fn get_bits_set(mut num: u64) -> String {
    let mut bs = String::new();
    let mut n: u8 = 0;
    while num > 0 {
        if num & 1 == 1 {
            bs = format!("{} {}", n.to_string(), &bs);
        }
        n += 1;
        num = num >> 1;
    }
    bs
}
