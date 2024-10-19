use core::str;

use saphyr_parser::{Event, Parser, ScanError};

/// Run the parser through the string.
///
/// The parser is run through both the `StrInput` and `BufferedInput` variants. The resulting
/// events are then compared and must match.
///
/// # Returns
/// This function returns the events if parsing succeeds, the error the parser returned otherwise.
///
/// # Panics
/// This function panics if there is a mismatch between the 2 parser invocations with the different
/// input traits.
fn run_parser(input: &str) -> Result<Vec<Event>, ScanError> {
    let mut str_events = vec![];
    let mut iter_events = vec![];

    for x in Parser::new_from_str(input) {
        str_events.push(x?.0);
    }
    for x in Parser::new_from_iter(input.chars()) {
        iter_events.push(x?.0);
    }

    assert_eq!(str_events, iter_events);

    Ok(str_events)
}

#[test]
fn fuzz_1() {
    // Crashing with an index out-of-bounds error.
    // In `scan_plain_scalar`, we would lookahead 1 and call `skip_break`, which requires a
    // lookahead of 2.
    let raw_input: &[u8] = &[
        1, 39, 110, 117, 108, 108, 34, 13, 13, 13, 13, 13, 10, 13, 13, 13, 13,
    ];
    let s = str::from_utf8(raw_input).unwrap();
    let _ = run_parser(s);
}

#[test]
fn fuzz_2() {
    // Crashing with an unwrap of a None value.
    // There is an imbalance of implicit flow mapping contexts here between the opening `[`/`{` and
    // closing `]`/`}`. We would test against flow-level when only `[` can create implicit flow
    // mappings.
    let raw_input: &[u8] = &[
        91, 91, 32, 101, 58, 9, 123, 63, 32, 45, 106, 101, 58, 9, 123, 63, 32, 44, 117, 101, 58, 9,
        123, 63, 32, 44, 9, 26, 58, 32, 126, 93, 8, 58, 32, 58, 10, 29, 58, 58, 58, 32, 58, 29, 63,
        32, 44, 9, 26, 58, 32, 126, 93, 8, 58, 32, 58, 10, 78, 32,
    ];
    let s = str::from_utf8(raw_input).unwrap();
    let _ = run_parser(s);
}
