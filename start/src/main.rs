use std::{env, io, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let var = &args[1];
    if var.contains('-'){
        let num_1 :String = var.split('-').collect();
        let num_1 = num_1.parse().unwrap();
        for args in env::args().skip(2) {
            prnt_lines(args, num_1).unwrap();
        }
    }
    else {
        let num_1 = 10;
        for args in env::args().skip(1) {
            prnt_lines(args,num_1).unwrap();
        }
    }
}

fn prnt_lines(p: String, this_line: usize) -> io::Result<()> {
    let file_contents  = fs::read_to_string(p).unwrap();
    let mut count = 0;
    for single_lin in file_contents.lines() {
        count = count + 1;
        println!("{}", single_lin);
        if  count >= this_line {
            break;
        }
    }
    Ok(())
}
