// Define reusable type alias

pub type DesignConfig<'a> = (
    &'a str,                  // design name
    Option<&'a str>,          // technology name
    Option<i32>,              // units
    Option<Vec<PropDef<'a>>>, // propdef
    Option<Vec<(i32, i32)>>,  // die area
    Option<Vec<Row<'a>>>,     // rows
    Option<Vec<Track<'a>>>,   // tracks
    Option<Vec<GcellGrid>>,   // gcellgrid
);

pub type PropDef<'a> = (
    &'a str, // object type of property. ie, design, region, group, component,...
    &'a str, // property name
    (
        char, // data type of property
        Option<&'a str>,
        Option<(i32, Option<(i32, i32)>)>,
        Option<(f64, Option<(f64, f64)>)>,
    ),
);

pub type Row<'a> = (
    &'a str, // name of row rule
    &'a str, // row rule type
    i32,     //  the origin of rule scope along axis X
    i32,     // the origin of rule scope along axis Y
    i32,     // orient of row rule
    i32,     // step number along axis X
    i32,     // step number along axis Y
    i32,     // step size along axis X
    i32,     // step size along axis Y
    Option<Properties<'a>>,
);

pub type Track<'a> = (
    char,                 // axis. 'X' or 'Y'
    i32,                  // the origin of gcell grid along axis X/Y
    i32,                  // the step number of track
    i32,                  // the step size of track
    Option<Vec<&'a str>>, // the metal layer of track
);

pub type GcellGrid = (
    char, // axis. 'X' or 'Y'
    i32,  // the origin of gcell grid along axis X/Y
    i32,  // the step number of gcell grid
    i32,  // the step size of gcell grid
);

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

pub type Location = (
    i32,        // location attribute
    (i32, i32), // location
    i32,        // orient
);

pub type Pts = Vec<(i32, i32)>;
pub type Rect = ((i32, i32), (i32, i32));
// NONDEFAULTRULES
pub type Ndr<'a> = (
    &'a str, // ndr name
    (
        bool, // whether hardspacing
        Vec<(
            // Layer rule.
            &'a str,     // layer name
            i32,         // width. integer
            Option<i32>, // diagwidth. integer
            Option<i32>, // spacing. integer
            Option<i32>, // wireext. integer
        )>,
        Vec<&'a str>,        // VIA. specifiy previous vias to use this rule
        Vec<&'a str>,        // VIARULE.
        Vec<(&'a str, i32)>, // (curLayer,minCuts). specifiy the minimuum number of cuts allowed for via using this cut layer
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
#[derive(Debug)]
pub enum ViaBody<'a> {
    Fixed(
        Vec<(
            &'a str,  // Layer Name
            Geometry, //
        )>,
    ),
    Generated(
        (
            &'a str,                      // viaRule
            (i32, i32),                   // cutSize. (xSize, ySize)
            (&'a str, &'a str, &'a str),  // LAYERS. (botmetalLayer,cutLayer,topMetalLayer)
            (i32, i32),                   // cutSpacing. (xCutSpacing,yCutSpacing)
            (i32, i32, i32, i32),         // endClosure. (xBotEnc, yBotEnc, xTopEnc, yTopEnc)
            Option<(i32, i32)>,           // ROWCOL. (numCutRows, NumCutCols)
            Option<(i32, i32)>,           // ORIGIN. (xOffset, yOffset)
            Option<(i32, i32, i32, i32)>, // OFFSET. (xBotOffset, yBotOffset, xTopOffset, yTopOffset)
            Option<&'a str>,              // PATTERN. cutPattern
        ),
    ),
}

pub type Via<'a> = (
    &'a str, // viaName
    ViaBody<'a>,
);

// GROUPS

pub type Group<'a> = (
    &'a str,      // groupName
    Vec<&'a str>, // compNamePattern. A component name, a list of component names or a pattern for a set of components
    &'a str,      // regionName
    Properties<'a>,
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
            &'a str, // name of layer
            bool,    // whether OPC
            Vec<Geometry>,
        ),
    ),
    Via(
        (
            &'a str, // name of via
            bool,    // whether OPC
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
        Option<i32>,                        // SOURCE
        (i32, Option<((i32, i32), i32)>),   // location and attribute
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

pub type RegularWireStmt<'a> = (
    i32, // 0: cover; 1: fixed; 2: routed; 3: noshield
    Vec<RegularWireBasic<'a>>,
);

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
        Vec<RegularWireStmt<'a>>,
        NetProperty<'a>,
    ), // feature
);

