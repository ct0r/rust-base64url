# base64url

Encodes and decodes base64 strings to url friendly format mentioned in [RFC4648](https://tools.ietf.org/html/rfc4648).

## Example

```rust
use base64url::{escape, unescape};

fn main() {
    let a = "A+/=";
    let b = "A-_";

    assert_eq!(escape(a), b);
    assert_eq!(unescape(b), a);
}
```
