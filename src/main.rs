//use enigo::*;

fn main() {
    //let mut enigo = Enigo::new();

}

// stores four positive integers
pub struct Pin {
    pub pin: u32,
    default: u32
}

impl Pin {
    fn new() -> Self {
        Pin {
            pin: 0000,
            default: 0000
        }
    }

    fn next(&mut self) {
        self.pin += 1;
    }

    // TODO: add reset function
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_test() {
        let mut pin: Pin = Pin::new();
        assert_eq!{pin.pin,0000};
        pin.next();
        assert_eq!(pin.pin,0001);
    }

    #[test]
    fn full_next_test() {
        let mut pin: Pin = Pin::new();
        for i in 0..10000 {
            assert_eq!(pin.pin, i);
            pin.next();
        }
    } 
}