pub type Vpin<'a> = (
    // VPIN
    &'a str,                  // vpin name
    &'a str,                  // layer name
    ((i32, i32), (i32, i32)), // vpin geometry
    i32,                      // 0: placed ; 1: fixed ; 2: covered
    (i32, i32),               // vpin location
    i32,                      // orient
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
        Option<&'a str>,          // nondefaultrule
        Vec<RegularWireStmt<'a>>, // regular wiring
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

pub enum SpecialWireStmt<'a> {
    Polygon((&'a str, Pts)),
    Rect((&'a str, Rect)),
    Route(
        (
            i32, // location attribute
            Vec<SpecialWireBasic<'a>>,
        ),
    ),
}

pub type SNet<'a> = (
    (
        &'a str,                               // special netName
        Vec<(Option<&'a str>, &'a str, bool)>, // componentName, pinName, whether synthesized.
    ), // basic
    (
        Option<i32>, // volts
        Vec<SpecialWireStmt<'a>>,
        SNetProperty<'a>,
    ), // feature
);

// NetProperty that used in NET
pub type NetProperty<'a> = (
    Option<i32>,     //SOURCE. 0: DIST; 1: NETLIST; 2:TEST; 3:TIMING; 4:USER
    bool,            // FIXEDBUMP
    Option<f64>,     // FREQUENCY
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
    (&'a str, &'a str), // pinName & netName
    (
        bool,            // Whether special
        Option<i32>,     // direction
        Option<&'a str>, // NetExpre
        Option<&'a str>, // PowerPin name
        Option<&'a str>, // GroundPin name
        Option<i32>,     // pin mode
        // antenna not supported
        Port<'a>,
    ), //
);

// #[derive(Debug, PartialEq)]
// pub enum PinAntenna<'a> {
//     PartialMetalArea((i32, Option<&'a str>)),
//     PartialMetalSideArea((i32, Option<&'a str>)),
//     PartialCutArea((i32, Option<&'a str>)),
//     DiffArea((i32, Option<&'a str>)),
//     Model(Option<i32>),
//     GateArea((i32, Option<&'a str>)),
//     MaxAreaCar((i32, &'a str)),
//     MaxSideAreaCar((i32, &'a str)),
//     MaxCutCar((i32, &'a str)),
// }

#[derive(Debug)]
pub enum Port<'a> {
    ManyPorts(Vec<(Vec<PortElem<'a>>, Location)>),
    SinglePort((Vec<PortElem<'a>>, Location)),
}

#[derive(Debug, PartialEq)]
pub enum PortElem<'a> {
    Layer((&'a str, Option<i32>, ((i32, i32), (i32, i32)))),
    Polygon((&'a str, Option<i32>, Vec<(i32, i32)>)),
    Via((&'a str, (i32, i32))),
}

pub type ScanChain<'a> = (
    &'a str,                        // name
    Option<(&'a str, Option<i32>)>, // partition
    Option<(
        &'a str, // IN pin
        &'a str, // OUT PIN
    )>, // commonscanpins
    Option<(
        Option<&'a str>, // if from component, then has name; whether none
        &'a str,         // pin name
    )>, // start.
    Option<(
        &'a str,     // scancell
        &'a str,     //  in pin
        &'a str,     // out pin
        Option<i32>, // max bits
    )>, // float
    Option<(
        &'a str,     // scancell
        &'a str,     // in pin
        &'a str,     // out pin
        Option<i32>, // max bits
    )>, // ordered
    Option<(
        Option<&'a str>, // if from component, then has name; whether none
        &'a str,         // pin name
    )>, // stop.
);
