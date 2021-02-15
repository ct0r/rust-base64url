use base64url::{escape, unescape};

#[test]
fn escape_replaces_symbols() {
    let val = escape("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");

    assert_eq!(
        val,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
    );
}

#[test]
fn escape_removes_padding() {
    assert_eq!(escape("AA=="), "AA");
    assert_eq!(escape("AAA="), "AAA");
    assert_eq!(escape("AAAA"), "AAAA");
}

#[test]
fn unescape_replaces_symbols() {
    let val = unescape("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");

    assert_eq!(
        val,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    );
}

#[test]
fn unescape_adds_padding() {
    assert_eq!(unescape("AA"), "AA==");
    assert_eq!(unescape("AAA"), "AAA=");
    assert_eq!(unescape("AAAA"), "AAAA");
}
