#![allow(deprecated)]
#[macro_use]
extern crate nom;
use nom::character::complete::{char, digit1 as digit};

use std::str;
use std::str::FromStr;

named!(
    uint32<i32>,
    map_res!(
        map_res!(recognize!(digit), str::from_utf8),
        FromStr::from_str
    )
);
named!(
    int32<i32>,
    map!(pair!(opt!(tag!("-")), uint32), |(sign, value): (
        Option<&[u8]>,
        i32
    )| sign
        .and_then(|s| if s[0] == b'-' { Some(-1i32) } else { None })
        .unwrap_or(1i32)
        * value)
);
named!(
    point<(i32, i32)>,
    delimited!(char('('), ws!(tuple!(int32, int32)), char(')'))
);
// named!(int32<&str>, delimited!(opt!(tag!("-")), digit));
// named!(
//     int32<i32>,
//     map_res!(map_res!(delimited!(opt!(tag!("-")), digit), str::from_utf8),FromStr::from_str));

// #[test]
// fn i32_test() {
//     assert_eq!(point(&b"(10 10)"[..]), Ok((&b""[..], (10, 10))));
//     assert_eq!(point(&b"(-10 200)"[..]), Ok((&b""[..], (-10, 200))));
// }
