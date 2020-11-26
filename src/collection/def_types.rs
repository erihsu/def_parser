// Define reusable type alias

pub type Properties<'a> = Vec<(
    // property defined in DESIGN Section
    &'a str, // property name
    PropValue<'a>,
)>;

#[derive(Debug, PartialEq)]
pub enum PropValue<'a> {
    SValue(&'a str),
    RValue(f64),
    IValue(i32),
}

#[derive(Debug, PartialEq)]
pub enum Geometry {
    Rect(((i32, i32), (i32, i32))),
    Polygon(Vec<(i32, i32)>),
}

pub type Pts = Vec<(i32, i32)>;
pub type Rect = ((i32, i32), (i32, i32));
// NONDEFAULTRULES
pub type Ndr<'a> = (
    &'a str, // ndr name
    (
        bool, // whether hardspacing
        Vec<(
            // Layer rule.
            &'a str, // layer name
            f64,     // width. integer
            f64,     // diagwidth. integer
            f64,     // spacing. integer
            f64,     // wireext. integer
        )>,
        Vec<&'a str>,        // VIA. specifiy previous vias to use this rule
        Vec<&'a str>,        // VIARULE.
        Vec<(&'a str, i32)>, // minCuts. specifiy the minimuum number of cuts allowed for via using this cut layer
        Properties<'a>,
    ),
);

// SLOT
pub type Slot<'a> = (
    &'a str,       // name of slot
    Vec<Geometry>, // rect/polygon
);

// PINPROPERTIES
pub type Pinprop<'a> = (
    Option<&'a str>, // PIN or componentName. if PIN, it's None, else Some(componentName)
    &'a str,         // pinName of exact PIN or component
    Properties<'a>,
);

// VIAS
pub type Via<'a> = (
    &'a str, // viaName
    (
        Option<&'a str>,                     // viaRule
        Option<(i32, i32)>,                  // cutSize. (xSize, ySize)
        Option<(&'a str, &'a str, &'a str)>, // LAYERS. (botmetalLayer,cutLayer,topMetalLayer)
        Option<(i32, i32)>,                  // cutSpacing. (xCutSpacing,yCutSpacing)
        Option<(i32, i32, i32, i32)>,        // endClosure. (xBotEnc, yBotEnc, xTopEnc, yTopEnc)
        Option<(i32, i32)>,                  // ROWCOL. (numCutRows, NumCutCols)
        Option<(i32, i32)>,                  // ORIGIN. (xOffset, yOffset)
        Option<(i32, i32)>, // OFFSET. (xBotOffset, yBotOffset, xTopOffset, yTopOffset)
        Option<&'a str>,    // PATTERN. cutPattern
        Vec<(
            &'a str,     // Rect Name
            Option<i32>, // maskNum
            Pts,         // list of points
        )>,
    ),
);

// GROUPS
#[derive(Debug, PartialEq)]
pub enum GroupRegion<'a> {
    PreDefined(&'a str), // Region. use predefined region by name
    NewDefined(((i32, i32), (i32, i32))),
}

pub type Group<'a> = (
    (
        &'a str,      // groupName
        Vec<&'a str>, // compNamePattern. A component name, a list of component names or a pattern for a set of components
    ),
    (
        Option<i32>, // SOFT. maxhalfperimeter
        Option<i32>, // SOFT. MAXX
        Option<i32>, // SOFT. MAXY
        GroupRegion<'a>,
        Properties<'a>,
    ),
);

// REGIONS
pub type Region<'a> = (
    (
        &'a str,   // region name
        Vec<Rect>, // define a region as one or more rectangular areas specified by pairs of coordinate points
    ),
    (
        Option<i32>, // TYPE. FENCE or GUIDE
        Properties<'a>,
    ),
);

// FILL
#[derive(Debug, PartialEq)]
pub enum Fill<'a> {
    Layer(
        (
            &'a str,     // name of layer
            Option<i32>, // Mask number
            bool,        // whether OPC
            Vec<Geometry>,
        ),
    ),
    Via(
        (
            &'a str,     // name of via
            Option<i32>, // mask number
            bool,        // whether OPC
            Pts,
        ),
    ),
}

