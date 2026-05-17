//! Mode switching (katakana conversion, live conversion)

use tracing::debug;

use super::*;

impl InputMethodEngine {
    /// Convert current input to katakana (Ctrl+K)
    /// One-shot conversion: converts displayed text to katakana and commits as katakana on Enter.
    pub(super) fn convert_to_katakana(&mut self) -> EngineResult {
        let romaji_buffer = self.converters.romaji.buffer().to_string();

        if self.input_buf.text.is_empty() && romaji_buffer.is_empty() {
            return EngineResult::consumed();
        }

        // Convert input_buf text to katakana
        if !self.input_buf.text.is_empty() {
            self.input_buf.text = karukan_engine::hiragana_to_katakana(&self.input_buf.text);
        }

        let preedit = self.set_composing_state();

        EngineResult::consumed()
            .with_action(EngineAction::UpdatePreedit(preedit))
            .with_action(EngineAction::UpdateAuxText(self.format_aux_composing()))
    }

    /// Toggle live conversion mode via Ctrl+Shift+L
    pub(super) fn toggle_live_conversion(&mut self) -> EngineResult {
        self.live.enabled = !self.live.enabled;
        let mode = if self.live.enabled { "ON" } else { "OFF" };
        debug!("Live conversion toggled: {}", mode);
        EngineResult::consumed()
            .with_action(EngineAction::UpdateAuxText(format!("ライブ変換: {}", mode)))
    }
}
