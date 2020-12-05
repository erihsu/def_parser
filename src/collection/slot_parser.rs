// nom
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use super::base::{number, tstring, ws};
use super::common::rect_or_polygon;
use super::def_types::Slot;

pub fn slot_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of slot
        Vec<Slot>,
    ),
> {
    delimited(
        tag("SLOTS"),
        tuple((terminated(number, ws(tag(";"))), many0(slot_member))),
        tag("END SLOTS"),
    )(input)
}

fn slot_member(input: &str) -> IResult<&str, Slot> {
    delimited(
        tag("-"),
        tuple((preceded(ws(tag("LAYER")), tstring), many0(rect_or_polygon))),
        ws(tag(";")),
    )(input)
}

// #[cfg(test)]
// mod tests {
//     use super::def_types::*;
//     use super::slot_parser::*;
//     use std::io::Read;

//     #[test]
//     fn test_slot_section() {
//         let mut input_def = std::fs::File::open("tests/slot_test.def").unwrap();
//         let mut data = String::new();
//         input_def.read_to_string(&mut data).unwrap();
//         let result = slot_section(&data).unwrap();

//         let slot_section = result.1;

//         let num = slot_section.0;
//         let slots = slot_section.1;

//         assert_eq!(num, 3);
//         assert_eq!(
//             slots,
//             vec![
//                 ("M1", vec![Geometry::Rect(((3, 3), (6, 8)))],),
//                 (
//                     "M2",
//                     vec![
//                         Geometry::Rect(((3, 3), (6, 8))),
//                         Geometry::Polygon(vec![
//                             (0, 0),
//                             (0, 10),
//                             (10, 10),
//                             (10, 20),
//                             (20, 20),
//                             (20, 0)
//                         ])
//                     ],
//                 ),
//                 ("M3", vec![Geometry::Rect(((3, 3), (6, 8)))])
//             ]
//         );
//     }
// }
