//! Tests for the learning cache and the Tab-skips-learning behavior.
//!
//! Space/Down: include learning candidates (default conversion).
//! Tab: skip learning candidates (lets users escape stale learned entries).

use karukan_engine::LearningCache;

use super::*;

/// Engine seeded with a learning entry `reading → surface`, no kanji model.
/// We bypass `init.rs` (which gates learning on settings + file I/O) and just
/// inject a populated `LearningCache` directly — these tests assert the
/// build_conversion_candidates branching, not the load path.
fn engine_with_learned(reading: &str, surface: &str) -> InputMethodEngine {
    let mut engine = InputMethodEngine::new();
    engine.converters.kanji = None;
    let mut cache = LearningCache::new(100);
    cache.record(reading, surface);
    engine.learning = Some(cache);
    engine
}

#[test]
fn build_candidates_includes_learning_when_not_skipped() {
    let mut engine = engine_with_learned("あい", "藍");

    let texts: Vec<String> = engine
        .build_conversion_candidates("あい", 9, false)
        .into_iter()
        .map(|c| c.text)
        .collect();

    assert!(
        texts.contains(&"藍".to_string()),
        "Space path (skip_learning=false) should surface learned `藍`, got {:?}",
        texts,
    );
}

#[test]
fn build_candidates_omits_learning_when_skipped() {
    let mut engine = engine_with_learned("あい", "藍");

    let texts: Vec<String> = engine
        .build_conversion_candidates("あい", 9, true)
        .into_iter()
        .map(|c| c.text)
        .collect();

    assert!(
        !texts.contains(&"藍".to_string()),
        "Tab path (skip_learning=true) must drop learned `藍`, got {:?}",
        texts,
    );
}

#[test]
fn tab_key_skips_learning_in_composing() {
    // End-to-end: type the reading, press Tab → learned candidate is gone.
    let mut engine = engine_with_learned("あい", "藍");

    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    assert_eq!(engine.input_buf.text, "あい");

    let result = engine.process_key(&press_key(Keysym::TAB));
    assert!(result.consumed);
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    let texts: Vec<String> = engine
        .state()
        .candidates()
        .unwrap()
        .candidates()
        .iter()
        .map(|c| c.text.clone())
        .collect();
    assert!(
        !texts.contains(&"藍".to_string()),
        "Tab must skip the learned `藍` candidate, got {:?}",
        texts,
    );
}

#[test]
fn space_key_keeps_learning_in_composing() {
    // Counterpart to tab_key_skips_learning_in_composing: Space stays on the
    // learning-included path so the default UX is unchanged.
    let mut engine = engine_with_learned("あい", "藍");

    engine.process_key(&press('a'));
    engine.process_key(&press('i'));

    let result = engine.process_key(&press_key(Keysym::SPACE));
    assert!(result.consumed);
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    let texts: Vec<String> = engine
        .state()
        .candidates()
        .unwrap()
        .candidates()
        .iter()
        .map(|c| c.text.clone())
        .collect();
    assert!(
        texts.contains(&"藍".to_string()),
        "Space must surface learned `藍`, got {:?}",
        texts,
    );
}

#[test]
fn ctrl_delete_removes_learning_entry() {
    // Type reading, press Space to enter conversion, then Ctrl+Delete to remove the learning entry.
    let mut engine = engine_with_learned("あい", "藍");

    engine.process_key(&press('a'));
    engine.process_key(&press('i'));

    // Enter conversion mode
    let result = engine.process_key(&press_key(Keysym::SPACE));
    assert!(result.consumed);
    assert!(matches!(engine.state(), InputState::Conversion { .. }));

    // Verify the learning candidate is present
    let candidates = engine.state().candidates().unwrap();
    let selected = candidates.selected().unwrap();
    assert_eq!(selected.text, "藍");
    assert_eq!(selected.source_label.as_deref(), Some("[履歴]"));

    // Press Ctrl+Delete to remove the learning entry
    let ctrl_delete = KeyEvent::new(Keysym::DELETE, KeyModifiers::new().with_control(true), true);
    let result = engine.process_key(&ctrl_delete);
    assert!(result.consumed);

    // Verify the learning entry is gone from the cache
    let cache = engine.learning.as_ref().unwrap();
    assert!(cache.lookup("あい").is_empty());

    // Verify the candidate list is rebuilt without the deleted entry
    let candidates = engine.state().candidates().unwrap();
    let texts: Vec<String> = candidates.candidates().iter().map(|c| c.text.clone()).collect();
    assert!(
        !texts.contains(&"藍".to_string()),
        "After Ctrl+Delete, `藍` should be gone from candidates, got {:?}",
        texts,
    );
}

#[test]
fn ctrl_delete_does_nothing_for_non_learning() {
    // Type reading, press Space, but the first candidate is NOT from learning.
    // Ctrl+Delete should be ignored.
    let mut engine = engine_with_learned("あい", "藍");

    // Add a dictionary candidate that will rank higher
    engine.converters.romaji.reset();
    engine.input_buf.clear();

    // Type something with no learning entry
    engine.process_key(&press('x'));
    engine.process_key(&press('y'));

    // Enter conversion mode
    let result = engine.process_key(&press_key(Keysym::SPACE));
    assert!(result.consumed);

    // The selected candidate should NOT be from learning
    let candidates = engine.state().candidates().unwrap();
    let selected = candidates.selected().unwrap();
    assert_ne!(selected.source_label.as_deref(), Some("[履歴]"));

    // Press Ctrl+Delete — should be ignored (not consumed)
    let ctrl_delete = KeyEvent::new(Keysym::DELETE, KeyModifiers::new().with_control(true), true);
    let result = engine.process_key(&ctrl_delete);
    assert!(!result.consumed);
}
