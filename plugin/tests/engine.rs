mod support;

use example::engine::Engine;

const SR: f32 = 48_000.0;

/// Runs mono input through the engine in host-sized blocks and returns the
/// left output.
fn render(input: &[f32], gain: f32) -> Vec<f32> {
    let mut engine = Engine::new(SR);
    engine.set_gain(gain);
    let mut left = input.to_vec();
    let mut right = input.to_vec();
    for (l, r) in left.chunks_mut(512).zip(right.chunks_mut(512)) {
        engine.process(l, r);
    }
    left
}

#[test]
fn gain_ramp_matches_golden() {
    let input = support::tone(110.0, 0.25, SR);
    support::check_golden("gain-ramp", &render(&input, 0.5), SR as u32);
}

#[test]
fn unity_gain_passes_input_through() {
    let input = support::tone(110.0, 0.1, SR);
    assert_eq!(render(&input, 1.0), input);
}
