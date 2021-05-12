use dec2flt::parse;

#[test]
pub fn test_parse() {
    assert_eq!(parse::<f64>("1.2345e22").unwrap(), 1.2345e22);
    assert_eq!(parse::<f64>("1.2345e30").unwrap(), 1.2345e30);
    assert_eq!(parse::<f64>("9007199254740992.0").unwrap(), 9007199254740992.0);
    assert_eq!(parse::<f64>("9007199254740993.0").unwrap(), 9007199254740993.0);
    assert_eq!(parse::<f64>("8.988465674311580536566680e307").unwrap(), 8.988465674311580536566680e307);
    assert_eq!(parse::<f64>("8.442911973260991817129021e-309").unwrap(), 8.442911973260991817129021e-309);
}
