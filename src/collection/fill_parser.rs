// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use super::base::{number, tstring, ws};
use super::common::{pt_list, rect_or_polygon};
use super::def_types::Fill;

pub fn fill_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of LAYERS
        Vec<Fill>,
    ),
> {
    delimited(
        tag("FILLS"),
        tuple((terminated(number, ws(tag(";"))), many0(fill_member))),
        ws(tag("END FILLS")),
    )(input)
}

fn fill_member(input: &str) -> IResult<&str, Fill> {
    delimited(
        tag("-"),
        alt((
            map(
                tuple((
                    preceded(ws(tag("LAYER")), tstring),
                    map(opt(ws(tag("+ OPC"))), |res: Option<&str>| match res {
                        Some(_) => true,
                        None => false,
                    }),
                    many0(rect_or_polygon),
                )),
                |res| Fill::Layer(res),
            ),
            map(
                tuple((
                    preceded(ws(tag("VIA")), tstring),
                    map(opt(ws(tag("+ OPC"))), |res: Option<&str>| match res {
                        Some(_) => true,
                        None => false,
                    }),
                    pt_list,
                )),
                |res| Fill::Via(res),
            ),
        )),
        ws(tag(";")),
    )(input)
}

// #[cfg(test)]
// mod tests {

//     use super::fill_parser::*;
//     use std::io::Read;

//     #[test]
//     fn test_fill_section() {
//         let mut input_def = std::fs::File::open("tests/fill_test.def").unwrap();
//         let mut data = String::new();
//         input_def.read_to_string(&mut data).unwrap();
//         let result = fill_section(&data).unwrap();

//         let fill_section = result.1;

//         let num = fill_section.0;
//         let fills = fill_section.1;

//         assert_eq!(num, 5);
//         // assert_eq!(
//         //     fills,
//         //     vec![
//         //         Fill::Via(("myvia1", Some(2), true, vec![(5000, 5000), (800, 800)])),
//         //         Fill::Layer((
//         //             "M1",
//         //             Some(2),
//         //             false,
//         //             vec![Geometry::Rect(((0, 2), (1, 10)))]
//         //         )),
//         //         Fill::Layer((
//         //             "M2",
//         //             None,
//         //             true,
//         //             vec![
//         //                 Geometry::Rect(((0, 2), (1, 10))),
//         //                 Geometry::Polygon(vec![
//         //                     (0, 0),
//         //                     (0, 10),
//         //                     (10, 10),
//         //                     (10, 20),
//         //                     (20, 20),
//         //                     (20, 0)
//         //                 ])
//         //             ]
//         //         )),
//         //         Fill::Layer(("M3", None, false, vec![Geometry::Rect(((0, 2), (1, 10)))])),
//         //         Fill::Via(("M1_M2", Some(202), true, vec![(2400, 0), (10, 10)])),
//         //         Fill::Via(("VIAGEN12_0", None, true, vec![(100, 100), (200, 100)])),
//         //     ]
//         // );
//     }
// }
