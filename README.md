# def_parser
DEF Specification Parser based on nom

## Syntax
The DEF(design exchange format) syntax definition is carrried out by Cadence, Inc in lef/def reference. Standard DEF file can contain following sections and there is no order requirement of each section.

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
**Notes**: [] around section means that the section in DEF is optional. For more details of each section definition, please refer to the [lef/def reference](http://www.ispd.cc/contests/18/lefdefref.pdf).

