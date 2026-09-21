#![no_std]

pub fn blink_delay(clock_hz: u32, presses: u32) -> u32 {
    clock_hz / (10+presses.min(10))
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn first_click() {
        assert_eq!(
            blink_delay(125_000_000),
            125_000_000 / 11
        );
    }
}

