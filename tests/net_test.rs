use def_parser::def_parser::net_parser::net_section;

use def_parser::def_parser::def_types::{PropValue, RouteElem};
use std::io::Read;

#[test]
fn test_net_section() {
    let mut input_def = std::fs::File::open("tests/icc/cts_net.def").unwrap();
    let mut data = String::new();
    input_def.read_to_string(&mut data).unwrap();
    let result = net_section(&data).unwrap();

    let net_section = result.1;

    let num = net_section.0;
    let nets = net_section.1;

    assert_eq!(num, 6);

    let net_1_feature = (
        vec![], // sheildnet
        vec![], // vpin
        vec![], // subnet
        None,   // xtalk
        None,   // ndr
        vec![],
        (
            Some(4),
            false, // fixedbump
            None,  // frequency
            None,  // original
            None,  // use
            None,  // pattern
            None,  // EXTCAP
            None,  // WEIGHT
            vec![],
        ), // netproperty
    );

    let net_2_feature = (
        vec!["SN1", "VDD"], // shieldnet
        vec![
            (
                "N1_VP0",
                Some("M3"),
                ((-333, -333), (333, 333)),
                Some(0),
                Some((189560, 27300)),
                Some(0),
            ),
            ("N1_VP8", None, ((-333, -333), (333, 333)), None, None, None),
        ], // vpin
        vec![(
            (
                "N1_SUB0",
                vec![
                    (Some("I2"), None, Some("A")),
                    (None, None, Some("P1")),
                    (None, Some("N1_VP9"), None),
                ],
            ),
            (
                Some("RULE1"),
                vec![(
                    2,
                    vec![(
                        "M1",
                        None,
                        None,
                        vec![
                            RouteElem::Pt((Some(168280), Some(63300), Some(700))),
                            RouteElem::Via(((None, Some(64500), None), "M1_M2")),
                            RouteElem::Via(((Some(169400), None, Some(800)), "M2_M3")),
                        ],
                    )],
                )],
            ),
        )], // subnet
        Some(2),
        Some("RULE1"),
        vec![(
            2,
            vec![
                (
                    "M2",
                    None,
                    None,
                    vec![
                        RouteElem::Pt((Some(14000), Some(341440), None)),
                        RouteElem::Pt((Some(9600), None, None)),
                        RouteElem::Via(((None, Some(282400), None), "nd1VIA12")),
                        RouteElem::Via(((Some(2400), None, None), "TURNM1_1")),
                    ],
                ),
                (
                    "M1",
                    None,
                    None,
                    vec![
                        RouteElem::Pt((Some(2400), Some(282400), None)),
                        RouteElem::Pt((Some(240), None, None)),
                    ],
                ),
            ],
        )],
        (
            Some(1),
            true,
            Some(100),
            Some("N2"),
            Some(6),
            Some(1),
            Some(1500000),
            Some(100),
            vec![
                ("strprop", PropValue::SValue("\"aString\"")),
                ("intprop", PropValue::IValue(1)),
                ("realprop", PropValue::RValue(1.1)),
                ("intrangeprop", PropValue::IValue(25)),
                ("realrangeprop", PropValue::RValue(25.25)),
            ],
        ),
    );

    assert_eq!(
        nets,
        vec![
            (
                (
                    "SCAN",
                    vec![
                        (Some("scancell1"), "PA10", true),
                        (Some("scancell2"), "PA2", true)
                    ]
                ),
                net_1_feature,
            ),
            (
                ("N1", vec![(Some("I1"), "A", false), (None, "P0", false)],),
                net_2_feature,
            )
        ]
    );
}
