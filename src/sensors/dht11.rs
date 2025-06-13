use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};

use crate::{sensors::dht_common::Dht, DhtError, DhtSensor, Reading};


/// A DHT11 sensor
pub struct Dht11<
    HE,
    D: DelayNs,
    P: InputPin<Error = HE> + OutputPin<Error = HE>,
> {
    dht: Dht<HE, D, P>,
}

impl<HE, D: DelayNs, P: InputPin<Error = HE> + OutputPin<Error = HE>>
    Dht11<HE, D, P>
{
    pub fn new(delay: D, pin: P) -> Self {
        Self {
            dht: Dht::new(delay, pin),
        }
    }

    fn parse_data(buf: &[u8]) -> (f32, f32) {
        (buf[0] as f32, buf[2] as f32)
    }
}

impl<HE,D: DelayNs, P: InputPin<Error = HE> + OutputPin<Error = HE>>
    DhtSensor<HE> for Dht11<HE, D, P>
{
    fn read(&mut self) -> Result<Reading, DhtError<HE>> {
        self.dht.read(Dht11::<HE, D, P>::parse_data)
    }
}