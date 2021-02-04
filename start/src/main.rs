use std::{env, io};
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() -> io::Result<()> {
    for filename in env::args().skip(1) {
        let i = 0;
        for char in filename.chars().clone() {
            if char.clone() == '-' {
                let num_1 = getnum(&filename);
                let num_1 = num_1.parse().unwrap();
                for filename in env::args().skip(2) {
                    let file = File::open(&filename)?;
                    let reader = BufReader::new(file);
                    for line in reader.lines().filter_map(|result| result.ok()) {
                        let i = i + 1;
                        while i < num_1 {
                            println!("{}", line);
                        }
                    }
                }
            }
            else {
                let file = File::open(&filename)?;
                let reader = BufReader::new(file);
                for line in reader.lines().filter_map(|result| result.ok()) {
                    let i = i + 1;
                    while i <= 10 {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn getnum(s: &String) -> String{
    let num = s.chars().skip(1).collect();
    return num
}