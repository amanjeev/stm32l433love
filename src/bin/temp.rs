#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::Resolution;
use embassy_stm32::adc::Temperature;
use embassy_stm32::pac;
use embassy_stm32::pac::adc;
use embassy_stm32::pac::GPIO;
use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    defmt::debug!("helloooooooooooo!!!");
    unsafe {
        pac::RCC.ccipr().modify(|w| {
            w.set_adcsel(0b11);
        });
        pac::RCC.ahb2enr().modify(|w| w.set_adcen(true));
    }

    let mut p = embassy_stm32::init(Default::default());
    let mut myadc = Adc::new(p.ADC1, &mut Delay);

    let t = myadc.enable_temperature();

    myadc.set_resolution(Resolution::EightBit);

    let mut channel = p.PA0;
    // let internal = myadc.read(&mut channel);
    // defmt::info!("internal is {}", internal);

    defmt::info!("before the loop");
    loop {
        defmt::info!("let there be light");
        let v = myadc.read(&mut channel);
        defmt::info!("temp u16 is: {}", v);
    }
}
