use std::io;
use std::array;
use std::error::Error;

fn main() {
    
}

// hrmmm time complexity of stuffing an unique char from a number (represented as string rn)
// without any removal would be O(n), then at worst case 9 more tries to remove items in set
// starting from 0-9 until 2 are left so basically O(n) for each battery.
//
// For m batteries, thats O(n * m) - cringe!!!
//
// we cant cur down on the number of batteries but can we save time by not iterating through
// every item of the battery? there are repeats, etc. What are the time compexities of
// string methods like .replace? -> its O(n)... cringe...
//
// can we cheat with memory/data structures? I guess since we aren't explicitly given
// the data and forced to read in thru stdin we could do sm else but idk
// There is some minute optimisations like early termination if you've seen all 0-9 before
// it ends but worst case time complexity will always be O(n) for 1 battery
//
// sorting first is worse, O(nlogn) hrm\mmmmmm also defeats the point of ordered set
// i tihnk theres 
fn one_solve() -> u128 {
    let battery: String = read_u64().expect("unable to read anymore");
    let mut mem: [char; 10] = ['0'; 10];
    let mut index = 0;

    for c in battery.chars() {
        if !mem.contains(&c) {
            mem[index] = c;
            index += 1;
        }
    }

    for i in 0..10 {
        let ichar = (('0' as u8) + i) as char;
        if mem.len() < 2 { break; }
        if 
    }

    42
}

fn read_u64() -> Result<String, Box<dyn Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line.pop();
    Ok(line)
}
