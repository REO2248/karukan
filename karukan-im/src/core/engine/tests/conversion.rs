use super::*;

#[test]
fn test_conversion_char_commits_and_continues() {
    let mut engine = InputMethodEngine::new();

    // Type "あい" and enter conversion
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press_key(Keysym::SPACE));
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    // Type 'k' during conversion → should commit candidate and start new input
    let result = engine.process_key(&press('k'));
    assert!(result.consumed);

    // Should have committed the conversion
    let has_commit = result
        .actions
        .iter()
        .any(|a| matches!(a, EngineAction::Commit(_)));
    assert!(has_commit, "Should have a commit action");

    // Should now be in Composing with 'k' in preedit
    assert!(matches!(engine.state(), InputState::Composing { .. }));
    assert_eq!(engine.preedit().unwrap().text(), "k");
}

#[test]
fn test_conversion_char_commits_and_continues_romaji() {
    let mut engine = InputMethodEngine::new();

    // Type "あ" and enter conversion
    engine.process_key(&press('a'));
    engine.process_key(&press_key(Keysym::SPACE));
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    // Type 'k', 'a' → commits conversion, then starts "か"
    engine.process_key(&press('k'));
    assert!(matches!(engine.state(), InputState::Composing { .. }));
    assert_eq!(engine.preedit().unwrap().text(), "k");

    engine.process_key(&press('a'));
    assert_eq!(engine.preedit().unwrap().text(), "か");
}

#[test]
fn test_shift_tab_navigation() {
    let mut engine = InputMethodEngine::new();

    // Type "あ" and enter conversion
    engine.process_key(&press('a'));
    engine.process_key(&press_key(Keysym::SPACE));

    // Assume we have at least 2 candidates.
    // Initial state: selected index 0.

    // Press Tab -> next candidate (index 1)
    engine.process_key(&press_key(Keysym::TAB));
    let index1 = match engine.state() {
        InputState::Conversion { candidates, .. } => candidates.cursor(),
        _ => panic!("Not in conversion state"),
    };
    assert_eq!(index1, 1);

    // Press Shift+Tab -> previous candidate (index 0)
    let shift_tab = KeyEvent::new(Keysym::TAB, KeyModifiers::new().with_shift(true), true);
    engine.process_key(&shift_tab);
    let index0 = match engine.state() {
        InputState::Conversion { candidates, .. } => candidates.cursor(),
        _ => panic!("Not in conversion state"),
    };
    assert_eq!(index0, 0);
}


