fn main() {

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

    fn reset(&mut self) {
        self.pin = self.default;
    }
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

    #[test]
    fn reset_test() {
        let mut pin: Pin = Pin::new();
        pin.next();
        pin.next();
        assert_eq!(pin.pin, 2);
        pin.reset();
        assert_eq!(pin.pin,0);
    }
}