mod bridge;
mod editor;
pub mod engine;

use engine::Engine;
use nice_plug::prelude::*;
use std::sync::Arc;

static PAGE: editor::Page = editor::Page {
    title: "Example",
    size: (360, 220),
    html: include_str!("../../ui/index.html"),
    version: env!("CARGO_PKG_VERSION"),
    background: (27, 26, 25, 255),
    dev_url_var: "EXAMPLE_UI_URL",
};

pub struct Example {
    params: Arc<ExampleParams>,
    engine: Engine,
}

#[derive(Params)]
struct ExampleParams {
    #[id = "gain"]
    gain: FloatParam,
}

impl Default for Example {
    fn default() -> Self {
        let gain = FloatParam::new("Gain", util::db_to_gain(0.0), FloatRange::gain_range(-30.0, 30.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db());
        Self { params: Arc::new(ExampleParams { gain }), engine: Engine::new(48_000.0) }
    }
}

impl Plugin for Example {
    const NAME: &'static str = "Example";
    const VENDOR: &'static str = "Dan Marino";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type Editor = editor::WebEditor;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _executor: AsyncExecutor<Self>) -> Option<Self::Editor> {
        Some(editor::WebEditor::new(&PAGE, self.params.clone()))
    }

    fn activate(
        &mut self,
        _layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl ActivateContext<Self>,
    ) -> bool {
        if self.engine.sample_rate() != buffer_config.sample_rate {
            self.engine = Engine::new(buffer_config.sample_rate);
        }
        self.engine.reset(self.params.gain.value());
        true
    }

    fn reset(&mut self) {
        self.engine.reset(self.params.gain.value());
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.engine.set_gain(self.params.gain.value());
        if let [left, right] = buffer.as_slice() {
            self.engine.process(left, right);
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Example {
    const CLAP_ID: &'static str = "com.danmarino.example";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A gain knob");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Example {
    const VST3_CLASS_ID: [u8; 16] = *b"ExampleDanMarino";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx];
}

nice_export_clap!(Example);
nice_export_vst3!(Example);
