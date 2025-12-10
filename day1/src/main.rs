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

    pub fn apply_rot(&self, i: &mut i32, zeros: &mut i32, state: &mut bool) -> () {
        let prev = i.clone();
        let mut s: i32 = self.size;
        if self.dir == String::from("L") { s *= -1; }
        *i += s;

        // *i is beyond the current range
        // to check if passthrough has occured, there are a few cases:
        // 0: rotation PAST 0 (i.e. from 80 R40 to 20, from 20 L40 to 80)
        // 1: rotation HITS 0 (i.e. from 40 L40 to 0; next rotation shouldn't count)
        //      a: that means that we check if flag set for next rot is true
        while *i > 99 || *i < 0 {
            if *i > 99 {
                *i -= 100;
                *zeros += 1;
                *state = true;
            }
            if *i < 0 {
                *i += 100;
                if !*state {
                    *zeros += 1;
                } else {
                    *state = false;
                }
            }

        }

        if *i == 0 {
            if !*state {
                *zeros += 1;
            }

            *state = true;
        } else {
            *state = false;
        }
        println!("{}{}: from {} to {}", self.dir, self.size, prev, *i);
    }

}

fn main() {
    let mut start: i32 = 50;
    let mut zeros: i32 = 0;
    let mut state: bool = false;

    loop {
        let input: String = read();
        if input == "" {
            return;
        }
        let r: Rot = Rot::new(input);

        r.apply_rot(&mut start, &mut zeros, &mut state);
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
