use nom::branch::alt;

use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of, space0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::error::ParseError;
use nom::multi::{many0, many1};

use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::str;
use std::str::FromStr;

use crate::action::action_types::{Geometry, Properties};

// // basic parse

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    move |i| delimited(multispace0, &inner, multispace0)(i)
}

// // typical string
// // ie. abcdef, de234, jkl_mn, ...
pub fn tstring(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(input)
}

// // parse string that is surrounded by " and ".
// // ie, "abc", "def"
pub fn qstring(input: &str) -> IResult<&str, &str> {
    ws(recognize(delimited(tag("\""), tstring, tag("\""))))(input)
}

// // signed integer number
// // ie, 100, -20
pub fn number(input: &str) -> IResult<&str, i32> {
    ws(map_res(
        recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        |res: &str| i32::from_str(res),
    ))(input)
}

fn number_str(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)))(input)
}

// parse orient
pub fn orient(input: &str) -> IResult<&str, i32> {
    ws(map_res(
        recognize(alt((
            tag("N"),
            tag("W"),
            tag("E"),
            tag("S"),
            tag("FN"),
            tag("FW"),
            tag("FS"),
            tag("FE"),
        ))),
        |res: &str| match res {
            "N" => Ok(0),
            "W" => Ok(1),
            "S" => Ok(2),
            "E" => Ok(3),
            "FN" => Ok(4),
            "FW" => Ok(5),
            "FS" => Ok(6),
            "FE" => Ok(7),
            _ => Err(0),
        },
    ))(input)
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

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn pt(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        ws(tag("(")),
        separated_pair(
            alt((tag("*"), number_str)),
            space0,
            alt((tag("*"), number_str)),
        ),
        ws(tag(")")),
    )(input)
}

pub fn pt_new(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        ws(tag("(")),
        separated_pair(number, space0, number),
        ws(tag(")")),
    )(input)
}

// routing pt
pub fn rtpt(input: &str) -> IResult<&str, (&str, &str, Option<&str>)> {
    delimited(
        ws(tag("(")),
        tuple((
            alt((ws(tag("*")), number_str)),
            alt((ws(tag("*")), number_str)),
            opt(number_str),
        )),
        ws(tag(")")),
    )(input)
}

pub fn rect(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    tuple((
        delimited(
            ws(tag("(")),
            separated_pair(number, space0, number),
            ws(tag(")")),
        ),
        delimited(
            ws(tag("(")),
            separated_pair(number, space0, number),
            ws(tag(")")),
        ),
    ))(input)
}

// pub fn pt_list(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
//     many1(pt)(input)
// }

pub fn pt_list(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    map(many1(pt), |res: Vec<(&str, &str)>| {
        let mut out = Vec::new();
        let mut prev_x = 0;
        let mut prev_y = 0;
        for (pt_x, pt_y) in res {
            prev_x = match pt_x.parse::<i32>() {
                Ok(n) => n,
                Err(_) => prev_x,
            };
            prev_y = match pt_y.parse::<i32>() {
                Ok(n) => n,
                Err(_) => prev_y,
            };
            let a_pt = (prev_x, prev_y);
            out.push(a_pt);
        }
        out
    })(input)
}

pub fn rect_list(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    many1(rect)(input)
}

pub fn x_or_y(input: &str) -> IResult<&str, char> {
    alt((char('X'), char('Y')))(input)
}

pub fn on_or_off(input: &str) -> IResult<&str, &str> {
    alt((tag("ON"), tag("OFF")))(input)
}

pub fn inline_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("#"), is_not("\n"), tag("\n"))(input)
}

pub fn source_type(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(preceded(ws(tag("SOURCE")), tstring)),
        |res: &str| match res {
            "DIST" => Ok(0),
            "NETLIST" => Ok(1),
            "TIMING" => Ok(2),
            "USER" => Ok(3),
            _ => Err(0),
        },
    )(input)
}

// different from tstring and qstring, comp_name might contain hierachical struct and bus bit information
// ie, i1/i2[2]/i3.
// Only support use busbit_char = "[]", divider char = "/"
pub fn comp_name(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        tstring,
        many0(alt((
            recognize(many1(tuple((tag("["), number, tag("]"))))),
            recognize(pair(tag("/"), tstring)),
        ))),
    )))(input)
}

// Properties
pub fn properties(input: &str) -> IResult<&str, Properties> {
    many0(tuple((
        preceded(ws(tag("+ PROPERTY")), tstring),
        opt(alt((qstring, tstring))),
        opt(float),
    )))(input)
}

pub fn rect_or_polygon(input: &str) -> IResult<&str, Geometry> {
    alt((
        map(
            preceded(tag("RECT"), rect),
            |res: ((i32, i32), (i32, i32))| Geometry::RECT(res),
        ),
        map(preceded(tag("POLYGON"), pt_list), |res: Vec<(i32, i32)>| {
            Geometry::POLYGON(res)
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::action::common_parse::*;

    #[test]
    fn test_float() {
        assert_eq!(float("3.14").unwrap(), ("", 3.14));
        assert_eq!(float(" 3.14").unwrap(), ("", 3.14));
        assert_eq!(float(" 3.14 ").unwrap(), ("", 3.14));
    }
    #[test]
    fn test_pt() {
        assert_eq!(pt("(200 200)").unwrap(), ("", ("200", "200")));
        assert_eq!(pt("( 200 200 )").unwrap(), ("", ("200", "200")));
        assert_eq!(pt("( 200 * )").unwrap(), ("", ("200", "*")));
        assert_eq!(pt("( * -200 )").unwrap(), ("", ("*", "-200")));
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

    #[test]
    fn test_orient() {
        assert_eq!(orient(" N").unwrap(), ("", 0));
        assert_eq!(orient("FN ").unwrap(), ("", 4));
    }

    #[test]
    fn test_pt_list() {
        assert_eq!(
            pt_list(" (100 200) (200 400) (* 100) ;").unwrap(),
            (";", vec![(100, 200), (200, 400), (200, 100)])
        );
    }
}
