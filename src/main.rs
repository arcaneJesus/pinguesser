fn main() {

}

// stores four positive integers
#[derive(Copy,Clone)]
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

    fn default(&mut self, default: u32) -> Self {
        self.default = default;
        self.reset();
        return *self;
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
        assert_eq!(pin.default, 0000);
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
    fn default_construct_test() {
        let mut pin: Pin = Pin::new().default(1111);
        assert_eq!(pin.pin, 1111);
        assert_eq!(pin.default, 1111);
        pin.next();
        pin.next();
        pin.reset();
        assert_eq!(pin.pin, 1111);
    }

    #[test]
    fn default_set_test() {
        let mut pin: Pin = Pin::new();
        pin.default(1111);
        assert_eq!(pin.pin, 1111);
        assert_eq!(pin.default, 1111);
        pin.next();
        pin.next();
        pin.reset();
        assert_eq!(pin.pin, 1111)
    }

    #[test]
    fn reset_test() {
        let mut pin: Pin = Pin::new();
        pin.next();
        pin.next();
        assert_eq!(pin.pin, 0002);
        pin.reset();
        assert_eq!(pin.pin,0000);
    }
}