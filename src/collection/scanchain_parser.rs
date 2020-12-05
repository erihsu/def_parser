// nom
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use super::base::{number, tstring, ws};
use super::def_types::ScanChain;

pub fn scanchain_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of scanchain
        Vec<ScanChain>,
    ),
> {
    delimited(
        tag("SCANCHAINS"),
        tuple((terminated(number, ws(tag(";"))), many0(scanchain_member))),
        ws(tag("END SCANCHAINS")),
    )(input)
}

fn scanchain_member(input: &str) -> IResult<&str, ScanChain> {
    delimited(
        tag("-"),
        tuple((
            tstring, // name
            opt(preceded(
                ws(tag("+ PARTITION")),
                tuple((tstring, opt(preceded(tag("MAXBITS"), number)))),
            )),
            opt(preceded(
                ws(tag("+ COMMONSCANPINS")),
                tuple((
                    delimited(ws(tag("(")), preceded(tag("IN"), tstring), ws(tag(")"))),
                    delimited(ws(tag("(")), preceded(tag("OUT"), tstring), ws(tag(")"))),
                )),
            )), // commonscanpin
            opt(preceded(
                ws(tag("+ START")),
                tuple((
                    map(tstring, |n| match n {
                        "PIN" => None,
                        _ => Some(n),
                    }),
                    tstring,
                )),
            )), // start
            opt(preceded(
                ws(tag("+ FLOATING")),
                tuple((
                    tstring,
                    delimited(ws(tag("(")), preceded(tag("IN"), tstring), ws(tag(")"))),
                    delimited(ws(tag("(")), preceded(tag("OUT"), tstring), ws(tag(")"))),
                    opt(delimited(
                        ws(tag("(")),
                        preceded(tag("BITS"), number),
                        ws(tag(")")),
                    )),
                )),
            )), // floating
            opt(preceded(
                ws(tag("+ ORDERED")),
                tuple((
                    tstring,
                    delimited(ws(tag("(")), preceded(tag("IN"), tstring), ws(tag(")"))),
                    delimited(ws(tag("(")), preceded(tag("OUT"), tstring), ws(tag(")"))),
                    opt(delimited(
                        ws(tag("(")),
                        preceded(tag("BITS"), number),
                        ws(tag(")")),
                    )),
                )),
            )), // ordered
            opt(preceded(
                ws(tag("+ STOP")),
                tuple((
                    map(tstring, |n| match n {
                        "PIN" => None,
                        _ => Some(n),
                    }),
                    tstring,
                )),
            )), // stop
        )),
        ws(tag(";")),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::collection::scanchain_parser::*;
    use std::io::Read;

    #[test]
    fn test_scanchain_section() {
        let mut input_def = std::fs::File::open("tests/scanchain_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = scanchain_section(&data).unwrap();

        let scanchain_section = result.1;

        let num = scanchain_section.0;
        let ndrs = scanchain_section.1;
    }
}
