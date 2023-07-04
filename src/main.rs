#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Pull};
use embassy_nrf::gpiote::{InputChannel, InputChannelPolarity};
use embassy_nrf::pwm::{
    Config, Prescaler, SequenceConfig, SequenceLoad, SequencePwm, SingleSequenceMode,
    SingleSequencer,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let ch1 = InputChannel::new(
        p.GPIOTE_CH0,
        Input::new(p.P1_06, Pull::Up),
        InputChannelPolarity::HiToLo,
    );

    let mut config = Config::default();
    config.prescaler = Prescaler::Div128;
    config.sequence_load = SequenceLoad::Individual;

    let mut seq_config = SequenceConfig::default();
    seq_config.refresh = 1;

    let mut pwm = unwrap!(SequencePwm::new_3ch(
        p.PWM0, p.P0_08, p.P1_09, p.P0_12, config,
    ));

    let gradients = [
        colorous::RAINBOW,
        colorous::SINEBOW,
        colorous::TURBO,
        colorous::BLUE_PURPLE,
        colorous::SPECTRAL,
        colorous::PINK_GREEN,
    ];

    let mut seq_words: [u16; 2048] = [0; 2048];

    loop {
        for gradient in gradients.iter() {
            for i in 0..512 {
                let rgb = gradient.eval_rational(i, 512);
                seq_words[i * 4] = rgb.r as u16;
                seq_words[i * 4 + 1] = rgb.g as u16;
                seq_words[i * 4 + 2] = rgb.b as u16;
            }

            let sequencer = SingleSequencer::new(&mut pwm, &seq_words, seq_config.clone());
            unwrap!(sequencer.start(SingleSequenceMode::Infinite));
            ch1.wait().await
        }
    }
}
