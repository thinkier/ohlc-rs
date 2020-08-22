use super::*;

#[test]
fn duration_test() {
    assert_eq!(duration_string(1), "Now");
    assert_eq!(duration_string(10), "10s");
    assert_eq!(duration_string(60), "1m");
    assert_eq!(duration_string(61), "1m1s");
    assert_eq!(duration_string(3600), "1h");
    assert_eq!(duration_string(3661), "1h1m1s");
    assert_eq!(duration_string(86400), "1d");
    assert_eq!(duration_string(86400 + 3661), "1d1h1m1s");
    assert_eq!(duration_string(604800), "1w");
    assert_eq!(duration_string(604800 + 86400 + 3661), "1w1d1h1m1s");
    assert_eq!(duration_string(2592000), "1m");
    assert_eq!(duration_string(2592000 + 604800 + 86400 + 3661), "1m1w1d1h1m1s");
    assert_eq!(duration_string(365 * 86400), "1y");
    assert_eq!(duration_string(365 * 86400 + 2592000 + 604800 + 86400 + 3661), "1y1m1w1d1h1m1s");
}
