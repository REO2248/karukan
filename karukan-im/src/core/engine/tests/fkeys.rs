use super::*;

#[test]
fn test_fkeys_in_composing() {
    let mut engine = InputMethodEngine::new();

    // Type "あいう"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    assert_eq!(engine.preedit().unwrap().text(), "あいう");

    // F7 -> Katakana
    engine.process_key(&press_key(Keysym::F7));
    assert_eq!(engine.preedit().unwrap().text(), "アイウ");

    // F8 -> Half-width Katakana
    engine.process_key(&press_key(Keysym::F8));
    assert_eq!(engine.preedit().unwrap().text(), "ｱｲｳ");

    // F6 -> Hiragana
    engine.process_key(&press_key(Keysym::F6));
    assert_eq!(engine.preedit().unwrap().text(), "あいう");
}

#[test]
fn test_ctrl_uio_in_composing() {
    let mut engine = InputMethodEngine::new();

    // Type "あいう"
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));

    // Ctrl+I -> Katakana
    engine.process_key(&press_ctrl(Keysym::KEY_I));
    assert_eq!(engine.preedit().unwrap().text(), "アイウ");

    // Ctrl+O -> Half-width Katakana
    engine.process_key(&press_ctrl(Keysym::KEY_O));
    assert_eq!(engine.preedit().unwrap().text(), "ｱｲｳ");

    // Ctrl+U -> Hiragana
    engine.process_key(&press_ctrl(Keysym::KEY_U));
    assert_eq!(engine.preedit().unwrap().text(), "あいう");
}

#[test]
fn test_fkeys_in_conversion() {
    let mut engine = InputMethodEngine::new();

    // Type "あいう" and start conversion
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press('u'));
    engine.process_key(&press_key(Keysym::SPACE));
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    // F7 -> Katakana (cancels conversion and converts to katakana)
    engine.process_key(&press_key(Keysym::F7));
    assert!(matches!(engine.state(), InputState::Composing { .. }));
    assert_eq!(engine.preedit().unwrap().text(), "アイウ");
}

#[test]
fn test_fkeys_with_numbers() {
    let mut engine = InputMethodEngine::new();

    // Type "あ123"
    engine.process_key(&press('a'));
    engine.process_key(&press('1'));
    engine.process_key(&press('2'));
    engine.process_key(&press('3'));
    assert_eq!(engine.preedit().unwrap().text(), "あ123");

    // F7 -> Katakana (Hiragana "あ" becomes "ア", numbers remain)
    engine.process_key(&press_key(Keysym::F7));
    assert_eq!(engine.preedit().unwrap().text(), "ア123");

    // F8 -> Half-width Katakana
    engine.process_key(&press_key(Keysym::F8));
    assert_eq!(engine.preedit().unwrap().text(), "ｱ123");
}

#[test]
fn test_fkeys_with_live_conversion() {
    let mut engine = make_live_conversion_engine();

    // Type "あい" (multi-char to ensure live conversion triggers)
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    // Live conversion should be active
    assert!(!engine.live.text.is_empty());

    // F7 -> Katakana. This should clear live conversion text and show "アイ"
    engine.process_key(&press_key(Keysym::F7));
    assert!(engine.live.text.is_empty());
    assert!(engine.live.frozen);
    assert_eq!(engine.preedit().unwrap().text(), "アイ");
}

#[test]
fn test_fkey_commit_on_new_input() {
    let mut engine = InputMethodEngine::new();

    // Type "あ" and convert to Katakana with F7
    engine.process_key(&press('a'));
    engine.process_key(&press_key(Keysym::F7));
    assert_eq!(engine.preedit().unwrap().text(), "ア");
    assert!(engine.live.frozen);

    // Type "i" -> "ア" should be committed, and "い" should start as new input
    let result = engine.process_key(&press('i'));
    
    // Verify commit action is present
    let committed = result.actions.iter().any(|a| matches!(a, EngineAction::Commit(text) if text == "ア"));
    assert!(committed, "Should commit 'ア'");

    // Verify state is back to Composing with "い"
    assert_eq!(engine.preedit().unwrap().text(), "い");
    assert!(!engine.live.frozen, "Frozen state should be reset");
}


