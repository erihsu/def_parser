// nom
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple, pair};
use nom::IResult;
// def
use crate::def_parser::base::{float, number, qstring, tstring, ws};
use crate::def_parser::common::pt_list;
use crate::def_parser::def_types::Via;

pub fn via_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // viaNum
        Vec<Via>,
    ),
> {
    delimited(
        tag("VIAS"),
        tuple((
            terminated(number, ws(tag(";"))), // numVia
            many0(via_member),
        )),
        tag("END VIAS"),
    )(input)
}

fn via_member(input: &str) -> IResult<&str, Via> {
    delimited(
        tag("-"),
        pair(//Gets an object from the first parser, then gets another object from the second parser.
            tstring,
            permutation((
                opt(preceded(ws(tag("+ VIARULE")), tstring)),
                opt(preceded(ws(tag("+ CUTSIZE")), tuple((number, number)))),
                opt(preceded(
                    ws(tag("+ LAYERS")),
                    tuple((tstring, tstring, tstring)),
                )),
                opt(preceded(ws(tag("+ CUTSPACING")), tuple((number, number)))),
                opt(preceded(
                    ws(tag("+ ENCLOSURE")),
                    tuple((number, number, number, number)),
                )),
                opt(preceded(ws(tag("+ ROWCOL")), tuple((number, number)))),
                opt(preceded(ws(tag("+ ORIGIN")), tuple((number, number)))),
                opt(preceded(ws(tag("+ OFFSET")), tuple((number, number, number, number)))),
                opt(preceded(ws(tag("+ PATTERN")), tstring)),
                many0(tuple((
                    preceded(alt((ws(tag("+ RECT")), ws(tag("+ POLYGON")))), tstring),
                    opt(preceded(ws(tag("+ MASK")), number)),
                    pt_list,
                ))),
            ))),
            ws(tag(";")),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::via_parser::*;
    use std::io::Read;

    #[test]
    fn test_via_section() {
        let mut input_def = std::fs::File::open("tests/via_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = via_section(&data).unwrap();
        let via_section = result.1;
        let num = via_section.0;
        let vias = via_section.1;
        assert_eq!(num, 6);
        assert_eq!(
            vias,
            vec![
                    ("VIAGEN12_0", 
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , None, vec![(-4400, -3800), (4400, 3800)]),
                                 ("M2" , Some(3), vec![(4500 , -3800), (4500 , 3800)]),
                                 ("V1" , Some(2), vec![(-3600 , -3800), (-2000 , -2200)]),
                                 ("V1" ,Some(1) ,vec![(-3600 , 2200), (-2000 , 3800)]),
                                 ("V1" ,Some(2) ,vec![(2000 , -3800), (3600 , -2200)]),
                                 ("V1" ,Some(3) ,vec![(2000 , 2200), (3600 , 3800)]),
                                ]
                        )
                    ),
                    ("VIAGEN12_2",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , None, vec![(-2500, -1500), (2500, 1500)]),
                                 ("M2" , None, vec![(-2500 , -1500), (2500 , 1500)]),
                                 ("V1" , None, vec![(-2360 , -960), (-760 , 640)]),
                                 ("V1" , None, vec![(-1320 , -960), (280 , 640)]),
                                 ("V1" , None, vec![(760 , -960), (2360 , 640)]),
                                ]
                        )
                    ),
                    ("VIAGEN12_3",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , None, vec![(-1600, -1600), (1600, 1600)]),
                                 ("M2" , None, vec![(-1600 , -1600), (1600 , 1600)]),
                                 ("V1" , None, vec![(-800 , -800), (800 , 800)]),
                                ]
                        )
                    ),
                    ("VIAGEN12_4",
                        (Some("VIAGEN12"),//VIARULE
                         Some((1600,1600)),//CUTSIZE
                         Some(("M1", "V1", "M2")), //LAYERS
                         Some((5600,6100)), //CUTSPACING
                         Some((100,100,150,150)), //ENCLOSURE
                         Some((5,14)),//ROWCOL             
                         Some((10,-10)), //ORIGIN
                         Some((0,0,20,-20)), //OFFSET
                         Some("2_FFE0_3_FFFF"), //PATTERN
                            vec![("" , None, vec![]),
                                ]
                        )
                    ),
                    ("M2_M1rct_0",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("V1" , None, vec![(-25 , -65), (25 , 65)]),
                                 ("M1" , None, vec![(-35 , -95), (35 , 95)]),
                                 ("M2" , None, vec![(-65 , -65), (65 , 65)]),
                                ]
                        )
                    ),
                    ("VIAGEN12_1",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , Some(2), vec![(-2500,-1500), (2500,2500),(1500,2500),(1500,1500),(2500,1500),(2500,-1500)]),
                                 ("M2" , None, vec![(-2500,-1500), (2500,1500)]),
                                 ("M2" , None, vec![(-2400,-960), (-700,640)]),
                                ]
                        )
                    ),
                    ("CUSTOMVIA",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , Some(3), vec![(-2500,-1500),
                                                        (-2500,2500),
                                                        (1500,2500),
                                                        (1500,1500),
                                                        (2500,1500),
                                                        (2500,-1500)]
                                    ),
                                ]
                        )
                    ),
                    ("TURNM1_1",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , None, vec![(-100,-60), (100,60)]),
                                ]
                        )
                    ),
                    ("TURNM2_1",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("M2" , None, vec![(-100,-60), (100,60)]),
                                ]
                        )
                    ),
                    ("TURNM3_1",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("M3" , None, vec![(-100,-60), (100,60)]),
                                ]
                        )
                    ),
                    ("myvia1",
                        (None, None, None,  None, None, None, None, None, None, 
                            vec![("METAL1" , Some(2), vec![(0 , 0), (40000 , 40000)]),
                                 ("V1" , Some(3), vec![(0 , 0), (40000 , 40000)]),
                                 ("M2" , None, vec![(0 , 0), (40000 , 40000)]),
                                ]
                        )
                    ),
                ]
            );
    }
}
