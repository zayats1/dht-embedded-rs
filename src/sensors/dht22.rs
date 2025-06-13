use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};

use crate::{sensors::dht_common::Dht, DhtError, DhtSensor, Reading};

/// A DHT22 sensor
pub struct Dht22<
    HE,
    D: DelayNs,
    P: InputPin<Error = HE> + OutputPin<Error = HE>,
> {
    dht: Dht<HE, D, P>,
}

impl<HE, D: DelayNs, P: InputPin<Error = HE> + OutputPin<Error = HE>>
    Dht22<HE, D, P>
{
    pub fn new(delay: D, pin: P) -> Self {
        Self {
            dht: Dht::new(delay, pin),
        }
    }

    fn parse_data(buf: &[u8]) -> (f32, f32) {
        let humidity = (((buf[0] as u16) << 8) | buf[1] as u16) as f32 / 10.0;
        let mut temperature = ((((buf[2] & 0x7f) as u16) << 8) | buf[3] as u16) as f32 / 10.0;
        if buf[2] & 0x80 != 0 {
            temperature = -temperature;
        }
        (humidity, temperature)
    }
}

impl<HE,  D: DelayNs, P: InputPin<Error = HE> + OutputPin<Error = HE>>
    DhtSensor<HE> for Dht22<HE,  D, P>
{
    fn read(&mut self) -> Result<Reading, DhtError<HE>> {
        self.dht.read(Dht22::<HE, D, P>::parse_data)
    }
}