// BLOCKAGE
#[derive(Debug, PartialEq)]
pub enum Blockage<'a> {
    Layer(
        (
            &'a str,
            (
                bool, // SLOTS. Whether creates a blockage on the specified layer where slots cannot be placed.
                bool, // PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
                bool, // FILLS. Whether creates a blockage on the specified layer where metal fills cannot be placed.
                bool, // EXCEPTPGNET. Indicates that whether the blockage only blocks signal net routing, and does not block power or ground net routing.
                Option<i32>, // SPACING or DESIGNRULEWIDTH. minimum spacing allowed between the blockage and any other routing shape
                Option<&'a str>, // COMPONENT. component with which to associate a blockage.
                Option<i32>, // MASK.
            ),
            Vec<Geometry>,
        ),
    ),
    Placement(
        (
            (
                bool, //SOFT. Indicates that whether the initial placement should not use the area, but later phases, such as timing optimization or clock tree synthesis, can use the blockage area.
                Option<f64>, // PARTIAL. Indicates that the initial placement should not use more than partial percentage of the blockage area for standard cells.
                bool, //PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
                Option<&'a str>, // COMPONENT. component with which to associate a blockage.
            ),
            Vec<Geometry>,
        ),
    ),
}

// COMPONENT
pub type Component<'a> = (
    (
        &'a str, // component(instance) name
        &'a str, // model name
    ), // basic
    (
        Option<&'a str>,                    // EEQMASTER
        Option<&'a str>,                    // GENERATE
        Option<i32>,                        // SOURCE
        Option<i32>,                        // WEIGHT
        Option<&'a str>,                    // REGION
        Option<(bool, i32, i32, i32, i32)>, // HALO
        Option<(i32, &'a str, &'a str)>,    // ROUTEDHALO
        Properties<'a>,                     // Properties
    ), //feature
);

// NET

#[derive(Debug, PartialEq)]
pub enum RouteElem<'a> {
    Pt(RtPt<'a>),
    Via(
        (
            RtPt<'a>,
            &'a str, // viaName
        ),
    ),
}

pub type RtPt<'a> = (Option<i32>, Option<i32>, Option<i32>);

pub type RouteBody<'a> = Vec<RouteElem<'a>>;

pub type RegularWireBasic<'a> = (
    &'a str,         // layer name
    Option<&'a str>, // TAPERRULE
    Option<i32>,     // stylNum,
    RouteBody<'a>,
);

pub type RegularWireStmt<'a> = Vec<(
    i32, // 0: cover; 1: fixed; 2: routed; 3: noshield
    Vec<RegularWireBasic<'a>>,
)>;

pub type Net<'a> = (
    (
        &'a str,                               // netName
        Vec<(Option<&'a str>, &'a str, bool)>, // componentName, pinName, whether synthesized.
    ), // basic
    (
        Vec<&'a str>,    // SHIELDNET
        Vec<Vpin<'a>>,   // VPIN
        Vec<SubNet<'a>>, // SUBNET
        Option<i32>,     //XTALK
        Option<&'a str>, // NONDEFAULTRULE
        RegularWireStmt<'a>,
        NetProperty<'a>,
    ), // feature
);

pub type Vpin<'a> = (
    // VPIN
    &'a str,                  // vpin name
    Option<&'a str>,          // layer name
    ((i32, i32), (i32, i32)), // vpin geometry
    Option<i32>,              // 0: placed ; 1: fixed ; 2: covered
    Option<(i32, i32)>,       // vpin location
    Option<i32>,              // orient
);

pub type SubNet<'a> = (
    // SUBNET
    (
        &'a str, // subnet name
        Vec<(
            Option<&'a str>, // compName or None
            Option<&'a str>, // VPIN name or None
            Option<&'a str>, // PIN name
        )>,
    ), // basic
    (
        Option<&'a str>,     // nondefaultrule
        RegularWireStmt<'a>, // regular wiring
    ), // feature
);

// Special Net

pub type SpecialWireBasic<'a> = (
    &'a str,     // layer name
    i32,         // route width
    Option<i32>, // shape code
    Option<i32>, // stylNum,
    RouteBody<'a>,
);

pub type SpecialWireStmt<'a> = Vec<(
    i32, // 0: cover; 1: fixed; 2: routed; 3: noshield
    Vec<SpecialWireBasic<'a>>,
)>;

