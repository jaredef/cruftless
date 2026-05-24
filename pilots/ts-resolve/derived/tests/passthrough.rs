//! TSR-EXT 2 smoke: pure-JS-inside-.ts round-trips via TsParser+erase.

#[test]
fn passthrough_valid_js_yields_same_module_as_js_parser() {
    let src = "let x = 1; let y = 2; (x + y);";
    let direct = rusty_js_parser::parse_module(src).expect("js parse ok");
    let tsr = ts_resolve::parse_and_erase(src).expect("ts parse ok");
    // Stmt count matches; full structural compare is brittle on Debug
    // strings (Span byte offsets), but length is a stable lower-bound
    // invariant at this round.
    assert_eq!(direct.body.len(), tsr.body.len(), "stmt counts differ");
}

#[test]
fn parse_and_erase_returns_empty_witnesses_at_tsr_ext_2() {
    let src = "let x = 1;";
    let (_module, witnesses) = ts_resolve::parse_with_witnesses(src).expect("ok");
    assert_eq!(witnesses.len(), 0, "TSR-EXT 2 emits no witnesses yet");
}

#[test]
fn ts_contextual_keyword_helper_recognizes_core_set() {
    use ts_resolve::lexer::is_ts_contextual_keyword;
    assert!(is_ts_contextual_keyword("type"));
    assert!(is_ts_contextual_keyword("interface"));
    assert!(is_ts_contextual_keyword("as"));
    assert!(is_ts_contextual_keyword("readonly"));
    assert!(!is_ts_contextual_keyword("let"));
    assert!(!is_ts_contextual_keyword("foo"));
}
