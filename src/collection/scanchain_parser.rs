// nom
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::def_types::ScanChain;

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
