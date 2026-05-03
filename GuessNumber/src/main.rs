mod dice;
use std::io::Read;
use dice::Dice;

fn main() {
    println!("Hello, world!");
    let mut d: Dice = Dice { value: 1 };
    d.roll();
    loop {
        let mut buffer: [u8; 3] = [0; 3];
        let _ = std::io::stdin().read_exact(&mut buffer);
        println!("Buffer: {0}, Value: {1}", buffer[0], d.value);
        if (buffer[0] - 48) == d.value {
            break;
        }
    }

}
