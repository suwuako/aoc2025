use std::io;
use std::error::{Error};
use regex::{Regex};

struct ParseType {
    ubound: u64,
    lbound: u64,
}

impl ParseType {
    fn new() -> Self {
        ParseType {
            ubound: 0,
            lbound: 0,
        }
    }
}

fn main() {
    let mut id_in = String::new();
    io::stdin().read_line(&mut id_in).unwrap();

    solve(&id_in);
    println!("exit");
}

fn solve(s: &String) -> Box<dyn Error> {
    let mut i = 0;
    loop {
        let pres: Result<ParseType, Box<dyn Error>> = parse(s, i);
        let pt: ParseType = match pres {
            Ok(n) => {
                n 
            }
            Err(e) => {
                println!("End reached, error: {}", e);
                return e;
            }
        };
        println!("{}", pt.lbound < pt.ubound);

        i += 1
    }
}


// parses the string to upper & lower bound int. Factors in which index of range you want
// i.e. 11-22,95-115,998-1012,1188511880-1188511890,222220-222224 for index 2 would return
// lower bound of 998 and upper bound of 1012
fn parse(s: &String, i: i32) -> Result<ParseType, Box<dyn Error>> {
    let range: Regex = Regex::new(",")?;
    let low_high: Regex = Regex::new("-")?;
    let res: Vec<u64> = low_high.split(range.split(s).collect::<Vec<&str>>()[i as usize])
        .map(|x| x.parse::<u64>()).collect::<Result<_, _>>()?;

    Ok(ParseType {
        lbound: res[0],
        ubound: res[1],
    })
}
