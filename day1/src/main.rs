use std::io;

struct Rot {
    dir: String,
    size: i32,
}

impl Rot {
    pub fn new(s: String) -> Self {
        let len = s.len() - 1;
        let trim = &s[0..len];

        let rot = &trim[0..1];
        let size = &trim[1..len];
        let isize: i32 = size.parse::<i32>().unwrap();

        Rot {
            dir: rot.to_string(),
            size: isize,
        }
    }

    pub fn apply_rot(&self, i: &mut i32, zeros: &mut i32) -> () {
        let mut s: i32 = self.size % 100;
        let prev = i.clone();

        if self.dir == String::from("L") {
            s *= -1;
        }

        // lookahead
        let lookahead = *i + s;
        if lookahead < 0 {
            // underflow
            *i = 100 + lookahead;
        } else if lookahead > 99 {
            *i = lookahead - 100;
        } else {
            *i += s;
        }

        if *i == 0 { *zeros += 1; }
        println!("{}{}: from {} to {}", self.dir, self.size, prev, i);
    }
}

fn main() {
    let mut start: i32 = 50;
    let mut zeros: i32 = 0;

    loop {
        let input: String = read();
        let r: Rot = Rot::new(input);

        r.apply_rot(&mut start, &mut zeros);
        println!("zeros: {zeros}");
    }
}

fn read() -> String {
    let mut input = String::new();
    input.clear();

    let res = io::stdin()
        .read_line(&mut input);

    match res {
        Ok(..) => {},
        Err(err) => println!("Err {err}"),
    }

    input
}
