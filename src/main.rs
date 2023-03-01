use std::io::{Result, Read, BufReader, prelude::*};
use std::fs::File;

fn main() -> Result<()> {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let f = File::open("pi-dec.txt")?;
    let mut o = File::create("pi-b26.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    let mut pi26: Vec<u8> = Vec::new();
    let mut pi: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    for value in buffer {
        let val = value as char;
        let v = match val.to_digit(10) {
            Some(u) => u as u8,
            None => 0 as u8
        };
        pi.push(v);
    }
    pi[0] = 0;
    pi[1] = 0;
    pi26.push(3);
    o.write_all(&[alphabet[3] as u8])?;
    while pi26.len() < pi.len() {
        let mut holder: Vec<u8> = Vec::new();
        let mut carry: u32 = 0;
        for &i in pi.iter().rev() {
            let me: u32 = i as u32 * 26 + carry;
            carry = me / 10;
            holder.push((me % 10) as u8);
        }
        holder.reverse();
        pi = holder;
        let tens = pi[0] * 10;
        let ones = pi[1];
        pi[0] = 0;
        pi[1] = 0;
        let pi26d = tens + ones;
        o.write_all(&[alphabet[usize::from(pi26d)] as u8])?;
        pi26.push(pi26d);
    }
    Ok(())
}
