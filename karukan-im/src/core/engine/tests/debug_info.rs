use super::*;

#[test]
fn test_toggle_debug_info() {
    let mut engine = InputMethodEngine::new();
    assert!(!engine.show_debug_info);

    // Initial aux text (debug info OFF)
    engine.process_key(&press('a'));
    let aux_off = engine.format_aux_composing();
    assert!(!aux_off.contains("jinen")); // Should not contain model name

    // Toggle ON
    engine.process_key(&press_ctrl_shift(Keysym::KEY_D));
    assert!(engine.show_debug_info);
    let aux_on = engine.format_aux_composing();
    assert!(aux_on.contains("Karukan (")); // Should contain model name or at least the format

    // Toggle OFF again
    engine.process_key(&press_ctrl_shift(Keysym::KEY_D));
    assert!(!engine.show_debug_info);
}

#[test]
fn test_debug_info_in_conversion() {
    let mut engine = InputMethodEngine::new();
    
    // Type "あ" and enter conversion
    engine.process_key(&press('a'));
    engine.process_key(&press_key(Keysym::SPACE));
    
    // Debug info OFF by default
    let aux_off = match &engine.state {
        InputState::Conversion { candidates, .. } => {
            let reading = engine.input_buf.text.clone();
            engine.format_aux_conversion_with_page(&reading, Some(candidates))
        }
        _ => panic!("Not in conversion state"),
    };
    assert!(!aux_off.contains("ms/"));

    // Toggle ON
    engine.toggle_debug_info();
    let aux_on = match &engine.state {
        InputState::Conversion { candidates, .. } => {
            let reading = engine.input_buf.text.clone();
            engine.format_aux_conversion_with_page(&reading, Some(candidates))
        }
        _ => panic!("Not in conversion state"),
    };
    assert!(aux_on.contains("ms/"));
}
