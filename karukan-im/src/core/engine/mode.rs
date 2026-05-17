//! Mode switching (katakana conversion, live conversion)

use tracing::debug;

use super::*;

impl InputMethodEngine {
    /// Convert current input to hiragana (F6)
    pub(super) fn convert_to_hiragana(&mut self) -> EngineResult {
        self.flush_romaji_to_composed();
        if self.input_buf.text.is_empty() {
            return EngineResult::consumed();
        }

        self.live.text.clear();
        self.live.frozen = true;
        let full = karukan_engine::half_width_to_full_width_katakana(&self.input_buf.text);
        self.input_buf.text = karukan_engine::katakana_to_hiragana(&full);
        let preedit = self.set_composing_state();

        EngineResult::consumed()
            .with_action(EngineAction::UpdatePreedit(preedit))
            .with_action(EngineAction::UpdateAuxText(self.format_aux_composing()))
    }

    /// Convert current input to katakana (F7 / Ctrl+K)
    /// One-shot conversion: converts displayed text to katakana and commits as katakana on Enter.
    pub(super) fn convert_to_katakana(&mut self) -> EngineResult {
        self.flush_romaji_to_composed();
        if self.input_buf.text.is_empty() {
            return EngineResult::consumed();
        }

        self.live.text.clear();
        self.live.frozen = true;
        // Convert input_buf text to katakana
        let full = karukan_engine::half_width_to_full_width_katakana(&self.input_buf.text);
        self.input_buf.text = karukan_engine::hiragana_to_katakana(&full);

        let preedit = self.set_composing_state();

        EngineResult::consumed()
            .with_action(EngineAction::UpdatePreedit(preedit))
            .with_action(EngineAction::UpdateAuxText(self.format_aux_composing()))
    }

    /// Convert current input to half-width katakana (F8)
    pub(super) fn convert_to_half_katakana(&mut self) -> EngineResult {
        self.flush_romaji_to_composed();
        if self.input_buf.text.is_empty() {
            return EngineResult::consumed();
        }

        self.live.text.clear();
        self.live.frozen = true;
        self.input_buf.text = karukan_engine::hiragana_to_half_katakana(&self.input_buf.text);
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

    /// Toggle debug info display via Ctrl+Shift+D
    pub(super) fn toggle_debug_info(&mut self) -> EngineResult {
        self.show_debug_info = !self.show_debug_info;
        let status = if self.show_debug_info { "ON" } else { "OFF" };
        debug!("Debug info display toggled: {}", status);

        // Update auxiliary text immediately based on current state
        let aux = match &self.state {
            InputState::Conversion { candidates, .. } => {
                let reading = self.input_buf.text.clone();
                self.format_aux_conversion_with_page(&reading, Some(candidates))
            }
            InputState::Composing { .. } => {
                if !self.live.text.is_empty() {
                    self.format_aux_suggest(&self.input_buf.text)
                } else {
                    self.format_aux_composing()
                }
            }
            _ => format!("デバッグ情報: {}", status),
        };

        EngineResult::consumed().with_action(EngineAction::UpdateAuxText(aux))
    }
}
