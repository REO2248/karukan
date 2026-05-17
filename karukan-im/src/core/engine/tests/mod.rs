//! Tests for the IME engine

use super::*;
use crate::core::keycode::KeyModifiers;

mod basic;
mod candidates;
mod conversion;
mod cursor;
mod fkeys;
mod katakana;
mod learning;
mod live_conversion;
mod passthrough;
mod rewriter;
mod strategy;
mod surrounding;

fn press(ch: char) -> KeyEvent {
    KeyEvent::press(Keysym(ch as u32))
}

fn press_key(keysym: Keysym) -> KeyEvent {
    KeyEvent::press(keysym)
}

fn press_ctrl(keysym: Keysym) -> KeyEvent {
    KeyEvent::new(keysym, KeyModifiers::new().with_control(true), true)
}

fn press_ctrl_shift(keysym: Keysym) -> KeyEvent {
    KeyEvent::new(
        keysym,
        KeyModifiers::new().with_control(true).with_shift(true),
        true,
    )
}

fn make_live_conversion_engine() -> InputMethodEngine {
    let mut engine = InputMethodEngine::new();
    engine.live.enabled = true;
    engine
}
