use std::io;
use ilog::IntLog;
use std::error::Error;
use std::num::ParseIntError;
use regex::Regex;

struct ParseType {
    ubound: u64,
    lbound: u64,
}

impl ParseType {
    fn new(lb: u64, ub: u64) -> Self {
        ParseType {
            lbound: lb,
            ubound: ub,
        }
    }
}

impl Iterator for ParseType {
    type Item = u64;

    fn next(self: &mut Self) -> Option<u64> {
        if self.lbound <= self.ubound {
            self.lbound += 1;
            Some(self.lbound)
        } else {
            None
        }
    }
}

trait U64EXT {
    fn digits(&self) -> u64;
    fn index_slice(&self, n: u64) -> Option<Vec<u64>>;
}

impl U64EXT for u64 {
    fn digits(&self) -> u64 {
        self.log10() as u64 + 1
    }

    fn index_slice(&self, n: u64) -> Option<Vec<u64>> {
        let digits = self.digits();
        let str: String = self.to_string();
        let mut strvec: Vec<&str> = Vec::new();

        if digits % n != 0 { return None; }
        for i in 0..n {
            let start: usize = (i * digits / n) as usize;
            let end: usize = ((i + 1) * digits / n) as usize;
            strvec.push(&str[start..end]);
        }

        let res = strvec.iter().map(|x| x.parse::<u64>()).collect();
        match res {
            Ok(n) => Some(n),
            Err(..) => panic!("What the shrigma"),
        }
    }
}


fn main() {
    let mut id_in = String::new();
    io::stdin().read_line(&mut id_in).unwrap();

    let soln = solve(&id_in);
    println!("soln: {}", soln);
}

// finds repetitions of numbers within int
fn rept_n(pt: &mut ParseType) -> u64 {
    let mut rept_sum = 0;

    'has_match: for i in pt {
        let slices: Option<Vec<u64>> = i.index_slice(2);
        let mut svec: Vec<u64> = match slices {
            Some(n) => n,
            None => continue,
        };

        let base = match svec.pop() {
            Some(n) => n,
            None => continue,
        };
        for item in svec {
            if base != item {
                continue 'has_match;
            }
        }

        rept_sum += i;
    }

    rept_sum
}

fn solve(s: &String) -> u64 {
    let mut i = 0;
    let mut sum: u64 = 0;
    loop {
        let pres: Result<ParseType, Box<dyn Error>> = parse(s, i);
        let mut pt: ParseType = match pres {
            Ok(n) => n,
            Err(..) => return sum,
        };

        sum += rept_n(&mut pt);
        i += 1
    }
}


// parses the string to upper & lower bound int. Factors in which index of range you want
// i.e. 11-22,95-115,998-1012,1188511880-1188511890,222220-222224 for index 2 would return
// lower bound of 998 and upper bound of 1012
fn parse(s: &String, i: i32) -> Result<ParseType, Box<dyn Error>> {
    Regex::new("-")?
        .split(Regex::new(",|\n|\0")?.split(s).collect::<Vec<&str>>()[i as usize])
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .map(|v| Ok(ParseType::new(v[0], v[1])))?
}
