use super::*;

// --- Katakana Conversion Tests ---
//
// Ctrl+K converts current input to katakana (one-shot conversion).
// The converted text is committed as katakana on Enter.

#[test]
fn test_ctrl_k_converts_to_katakana() {
    let mut engine = InputMethodEngine::new();

    // Type "aiueo" -> "あいうえお"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    engine.process_key(&press('e'));
    engine.process_key(&press('o'));
    assert_eq!(engine.preedit().unwrap().text(), "あいうえお");

    // Press Ctrl+k -> should convert preedit to katakana (preedit shows "アイウエオ")
    let ctrl_k = KeyEvent {
        keysym: Keysym::KEY_K,
        modifiers: KeyModifiers {
            control_key: true,
            shift_key: false,
            alt_key: false,
            super_key: false,
        },
        is_press: true,
    };
    let result = engine.process_key(&ctrl_k);

    assert!(result.consumed);
    // Should NOT commit yet - just convert display
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(!has_commit, "Should NOT commit on Ctrl+K");

    // Preedit should show katakana
    assert_eq!(engine.preedit().unwrap().text(), "アイウエオ");
    assert!(matches!(engine.state(), InputState::Composing { .. }));

    // Now press Enter -> should commit as katakana
    let enter_result = engine.process_key(&press_key(Keysym::RETURN));
    let has_katakana_commit = enter_result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(text) if text == "アイウエオ"));
    assert!(has_katakana_commit, "Should commit as katakana after Enter");
    assert!(matches!(engine.state(), InputState::Empty));
}

#[test]
fn test_ctrl_k_with_empty_input() {
    let mut engine = InputMethodEngine::new();

    // No input, Ctrl+k should do nothing harmful
    let ctrl_k = KeyEvent {
        keysym: Keysym::KEY_K,
        modifiers: KeyModifiers {
            control_key: true,
            shift_key: false,
            alt_key: false,
            super_key: false,
        },
        is_press: true,
    };
    let result = engine.process_key(&ctrl_k);

    // Should not crash, state should remain empty
    assert!(matches!(engine.state(), InputState::Empty));
    // No commit action with empty text
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(!has_commit);
}

#[test]
fn test_ctrl_k_uppercase_converts_to_katakana() {
    let mut engine = InputMethodEngine::new();

    // Type "aiueo" -> "あいうえお"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    engine.process_key(&press('e'));
    engine.process_key(&press('o'));
    assert_eq!(engine.preedit().unwrap().text(), "あいうえお");

    // Press Ctrl+K (uppercase K) -> should convert preedit to katakana
    let ctrl_k_upper = KeyEvent {
        keysym: Keysym::KEY_K_UPPER,
        modifiers: KeyModifiers {
            control_key: true,
            shift_key: false,
            alt_key: false,
            super_key: false,
        },
        is_press: true,
    };
    let result = engine.process_key(&ctrl_k_upper);

    assert!(result.consumed);
    // Should NOT commit yet
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(!has_commit, "Should NOT commit on Ctrl+K");

    // Preedit should show katakana
    assert_eq!(engine.preedit().unwrap().text(), "アイウエオ");

    // Type more → new input goes through romaji conversion (not katakana mode)
    engine.process_key(&press('a'));
    // Preedit should show katakana + new hiragana
    assert!(engine.preedit().unwrap().text().ends_with("あ"));
}
