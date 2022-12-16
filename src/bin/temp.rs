#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::AdcPin;
use embassy_stm32::adc::Resolution;
use embassy_stm32::adc::Temperature;
use embassy_stm32::pac::GPIO;
use embassy_stm32::pac::adc;
use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_stm32::init(Default::default());
    let mut myadc = Adc::new(p.ADC1, &mut Delay);
    let t = myadc.enable_temperature();
    myadc.set_resolution(Resolution::default());
    myadc.enable_vbat();
    let channel = p.PA0;
    // let internal = myadc.read

    loop {
        info!("let there be light");
        let v = myadc.read(&mut Temperature);
        info!("temp u16 is: {}", v);
    }
}
