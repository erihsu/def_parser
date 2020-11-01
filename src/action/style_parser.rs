use crate::action::action_types::Pts;
use crate::action::common_parse::{number, pt_list_new, ws};

use nom::bytes::complete::tag;

use nom::multi::many1;

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn styles_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // total number of styles
        Vec<(
            i32, // styleNum
            Pts, //
        )>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("STYLES")), number, ws(tag(";"))),
            many1(delimited(ws(tag("-")), style_member, ws(tag(";")))),
        )),
        ws(tag("END STYLES")),
    )(input)
}

fn style_member(input: &str) -> IResult<&str, (i32, Pts)> {
    tuple((preceded(ws(tag("STYLE")), number), pt_list_new))(input)
}
