use nom::branch::alt;

use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, space0};
use nom::combinator::{map, opt, recognize};

use nom::multi::{many0, many1};

use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::str;

use crate::def_parser::base::{float, number, number_str, qstring, tstring, ws};
use crate::def_parser::def_types::{Geometry, PropValue, Properties, RouteBody, RouteElem, RtPt};
// common parser used in def_parser. These parser are very commonly used in def_parser so collect them together.

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
pub fn rtpt(input: &str) -> IResult<&str, RtPt> {
    delimited(
        ws(tag("(")),
        tuple((
            alt((map(ws(tag("*")), |_| None), map(number, |n| Some(n)))),
            alt((map(ws(tag("*")), |_| None), map(number, |n| Some(n)))),
            opt(number),
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
    many0(preceded(
        ws(tag("+ PROPERTY")),
        tuple((
            tstring,
            alt((
                map(tstring, |res: &str| PropValue::SValue(res)),
                map(qstring, |res: &str| PropValue::SValue(res)),
                map(float, |res: f64| PropValue::RValue(res)),
                map(number, |res: i32| PropValue::IValue(res)),
            )),
        )),
    ))(input)
}

pub fn rect_or_polygon(input: &str) -> IResult<&str, Geometry> {
    alt((
        map(
            preceded(tag("RECT"), rect),
            |res: ((i32, i32), (i32, i32))| Geometry::Rect(res),
        ),
        map(preceded(tag("POLYGON"), pt_list), |res: Vec<(i32, i32)>| {
            Geometry::Polygon(res)
        }),
    ))(input)
}

// Routing point
pub fn route_body(input: &str) -> IResult<&str, RouteBody> {
    many0(alt((
        map(pair(rtpt, tstring), |res| RouteElem::Via(res)),
        map(rtpt, |res| RouteElem::Pt(res)),
    )))(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::common::*;

    #[test]
    fn test_pt() {
        assert_eq!(pt("(200 200)").unwrap(), ("", ("200", "200")));
        assert_eq!(pt("( 200 200 )").unwrap(), ("", ("200", "200")));
        assert_eq!(pt("( 200 * )").unwrap(), ("", ("200", "*")));
        assert_eq!(pt("( * -200 )").unwrap(), ("", ("*", "-200")));
    }

    #[test]
    fn test_pt_list() {
        assert_eq!(
            pt_list(" (100 200) (200 400) (* 100) ;").unwrap(),
            (";", vec![(100, 200), (200, 400), (200, 100)])
        );
    }
    #[test]
    fn test_property() {
        assert_eq!(
            properties(
                "  + PROPERTY strprop \"aString\" 
  + PROPERTY intprop 1 
  + PROPERTY realprop 1.1 
  + PROPERTY intrangeprop 25
  + PROPERTY realrangeprop 25.25"
            )
            .unwrap(),
            (
                "",
                vec![
                    ("strprop", PropValue::SValue("\"aString\"")),
                    ("intprop", PropValue::IValue(1)),
                    ("realprop", PropValue::RValue(1.1)),
                    ("intrangeprop", PropValue::IValue(25)),
                    ("realrangeprop", PropValue::RValue(25.25))
                ]
            )
        );
    }

    #[test]
    fn test_route_body() {
        assert_eq!(
            route_body("( 14000 341440 ) ( 9600 * ) ( * 282400 ) M1_M2 ( 2400 * ) VIAGEN12_0")
                .unwrap(),
            (
                "",
                vec![
                    RouteElem::Pt((Some(14000), Some(341440), None)),
                    RouteElem::Pt((Some(9600), None, None)),
                    RouteElem::Via(((None, Some(282400), None), "M1_M2")),
                    RouteElem::Via(((Some(2400), None, None), "VIAGEN12_0")),
                ]
            )
        );
    }
}
