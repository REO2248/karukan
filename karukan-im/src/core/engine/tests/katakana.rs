use super::*;

// --- Katakana Conversion Tests ---
//
// F7 converts current input to katakana (one-shot conversion).
// The converted text is committed as katakana on Enter.

#[test]
fn test_f7_converts_to_katakana() {
    let mut engine = InputMethodEngine::new();

    // Type "aiueo" -> "あいうえお"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    engine.process_key(&press('e'));
    engine.process_key(&press('o'));
    assert_eq!(engine.preedit().unwrap().text(), "あいうえお");

    // Press F7 -> should convert preedit to katakana (preedit shows "アイウエオ")
    let f7 = press_key(Keysym::F7);
    let result = engine.process_key(&f7);

    assert!(result.consumed);
    // Should NOT commit yet - just convert display
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(!has_commit, "Should NOT commit on F7");

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
fn test_f7_with_empty_input() {
    let mut engine = InputMethodEngine::new();

    // No input, F7 should do nothing harmful
    let f7 = press_key(Keysym::F7);
    let result = engine.process_key(&f7);

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
fn test_f7_converts_and_continues() {
    let mut engine = InputMethodEngine::new();

    // Type "aiueo" -> "あいうえお"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    engine.process_key(&press('e'));
    engine.process_key(&press('o'));
    assert_eq!(engine.preedit().unwrap().text(), "あいうえお");

    // Press F7 -> should convert preedit to katakana
    let f7 = press_key(Keysym::F7);
    let result = engine.process_key(&f7);

    assert!(result.consumed);
    // Should NOT commit yet
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(!has_commit, "Should NOT commit on F7");

    // Preedit should show katakana
    assert_eq!(engine.preedit().unwrap().text(), "アイウエオ");

    // Type more → new input commits the previous katakana and starts new hiragana
    let result = engine.process_key(&press('a'));
    
    // Verify commit action is present (as per the refined behavior)
    let committed = result.actions.iter().any(|a| matches!(a, EngineAction::Commit(text) if text == "アイウエオ"));
    assert!(committed, "Should commit 'アイウエオ'");

    // Preedit should show the new character
    assert_eq!(engine.preedit().unwrap().text(), "あ");
}

