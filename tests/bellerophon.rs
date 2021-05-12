use dec2flt::dec2flt;

#[test]
pub fn test_bellerophon() {
    // Check simple
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992, 0, false));
    assert_eq!((9007199254740992f64, false), dec2flt::bellerophon(9007199254740993, 0, false));
    assert_eq!((9007199254740994f64, true), dec2flt::bellerophon(9007199254740994, 0, false));
    assert_eq!((9007199254740996f64, false), dec2flt::bellerophon(9007199254740995, 0, false));
    assert_eq!((9007199254740996f64, true), dec2flt::bellerophon(9007199254740996, 0, false));

    // Check scaled up without truncation.
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992000, -3, false));
    assert_eq!((9007199254740992f64, false), dec2flt::bellerophon(9007199254740993000, -3, false));
    assert_eq!((9007199254740994f64, true), dec2flt::bellerophon(9007199254740994000, -3, false));
    assert_eq!((9007199254740996f64, false), dec2flt::bellerophon(9007199254740995000, -3, false));
    assert_eq!((9007199254740996f64, true), dec2flt::bellerophon(9007199254740996000, -3, false));

    // Test narrower bounds.
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992900, -3, false));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992950, -3, false));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992975, -3, false));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992990, -3, false));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992995, -3, false));
    assert_eq!((9007199254740992f64, false), dec2flt::bellerophon(9007199254740992996, -3, false));
    assert_eq!((9007199254740994f64, false), dec2flt::bellerophon(9007199254740993004, -3, false));
    assert_eq!((9007199254740994f64, true), dec2flt::bellerophon(9007199254740993005, -3, false));

    // Check scaled up with truncation.
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992000, -3, true));
    assert_eq!((9007199254740992f64, false), dec2flt::bellerophon(9007199254740993000, -3, true));
    assert_eq!((9007199254740994f64, true), dec2flt::bellerophon(9007199254740994000, -3, true));
    assert_eq!((9007199254740996f64, false), dec2flt::bellerophon(9007199254740995000, -3, true));
    assert_eq!((9007199254740996f64, true), dec2flt::bellerophon(9007199254740996000, -3, true));

    // Test these narrower bounds are much looser.
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992900, -3, true));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992950, -3, true));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992975, -3, true));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992990, -3, true));
    assert_eq!((9007199254740992f64, true), dec2flt::bellerophon(9007199254740992991, -3, true));
    assert_eq!((9007199254740992f64, false), dec2flt::bellerophon(9007199254740992992, -3, true));
    assert_eq!((9007199254740994f64, false), dec2flt::bellerophon(9007199254740993009, -3, true));
    assert_eq!((9007199254740994f64, true), dec2flt::bellerophon(9007199254740993010, -3, true));
}
