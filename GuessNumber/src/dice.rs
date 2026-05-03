use rand::prelude::*;
pub struct Dice {
    pub value : u8
}

impl Dice {
    pub fn roll(&mut self) {
        let mut rng = rand::rng();
        let number = (rng.random::<u8>() % 6) + 1;
        self.value = number;
    }
}
