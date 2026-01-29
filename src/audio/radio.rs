use super::fdsp_host::FundspConfig;
use bevy::prelude::*;
use fundsp::prelude::*;

pub fn _radio() -> impl Bundle {
    let input = highpass_hz(400.0, 2.0)
            >> bell_hz(1200.0, 4.0, 1.5)
            // poor signal quality simulation
            // >> shape(SoftCrush(2.0))
            >> shape(Tanh(16.0))
            >> lowpass_hz(2800.0, 4.0)
            >> ( limiter(0.005, 0.250) * 0.25 );

    let noise = (white() * 0.1)
        >> highpass_hz(400.0, 2.0)
        >> bell_hz(1200.0, 4.0, 1.5)
        >> shape(SoftCrush(2.0))
        >> (lowpass_hz(2800.0, 4.0) * 8.0);

    let amp_adjustment = map(|i: &Frame<f32, U1>| (0.9 - i[0] * 12.0).clamp(0.0, 1.0));
    let branch = pass() & (meter(Meter::Rms(0.1)) >> (amp_adjustment * noise));

    FundspConfig::_new_downmix(input >> branch)
}
