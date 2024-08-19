use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GlyphFlags: u8 {
        ///	If set, the point is on the curve;
        /// Otherwise, it is off the curve.
        const OnCurve = 1;

        /// If set, the corresponding x-coordinate is 1 byte long;
        /// Otherwise, the corresponding x-coordinate is 2 bytes long
        const ShortVectorX = 1 << 1;

        /// If set, the corresponding y-coordinate is 1 byte long;
        /// Otherwise, the corresponding y-coordinate is 2 bytes long
        const ShortVectorY = 1 << 2;

        /// If set, the next byte specifies the number of additional times this set of flags is to be repeated.
        /// In this way, the number of flags listed can be smaller than the number of points in a character.
        const Repeat = 1 << 3;

        /// This flag has one of two meanings, depending on how the x-Short Vector flag is set.
        /// If the x-Short Vector bit is set, this bit describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        /// If the x-short Vector bit is not set, and this bit is set, then the current x-coordinate is the same as the previous x-coordinate.
        /// If the x-short Vector bit is not set, and this bit is not set, the current x-coordinate is a signed 16-bit delta vector. In this case, the delta vector is the change in x
        const IsSameX = 1 << 4;

        /// See `IsSameX`
        const IsPositiveX = 1 << 4;

        /// This flag has one of two meanings, depending on how the y-Short Vector flag is set.
        /// If the y-Short Vector bit is set, this bit describes the sign of the value, with a value of 1 equalling positive and a zero value negative.
        /// If the y-short Vector bit is not set, and this bit is set, then the current y-coordinate is the same as the previous y-coordinate.
        /// If the y-short Vector bit is not set, and this bit is not set, the current y-coordinate is a signed 16-bit delta vector. In this case, the delta vector is the change in y
        const IsSameY = 1 << 5;

        ///See `IsSameY`
        const IsPositiveY = 1 << 5;

        /// Set to zero
        const Reserved = 1 << 6 | 1 << 7;
    }
}