pub type SNet<'a> = (
    (
        &'a str,                       // special netName
        Vec<(&'a str, &'a str, bool)>, // componentName, pinName, whether pin from PIN macro.
    ), // basic
    (
        Option<f64>, // volts
        SpecialWireStmt<'a>,
        SNetProperty<'a>,
    ), // feature
);

// NetProperty that used in NET
pub type NetProperty<'a> = (
    i32,             //SOURCE. 0: DIST; 1: NETLIST; 2:TEST; 3:TIMING; 4:USER
    bool,            // FIXEDBUMP
    Option<i32>,     // FREQUENCY
    Option<&'a str>, // ORIGINAL
    Option<i32>, // USE. 0: ANALOG; 1:CLOCK; 2:GROUND; 3:POWER; 4:RESET; 5: SCAN; 6:SIGNAL; 7: TIEOFF
    Option<i32>, // PATTERN. 0: BALANCED; 1:STEINER; 2:TRUNK; 3:WIREDLOGIC
    Option<i32>, // ESTCAP
    Option<i32>, // WEIGHT
    Properties<'a>,
);

// NetProperty that used in Special Net
pub type SNetProperty<'a> = (
    i32,             //SOURCE. 0: DIST; 1: NETLIST; 2:TEST; 3:TIMING; 4:USER
    bool,            // FIXEDBUMP
    Option<&'a str>, // ORIGINAL
    i32, // USE. 0: ANALOG; 1:CLOCK; 2:GROUND; 3:POWER; 4:RESET; 5: SCAN; 6:SIGNAL; 7: TIEOFF
    i32, // PATTERN. 0: BALANCED; 1:STEINER; 2:TRUNK; 3:WIREDLOGIC
    Option<i32>, // ESTCAP
    Option<i32>, // WEIGHT
    Properties<'a>,
);

// Style
pub type Style = (
    i32, // style number
    Pts,
);

// Pin
pub type Pin<'a> = (
    (&'a str, &'a str),          // pinName & netName
    bool,                        // Whether special
    i32,                         // direction
    Option<&'a str>,             // NetExpre
    Option<&'a str>,             // PowerPin name
    Option<&'a str>,             // GroundPin name
    Option<i32>,                 // pin mode
    Vec<(i32, Option<&'a str>)>, // ANTENNAPINPARTIALMETALAREA
    Vec<(i32, Option<&'a str>)>, // ANTENNAPINPARTIALMETALSIDEAREA
    Vec<(i32, Option<&'a str>)>, // ANTENNAPINPARTIALCUTAREA
    Vec<(i32, Option<&'a str>)>, // ANTENNAPINDIFFAREA
    Option<i32>,                 // ANTENNAMODEL
    Vec<(i32, Option<&'a str>)>, // ANTENNAPINGATEAREA
    Vec<(i32, &'a str)>,         // ANTENNAPINMAXAREACAR
    Vec<(i32, &'a str)>,         // ANTENNAPINMAXSIDEAREACAR
    Vec<(i32, &'a str)>,         // ANTENNAPINMAXCUTCAR
    Ports<'a>,                   //
);

type Ports<'a> = Vec<Port<'a>>;

pub type Port<'a> = (
    Vec<PortElem<'a>>,
    i32,        // location attribute
    (i32, i32), // location
    i32,        // orient
);

#[derive(Debug)]
pub enum PortElem<'a> {
    Layer((&'a str, Option<i32>, ((i32, i32), (i32, i32)))),
    Polygon((&'a str, Option<i32>, Vec<(i32, i32)>)),
    Via((&'a str, (i32, i32))),
}

pub type ScanChain<'a> = (
    &'a str, // name
    (
        Option<(&'a str, i32)>, // partition
        Option<(
            &'a str, // IN pin
            &'a str, // OUT PIN
        )>, // commonscanpins
        Option<(
            Option<&'a str>, // if from component, then has name; whether none
            &'a str,         // pin name
        )>, // start.
        Option<(
            (
                &'a str, // scancell
                &'a str,
            ), //  in pin
            (
                &'a str, // scancell
                &'a str,
            ), // out pin
            i32, // max bits
        )>, // float
        Option<(
            (
                &'a str, // scancell
                &'a str, // in pin
            ),
            (
                &'a str, // scancell
                &'a str, // out pin
            ),
            i32, // max bits
        )>, // ordered
        Option<(
            Option<&'a str>, // if from component, then has name; whether none
            &'a str,         // pin name
        )>, // stop.
    ),
);
