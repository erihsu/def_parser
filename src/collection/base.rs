use nom::branch::alt;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of};
use nom::combinator::{map_res, opt, recognize};
use nom::error::ParseError;
use nom::multi::{many0, many1};

use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::str;
use std::str::FromStr;

// basic parse. Independent from def_parser but it's the most basic parser in def_parser.

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// // typical string
// // ie. abcdef, de234, jkl_mn, ...
pub fn tstring(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(input)
}

// // allow tstring preceded with number
pub fn itstring(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"), digit1)),
        many0(alt((alphanumeric1, tag("_")))),
    )))(input)
}

// // parse string that is surrounded by " and ".
// // ie, "abc", "def"
pub fn qstring(input: &str) -> IResult<&str, &str> {
    ws(recognize(delimited(tag("\""), tstring, tag("\""))))(input)
}

// use for component pattern recognize
pub fn component_pattern(input: &str) -> IResult<&str, &str> {
    ws(recognize(tuple((
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
        opt(tag("*")),
    ))))(input)
}

// // signed integer number
// // ie, 100, -20
pub fn number(input: &str) -> IResult<&str, i32> {
    ws(map_res(
        recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        |res: &str| i32::from_str(res),
    ))(input)
}

pub fn number_str(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)))(input)
}

// parse unsigned floating number
// The following is adapted from the Python parser by Valentin Lorentz (ProgVal).
pub fn float(input: &str) -> IResult<&str, f64> {
    ws(map_res(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((decimal, char('.'), opt(decimal)))),
        )),
        |res: &str| f64::from_str(res),
    ))(input)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

#[cfg(test)]
mod tests {
    use crate::collection::base::*;

    #[test]
    fn test_float() {
        assert_eq!(float("3.14").unwrap(), ("", 3.14));
        assert_eq!(float(" 3.14").unwrap(), ("", 3.14));
        assert_eq!(float(" 3.14 ").unwrap(), ("", 3.14));
    }
    #[test]
    fn test_tstring() {
        assert_eq!(tstring("abcd_edf").unwrap(), ("", "abcd_edf"));
        assert_eq!(tstring("ab23cd_edf ").unwrap(), ("", "ab23cd_edf"));
        assert_eq!(tstring(" ab23cd_edf").unwrap(), ("", "ab23cd_edf"));
    }
    #[test]
    fn test_qstring() {
        assert_eq!(qstring("\"abcd_edf\"").unwrap(), ("", "\"abcd_edf\""));
    }

    #[test]
    fn test_number() {
        assert_eq!(number(" 123").unwrap(), ("", 123));
        assert_eq!(number("-23 ").unwrap(), ("", -23));
    }
}
