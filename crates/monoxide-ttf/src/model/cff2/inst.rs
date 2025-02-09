//! CFF2 instructions

macro_rules! inst {
    (
        $(#[$inst_attr:meta])*
        $vis:vis // instruction visibility
        enum $typename:ident { // instruction type name
            // instructions
            $(
                $(#[$attr:meta])* // Meta attributes
                $name:ident // instruction name
                (
                    $encoding:expr, // encoded bytes for the instruction
                    // Stack manipulation
                    $([ $( $stack:tt )* ])|* // stack manipulation
                )
            ),* $(,)?
        }
    ) => {
        $(#[$inst_attr])*
        $vis enum $typename {
            $(
                $(#[$attr])*
                $name
            ),*
        }

        impl $typename {
            /// Returns the encoded bytes for the instruction.
            pub fn encoding(&self) -> &'static [u8] {
                match self {
                    $(
                        $typename::$name => &$encoding,
                    )*
                }
            }
        }
    };
}

inst! { pub enum TopDictInst {
    /// Specifies the offset to the CharStringINDEX subtable, from the start of the CFF2 table.
    /// Required.
    CharStringIndexOffset( [0x11], [ offset ] ),

    /// Specifies the offset to the VariationStore subtable, from the start of the CFF2 table.
    /// Required in fonts that have CFF2 glyph variations, otherwise forbidden.
    VariationStoreOffset( [0x18], [ offset ] ),

    /// Specifies the offset to the FontDICTINDEX subtable, from the start of the CFF2 table.
    /// Required.
    FontDictIndexOffset( [0x0c, 0x24], [ offset ] ),

    /// Specifies the offset to the FontDICTSelect subtable, from the start of the CFF2 table.
    /// Optional. If the CFF2 table has only one FontDICT, there is no need for a FontDICTSelect subtable, and the FontDICTSelectOffset operator must not be used.
    FontDictSelectOffset( [0x0c, 0x25], [ offset ] ),

    /// Specifies the scale factor for glyph coordinates within the em square, similar to the unitsPerEm field in the 'head' table. However, a reciprocal value is used (thus 1 / unitsPerEm). This value occurs as the first and fourth operands—both scale operands must have the same value. Other operands must be zero.
    /// Required if unitsPerEm is not equal to 1000. For the common case in which unitsPerEm is 1000, the key-value pair is created with the default value and the FontMatrix operator should be omitted.
    FontMatrix( [0x0c, 0x07], [ scale 0 0 scale 0 0 ] ),
}}

inst! { pub enum CharStringOps {
    /// Moves the current point to a position at the relative coordinates (dx1, dy1) and starts a new contour.
    RMoveTo( [0x15], [ dx1 dy1 ] ),

    /// Moves the current point dx units in the horizontal direction and starts a new contour.
    HMoveTo( [0x16], [ dx ] ),

    /// Moves the current point dy units in the vertical direction and starts a new contour.
    VMoveTo( [0x04], [ dy ] ),

    /// Appends a line segment from the current point to a position at the relative coordinates (dx, dy). Additional rlineto operations are performed for all subsequent argument pairs. The number of arguments must be even, and the number of lines is determined from the number of arguments on the stack.
    RLineTo( [0x05], [ (dx dy)+ ] ),

    /// Appends a horizontal line segment from the current point to a position at the relative coordinates (d, 0). When multiple arguments are used, additional line segments are appended in alternating vertical and horizontal orientations. Thus, the second argument appends a vertical line from the end of the previous line to the relative coordinates (0, d), the third argument appends a horizontal line to the relative coordinates (d, 0), and so on. The number of line segments is determined from the number of arguments on the stack.
    HLineTo( [0x06], [ d+ ] ),

    /// Appends a vertical line segment from the current point to a position at the relative coordinates (0, d). As with hlineto, when multiple arguments are used, the orientation alternates for each additional argument.
    VLineTo( [0x07], [ d+ ] ),

    /// Appends a cubic Bézier curve, defined by the points p0, p1, p2, p3 where p0 is located at the current point, p1 is the first off-curve point given by the relative coordinates (dxa, dya), p2 is the second off-curve point given by the relative coordinates (dxb, dyb) and p3 is the end point given by the relative coordinates (dxc, dyc). The location of each point is defined relative to the preceding one. For each subsequent set of six arguments, an additional curve is appended to the previous curve segment. The number of curve segments is determined from the number of arguments on the stack and is limited only by the size of the stack.
    RRCurveTo( [0x08], [ (dxa dya dxb dyb dxc dyc)+ ] ),

    /// Appends one or more Bézier curves starting from the current point, as described by the dxa…dxc set(s) of arguments. If the argument count is a multiple of four, the curve starts and ends with horizontal tangents. For each curve segment, the first off-curve point is given by relative coordinates (dxa, 0), the second off-curve point is given by relative coordinates (dxb, dyb), and the end point is given by relative coordinates (dxc, 0)
    /// If the argument count is one more than a multiple of four, the first curve does not begin with a horizontal tangent and dya is given by the first argument; thus the first off-curve point is at (dxa, dya) relative to the current point. This option is available only for the first curve segment.
    HHCurveTo( [0x1b], [ dya? (dxa dxb dyb dxc)+ ] ),

    /// Appends one or more Bézier curves starting from the current point with start/end tangents that alternate between horizontal and vertical.
    HVCurveTo( [0x1f], [ dx1 dx2 dy2 dy3 dx3? ] | [ (dxa dxb dyb dyc dyd dxe dye dxf)+ dyf? ] | [ dx1 dx2 dy2 dy3 (dya dxb dyb dxc dxd dxe dye dyf)* dxf? ] ),

    /// Appends one or more Bézier curves starting from the current point with start/end tangents that alternate between vertical and horizontal.
    VHCurveTo( [0x1e], [ dy1 dx2 dy2 dx3 dy3? ] | [ (dya dxb dyb dxc dxd dxe dye dyf)+ dxf? ] | [ dy1 dx2 dy2 dx3 (dxa dxb dyb dyc dyd dxe dye dxf)* dyf? ] ),

    /// Appends a sequence of Bézier curves followed by a line.
    RCurveLine( [0x18], [ (dxa dya dxb dyb dxc dyc)+ dxd dyd ] ),

    /// Appends a sequence of lines followed by a Bézier curve.
    RLineCurve( [0x19], [ (dxa dya)+ dxb dyb dxc dyc dxd dyd ] ),

    /// Appends two Bézier curves described by dx1…dy6. The operand fd specifies a flex depth threshold in 1/100ths of a device pixel. The curves must be rendered as a straight line when the flex depth is less than fd/100 device pixels, and as curved lines when the flex depth is greater than or equal to fd/100 device pixels.
    Flex( [0x0c, 0x23], [ dx1 dy1 dx2 dy2 dx3 dy3 dx4 dy4 dx5 dy5 dx6 dy6 fd ] ),

    /// Appends two Bézier curves described by dx1…dx6. The curves must be rendered as a straight line when the flex depth is less than 0.5 device pixels (threshold fd = 50 is implicit), and as curves when the flex depth is greater than or equal to 0.5 device pixels.
    HFlex( [0x0c, 0x22], [ dx1 dx2 dy2 dx3 dx4 dx5 dx6 ] ),

    /// Appends two Bézier curves described by dx1…dx6. The curves must be rendered as a straight line when the flex depth is less than 0.5 device pixels (threshold fd = 50 is implicit), and as curves when the flex depth is greater than or equal to 0.5 device pixels.
    HFlex1( [0x0c, 0x24], [ dx1 dy1 dx2 dy2 dx3 dx4 dx5 dy5 dx6 ] ),

    /// Appends two Bézier curves described by dx1…d6. The d6 operand will be interpreted as either a dx or dy value depending on the combined curve. The curves must be rendered as a straight line when the flex depth is less than 0.5 device pixels (threshold fd = 50 is implicit), and as curves when the flex depth is greater than or equal to 0.5 device pixels.
    Flex1( [0x0c, 0x25], [ dx1 dy1 dx2 dy2 dx3 dy3 dx4 dy4 dx5 dy5 d6 ] ),


    /// Calls the subroutine in LocalSubrINDEX with the index determined by subr#. This operand must be added to the subroutine bias number before being used as the index. See CharString concepts: Subroutines for details on calculation of the subroutine bias.
    CallSubr( [0x0a], [ ... subr_idx ] ),

    /// Calls the subroutine in GlobalSubrINDEX with the index determined by subr#. This operand must be added to the subroutine bias number before being used as the index. See CharString concepts: Subroutines for details on calculation of the subroutine bias.
    CallGSubr( [0x1d], [ ... subr_idx ] ),

    /// Activates a particular list of variation regions from a VariationStore subtable. May only be used once in a CharString and must precede the first use of the blend operator.
    VSIndex( [0x0f], [ ivd ] ),

    /// Pops n + n * k + 1 operands from the stack, processes them according to the OpenType variations interpolation algorithm, then pushes n result numbers back onto the stack.
    Blend( [0x10], [ default_values+ n_k_deltas+ n ] ),
}}
