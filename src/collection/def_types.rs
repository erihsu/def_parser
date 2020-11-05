// Define reusable type alias

pub type Properties<'a> = Vec<(
    // property defined in DESIGN Section
    &'a str,         // property name
    Option<&'a str>, // property value if it's STRING type
    Option<f64>,
)>;

#[derive(Debug)]
pub enum Geometry {
    Rect(((i32, i32), (i32, i32))),
    Polygon(Vec<(i32, i32)>),
}

pub type Pts = Vec<(i32, i32)>;
// NONDEFAULTRULES
pub type Ndr<'a> = (
    &'a str, // nondefault layer name
    bool,    // whether hardspacing
    Vec<(
        // Layer rule.
        &'a str, // layer name
        i32,     // width. integer
        i32,     // diagwidth. integer
        i32,     // spacing. integer
        i32,     // wireext. integer
    )>,
    Vec<&'a str>,   // VIA. specifiy previous vias to use this rule
    &'a str,        // VIARULE.
    (&'a str, i32), // minCuts. specifiy the minimuum number of cuts allowed for via using this cut layer
    Properties<'a>,
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
    &'a str,                             // viaName
    Option<&'a str>,                     // viaRule
    Option<(i32, i32)>,                  // cutSize. (xSize, ySize)
    Option<(&'a str, &'a str, &'a str)>, // LAYERS. (botmetalLayer,cutLayer,topMetalLayer)
    Option<(i32, i32)>,                  // cutSpacing. (xCutSpacing,yCutSpacing)
    Option<(i32, i32, i32, i32)>,        // endClosure. (xBotEnc, yBotEnc, xTopEnc, yTopEnc)
    Option<(i32, i32)>,                  // ROWCOL. (numCutRows, NumCutCols)
    Option<(i32, i32)>,                  // ORIGIN. (xOffset, yOffset)
    Option<(i32, i32)>,                  // OFFSET. (xBotOffset, yBotOffset, xTopOffset, yTopOffset)
    Option<&'a str>,                     // PATTERN. cutPattern
    Vec<(
        &'a str,     // Rect Name
        Option<i32>, // maskNum
        Pts,         // list of points
    )>,
);

// GROUPS
#[derive(Debug)]
pub enum GroupRegion<'a> {
    PreDefined(&'a str), // Region. use predefined region by name
    NewDefined(((i32, i32), (i32, i32))),
}

pub type Group<'a> = (
    &'a str,      // groupName
    Vec<&'a str>, // compNamePattern. A component name, a list of component names or a pattern for a set of components
    Option<i32>,  // SOFT. maxhalfperimeter
    Option<i32>,  // SOFT. MAXX
    Option<i32>,  // SOFT. MAXY
    GroupRegion<'a>,
    Properties<'a>,
);

// REGIONS
pub type Region<'a> = (
    &'a str,                       // region name
    Vec<((i32, i32), (i32, i32))>, // define a region as one or more rectangular areas specified by pairs of coordinate points
    Option<i32>,                   // TYPE. FENCE or GUIDE
    Properties<'a>,
);

// FILL
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Blockage<'a> {
    Layer(
        (
            &'a str,
            (
                Option<i32>, // SPACING or DESIGNRULEWIDTH. minimum spacing allowed between the blockage and any other routing shape
                Option<&'a str>, // COMPONENT. component with which to associate a blockage.
                bool, // SLOTS. Whether creates a blockage on the specified layer where slots cannot be placed.
                bool, // FILLS. Whether creates a blockage on the specified layer where metal fills cannot be placed.
                bool, // PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
                bool, // EXCEPTPGNET. Indicates that whether the blockage only blocks signal net routing, and does not block power or ground net routing.
                i32,  // MASK.
            ),
            Vec<Geometry>,
        ),
    ),
    Placement(
        (
            (
                Option<&'a str>, // COMPONENT. component with which to associate a blockage.
                bool, //PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
                bool, //SOFT. Indicates that whether the initial placement should not use the area, but later phases, such as timing optimization or clock tree synthesis, can use the blockage area.
                Option<f64>, // PARTIAL. Indicates that the initial placement should not use more than partial percentage of the blockage area for standard cells.
            ),
            Vec<Geometry>,
        ),
    ),
}

