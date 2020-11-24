# def_parser
DEF Specification Parser based on nom

## Syntax
The **DEF(Design Exchange Format)** syntax definition is carrried out by **Cadence Design System, Inc** in lef/def reference. Standard DEF file can contain following sections and there is no order requirement of each section.

```
[ VERSION statement ]
[ DIVIDERCHAR statement ]
[ BUSBITCHARS statement ]
DESIGN statement
[ TECHNOLOGY statement ]
[ UNITS statement ]
[ HISTORY statement ]
[ PROPERTYDEFINITIONS section ]
[ DIEAREA statement ]
[ ROWS statement ]
[ TRACKS statement ]
[ GCELLGRID statement ]
[ VIAS statement ]
[ STYLES statement ]
[ NONDEFAULTRULES statement ]
[ REGIONS statement ]
[ COMPONENTS section ]
[ PINS section ]
[ PINPROPERTIES section ]
[ BLOCKAGES section ]
[ SLOTS section ]
[ FILLS section ]
[ SPECIALNETS section ]
[ NETS section ]
[ SCANCHAINS section ]
[ GROUPS section ]
[ BEGINEXT section ]
END DESIGN statement
```
**Notes**: `[]` around section means that the section in DEF is optional. For more details of each section definition, please refer to the [lef/def reference](http://www.ispd.cc/contests/18/lefdefref.pdf).

## Limitation
1.Each statement should strictly keep the same order as description in def/lef reference. And you should make sure each statement definition keep same pattern with our [def_types](src/collection/def_types.rs)

2. New feature of higher version up 5.7 is not considered in this parser. Welcome contribute.

