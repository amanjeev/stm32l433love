#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::Resolution;

use embassy_stm32::pac;

use embassy_time::Delay;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // defmt::println!("helloooooooooooo!!!");
    unsafe {
        pac::RCC.ccipr().modify(|w| {
            w.set_adcsel(0b11);
        });
        pac::RCC.ahb2enr().modify(|w| w.set_adcen(true));
    }

    let p = embassy_stm32::init(Default::default());
    let mut myadc = Adc::new(p.ADC1, &mut Delay);

    let _t = myadc.enable_temperature();

    myadc.set_resolution(Resolution::EightBit);

    let mut channel = p.PA0;
    let internal = myadc.read(&mut channel);
    defmt::println!("internal is {}", internal);

    // defmt::println!("before the loop");
    loop {
        // defmt::println!("let there be light");
        let v = myadc.read(&mut channel);
        // defmt::println!("temp u16 is: {}", v);
        let temp = ((v as f32) * ((125.0 - (-40.0)) / 256.0)) + 30.0;
        defmt::println!("Temperature is: {}", temp);
    }
}
