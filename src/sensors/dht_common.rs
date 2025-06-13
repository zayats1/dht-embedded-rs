use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin, PinState}};

use crate::{DhtError, Reading};


#[doc(hidden)]
pub struct Dht<
    HE,
    D: DelayNs,
    P: InputPin<Error = HE> + OutputPin<Error = HE>,
> {
    delay: D,
    pin: P,
}

impl<HE,D: DelayNs, P: InputPin<Error = HE> + OutputPin<Error = HE>>
    Dht<HE, D, P>
{
    pub fn new(delay: D, pin: P) -> Self {
        Self { delay, pin }
    }

    ///  User may need to disable interrupts while reading
    pub fn read(&mut self, parse_data: fn(&[u8]) -> (f32, f32)) -> Result<Reading, DhtError<HE>> {
        let mut buf: [u8; 5] = [0; 5];

        // Wake up the sensor
        self.pin.set_low()?;
        self.delay.delay_us(3000);

        // Ask for data
        self.pin.set_high()?;
        self.delay.delay_us(25);

        // Wait for DHT to signal data is ready (~80us low followed by ~80us high)
        self.wait_for_level(PinState::High, 85, DhtError::NotPresent)?;
        self.wait_for_level(PinState::Low, 85, DhtError::NotPresent)?;

        // Now read 40 data bits
        for bit in 0..40 {
            // Wait ~50us for high
            self.wait_for_level(PinState::High, 55, DhtError::Timeout)?;

            // See how long it takes to go low, with max of 70us
            let elapsed = self.wait_for_level(PinState::Low, 70, DhtError::Timeout)?;
            // If it took at least 30us to go low, it's a '1' bit
            if elapsed > 30 {
                let byte = bit / 8;
                let shift = 7 - bit % 8;
                buf[byte] |= 1 << shift;
            }
        }

        let checksum = (buf[0..=3]
            .iter()
            .fold(0u16, |accum, next| accum + *next as u16)
            & 0xff) as u8;
        if buf[4] == checksum {
            let (humidity, temperature) = parse_data(&buf);
            if !(0.0..=100.0).contains(&humidity) {
                Err(DhtError::InvalidData)
            } else {
                Ok(Reading {
                    humidity,
                    temperature,
                })
            }
        } else {
            Err(DhtError::ChecksumMismatch(buf[4], checksum))
        }
    }

    fn wait_for_level(
        &mut self,
        level: PinState,
        timeout_us: u32,
        on_timeout: DhtError<HE>,
    ) -> Result<u32, DhtError<HE>> {
        for elapsed in 0..=timeout_us {
            let is_ready = match level {
                PinState::High => self.pin.is_high(),
                PinState::Low => self.pin.is_low(),
            }?;

            if is_ready {
                return Ok(elapsed);
            }
            self.delay.delay_us(1);
        }
        Err(on_timeout)
    }
}