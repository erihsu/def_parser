use nom::branch::alt;

use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, space0};
use nom::combinator::{map, map_res, opt, recognize};

use nom::multi::{many0, many1};

use nom::branch::permutation;

use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::str;

use crate::def_parser::base::{float, number, number_str, qstring, tstring, ws};
use crate::def_parser::def_types::{Geometry, NetProperty, Properties, RoutingPoint};

// common parser used in def_parser. These parser are very commonly used in def_parser so collect them together.

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
            |res: ((i32, i32), (i32, i32))| Geometry::Rect(res),
        ),
        map(preceded(tag("POLYGON"), pt_list), |res: Vec<(i32, i32)>| {
            Geometry::Polygon(res)
        }),
    ))(input)
}

// property that commonly used in net and snet
pub fn net_property(input: &str) -> IResult<&str, NetProperty> {
    permutation((
        map(
            preceded(ws(tag("+ SOURCE")), tstring),
            |res: &str| match res {
                "DIST" => Some(0),
                "NETLIST" => Some(1),
                "TEST" => Some(2),
                "TIMING" => Some(3),
                "USER" => Some(4),
                _ => None,
            },
        ),
        map(opt(ws(tag("+ FIXEDBUMP"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ ORIGINAL")), tstring)),
        map(preceded(ws(tag("+ USE")), tstring), |res: &str| match res {
            "ANALOG" => Some(0),
            "CLOCK" => Some(1),
            "GROUND" => Some(2),
            "POWER" => Some(3),
            "RESET" => Some(4),
            "SCAN" => Some(5),
            "SIGNAL" => Some(6),
            "TIEOFF" => Some(7),
            _ => None,
        }),
        map(
            preceded(ws(tag("+ PATTERN")), tstring),
            |res: &str| match res {
                "BALANCED" => Some(0),
                "STEINER" => Some(1),
                "TRUNK" => Some(2),
                "WIREDLOGIC" => Some(3),
                _ => None,
            },
        ),
        opt(preceded(ws(tag("+ ESTCAP")), number)),
        opt(preceded(ws(tag("+ WEIGHT")), number)),
        properties,
    ))(input)
}

// Routing point
pub fn routing_point(input: &str) -> IResult<&str, Vec<RoutingPoint>> {
    many0(alt((
        map(
            delimited(
                ws(tag("(")),
                tuple((number, number, opt(number))),
                ws(tag(")")),
            ),
            |res: (i32, i32, Option<i32>)| RoutingPoint::Pt(res),
        ),
        map(pair(tstring, opt(orient)), |res: (&str, Option<i32>)| {
            RoutingPoint::Via(res)
        }),
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
