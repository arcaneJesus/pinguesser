use enigo::*;

fn main() {
    let mut enigo = Enigo::new();

}

// stores four positive integers
pub struct Pin {
    pub pin: u32,
    default: u32
}

impl Pin {
    fn new() -> Self {
        Pin {
            pin: 0000.clamp(0,9999),
            default: 0000.clamp(0,9999)
        }
    }

    fn new(default:u32) -> Self {
        Pin {
            pin: default.clamp(0,9999),
            default: default.clamp(0,9999)
        }
    }

    fn next(&mut self) -> Self {
        Pin {
            pin: (self.pin + 1).clamp(0,9999)
        }
    }
}

#[cfg(test)]
mod pin_tests {
    use super::*;
    
    #[test]
    fn create_new_pin() {
        let mut pin: Pin = Pin::new();
        assert_eq!{pin, mut Pin {
            pin: 0000.clamp(0,9999),
            default:0000.clamp(0,9999)
        }}
    }
}