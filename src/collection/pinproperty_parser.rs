// nom
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

// def
use super::base::{number, tstring, ws};
use super::common::properties;
use super::def_types::Pinprop;

pub fn pinproperty_section(input: &str) -> IResult<&str, (i32, Vec<Pinprop>)> {
    delimited(
        tag("PINPROPERTIES"),
        tuple((terminated(number, ws(tag(";"))), many0(pinproperty_member))),
        tag("END PINPROPERTIES"),
    )(input)
}

fn pinproperty_member(input: &str) -> IResult<&str, Pinprop> {
    delimited(
        tag("-"),
        tuple((
            map(tstring, |res: &str| match res {
                "PIN" => None,
                s => Some(s),
            }),
            tstring,
            properties,
        )),
        ws(tag(";")),
    )(input)
}

// #[cfg(test)]
// mod tests {
//     use super::def_types::*;
//     use super::pinproperty_parser::*;
//     use std::io::Read;

//     #[test]
//     fn test_pinprop_section() {
//         let mut input_def = std::fs::File::open("tests/pinprop_test.def").unwrap();
//         let mut data = String::new();
//         input_def.read_to_string(&mut data).unwrap();
//         let result = pinproperty_section(&data).unwrap();

//         let pinproperty_section = result.1;

//         let num = pinproperty_section.0;
//         let pinprops = pinproperty_section.1;

//         assert_eq!(num, 2);
//         assert_eq!(
//             pinprops,
//             vec![
//                 (
//                     None,
//                     "P0",
//                     vec![
//                         ("strprop", PropValue::SValue("\"aString\"")),
//                         ("intprop", PropValue::IValue(1)),
//                         ("realprop", PropValue::RValue(1.1)),
//                         ("intrangeprop", PropValue::IValue(25)),
//                         ("realrangeprop", PropValue::RValue(25.25))
//                     ]
//                 ),
//                 (
//                     Some("I1"),
//                     "A",
//                     vec![
//                         ("strprop", PropValue::SValue("\"aString\"")),
//                         ("intprop", PropValue::IValue(1)),
//                         ("realprop", PropValue::RValue(1.1)),
//                         ("intrangeprop", PropValue::IValue(25)),
//                         ("realrangeprop", PropValue::RValue(25.25))
//                     ]
//                 )
//             ]
//         );
//     }
// }
