use std::env::args;

fn main() {
    let args = args();
    let num = args.skip(1).next().unwrap().parse::<u64>().unwrap();
    print_num(num);
}

fn print_num(num: u64) {
    println!("hexdecimal: {:x}", num);
    println!("   decimal: {}", num);
    println!("     octal: {:o}", num);
    println!("    binary: {:b}", num);

    print!("  bits set: ");
    bits_set(num);
    print!("\n");
}

fn bits_set(mut num: u64) {
    let mut result: Vec<u8> = Vec::new();
    let mut n = 0;
    while num > 0 {
        if num & 1 == 1 {
            result.push(n);
        }
        n += 1;
        num = num >> 1;
    }

    for c in result.iter().rev() {
        print!("{} ", c);
    }
}
