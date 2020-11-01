use crate::action::action_types::Fill;
use crate::action::common_parse::{number, pt_list_new, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many1;

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn fill_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of LAYERS
        Vec<Fill>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("FILLS")), number, ws(tag(";"))),
            many1(preceded(ws(tag("-")), layer_or_via_fill)),
        )),
        ws(tag("END FILLS")),
    )(input)
}

fn layer_or_via_fill(input: &str) -> IResult<&str, Fill> {
    tuple((
        map(alt((tag("LAYER"), tag("VIA"))), |res: &str| match res {
            "LAYER" => 0,
            "VIA" => 1,
            _ => 2,
        }),
        tstring,
        opt(preceded(ws(tag("+ MASK")), number)),
        map(opt(ws(tag("+ OPC"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        delimited(
            alt((ws(tag("RECT")), ws(tag("POLYGON")), ws(tag("")))),
            pt_list_new,
            ws(tag(";")),
        ),
    ))(input)
}
