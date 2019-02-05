use clap::{Arg, App};
use rand::Rng;

const CHECK_CODES: [i32; 8] = [7, 9, 5, 3, 2, 4, 6, 8];

fn get_digits(number: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = number;

    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }

    digits.push(n);

    digits
}

fn main() {
    let matches = App::new("International post tracking numbers generator")
                      .version("1.0")
                      .arg(Arg::with_name("count")
                          .short("c")
                          .long("count")
                          .value_name("INTEGER")
                          .help("Number of tracking numbers to generate (default: 1)"))
                      .arg(Arg::with_name("PREFIX")
                          .required(true)
                          .index(1))
                      .arg(Arg::with_name("POSTFIX")
                          .required(true)
                          .index(2))
                      .get_matches();

    let mut rng = rand::thread_rng();

    let prefix = matches.value_of("PREFIX").unwrap();
    let postfix = matches.value_of("POSTFIX").unwrap();

    for _ in 1..matches.value_of("count").unwrap_or("1").parse::<i32>().unwrap()+1 {
        let id = rng.gen_range(0, 100000000);

        let mut check = 0;
        let mut i = 0;
        for number in get_digits(id) {
            check += number * CHECK_CODES[i];
            i += 1;
        }
        check = 11 - check % 11;

        check = match check {
            10 => 0,
            11 => 5,
            _ => check
        };

        println!("{}{:08}{}{}", prefix, id, check, postfix);
    }
}