// COMPONENT
pub type Component<'a> = (
    &'a str,                            // component(instance) name
    &'a str,                            // model name
    Option<&'a str>,                    // EEQMASTER
    Option<&'a str>,                    // GENERATE
    Option<i32>,                        // SOURCE
    Option<i32>,                        // WEIGHT
    Option<&'a str>,                    // REGION
    Option<(bool, i32, i32, i32, i32)>, // HALO
    Option<(i32, &'a str, &'a str)>,    // ROUTEDHALO
    Properties<'a>,                     // Properties
);

// NET

#[derive(Debug)]
pub enum RoutingPoint<'a> {
    Pt((i32, i32, Option<i32>)),
    Via(
        (
            &'a str,     // viaName
            Option<i32>, // orient
        ),
    ),
}

pub type RegularWireBasic<'a> = (
    Option<i32>,           // 0: cover; 1: fixed; 2: routed; 3: noshield
    &'a str,               // layer name
    Option<&'a str>,       // TAPERRULE
    Option<i32>,           // stylNum,
    Vec<RoutingPoint<'a>>, // routing points
);

pub type Net<'a> = (
    (&'a str, bool),               // netName, whether mustjoin
    Vec<(&'a str, &'a str, bool)>, // componentName, pinName, whether pin from PIN macro.
    Vec<&'a str>,                  // SHIELDNET
    Vec<(
        // VPIN
        &'a str,                  // vpin name
        Option<&'a str>,          // layer name
        ((i32, i32), (i32, i32)), // vpin geometry
        Option<i32>,              // 0: placed ; 1: fixed ; 2: covered
        Option<(i32, i32)>,       // vpin location
        Option<i32>,              // orient
    )>,
    Vec<(
        // SUBNET
        &'a str,                                                   // subnet name
        Vec<(&'a str, &'a str)>,                                   // pinname or vpin name
        Option<&'a str>,                                           // nondefaultrule
        Option<(RegularWireBasic<'a>, Vec<RegularWireBasic<'a>>)>, // regular wiring
    )>,
    Option<i32>, //XTALK
    Option<i32>, // FREQUENCY
    NetProperty<'a>,
);

// Special Net

#[derive(Debug)]
pub enum WireOption<'a> {
    Cover(bool),
    Fixed(bool),
    Routed(bool),
    Shield(&'a str),
}

#[derive(Debug)]
pub enum WireShape {
    Ring,
    PadRing,
    BlockRing,
    Stripe,
    FollowPin,
    IOWire,
    CoreWire,
    BlockWire,
    BlockageWire,
    FillWire,
    FillWireOpc,
    DrcFill,
}

pub type SpecialWireBasic<'a> = (
    Option<WireOption<'a>>,
    (&'a str, i32),
    Option<WireShape>,
    Option<i32>,
    Vec<RoutingPoint<'a>>,
);

pub type SNet<'a> = (
    &'a str,                       // special netName
    Vec<(&'a str, &'a str, bool)>, // componentName, pinName, whether pin from PIN macro.
    Option<f64>,                   // volts
    Option<(SpecialWireBasic<'a>, Vec<SpecialWireBasic<'a>>)>,
    NetProperty<'a>,
);

// NetProperty that commonly used in NET and Special Net
pub type NetProperty<'a> = (
    Option<i32>,     //SOURCE. 0: DIST; 1: NETLIST; 2:TEST; 3:TIMING; 4:USER
    bool,            // FIXEDBUMP
    Option<&'a str>, // ORIGINAL
    Option<i32>, // USE. 0: ANALOG; 1:CLOCK; 2:GROUND; 3:POWER; 4:RESET; 5: SCAN; 6:SIGNAL; 7: TIEOFF
    Option<i32>, // PATTERN. 0: BALANCED; 1:STEINER; 2:TRUNK; 3:WIREDLOGIC
    Option<i32>, // ESTCAP
    Option<i32>, // WEIGHT
    Properties<'a>,
);

// Style
pub type Style = (
    i32, // style number
    Pts,
);
