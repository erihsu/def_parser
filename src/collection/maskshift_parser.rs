// nom
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;

// def
use super::base::{tstring, ws};

pub fn maskshift_section(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(tag("COMPONENTMASKSHIFT"), many1(tstring), ws(tag(";")))(input)
}
