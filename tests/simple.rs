use dec2flt::parse;

#[test]
pub fn test_parse() {
    assert_eq!(parse::<f64>("1.2345e22").unwrap(), 1.2345e22);
    assert_eq!(parse::<f64>("1.2345e30").unwrap(), 1.2345e30);
    assert_eq!(parse::<f64>("9007199254740992.0").unwrap(), 9007199254740992.0);
    assert_eq!(parse::<f64>("9007199254740993.0").unwrap(), 9007199254740993.0);
    assert_eq!(parse::<f64>("8.988465674311580536566680e307").unwrap(), 8.988465674311580536566680e307);
    assert_eq!(parse::<f64>("8.442911973260991817129021e-309").unwrap(), 8.442911973260991817129021e-309);

    // Handle largest subnormal float.
    assert_eq!(parse::<f64>("2.225073858507201e-308").unwrap(), 2.225073858507201e-308);

    // Handle smallest normal float.
    assert_eq!(parse::<f64>("2.2250738585072014e-308").unwrap(), 2.2250738585072014e-308);

    // Handle largest normal float.
    assert_eq!(parse::<f64>("1.7976931348623157e+308").unwrap(), 1.7976931348623157e+308);

    // Handle literal infinities, but those seemingly well-handled.
    assert_eq!(parse::<f64>("1.8e+308").unwrap(), f64::INFINITY);

    // Handle debugging issues from huge-pow10 stalling.
    assert_eq!(parse::<f64>("18293e304").unwrap(), f64::INFINITY);
    assert_eq!(parse::<f64>("18294e304").unwrap(), f64::INFINITY);

    // Handle errors from rand-f64.
    assert_eq!(parse::<f64>("1931716584395492135994702176489828092381050353408077242878204528120199139939481194323196451544219842047651625146822916259264650699531105941226839770229443539950385549852181010472089599837804959638240722480117275160194670960071081546721577526042972609436090537318014304143692826510933").unwrap(), 1.9317165843954923e+282);
    assert_eq!(parse::<f32>("2.973339841440518e-46").unwrap(), 2.973339841440518e-46);
    assert_eq!(parse::<f32>("1.0511814862426301e-45").unwrap(), 1e-45);
    assert_eq!(parse::<f32>("7.015488112949493e-46").unwrap(), 1e-45);
    assert_eq!(parse::<f32>("2.47032822920623272e-324").unwrap(), 0.0);
    assert_eq!(parse::<f64>("2.47032822920623272e-324").unwrap(), 0.0);
    assert_eq!(parse::<f64>("2.470328229206232721e-324").unwrap(), 5e-324);
}
