use bitflags::bitflags;

pub mod keyname_table;

/// replaces GdkByteOrder
pub enum ByteOrder {
    LsbFirst,
    MsbFirst,
}

bitflags! {
    /// Types of key or button modifiers
    pub struct ModifierType: u32 {
        const SHIFT_MASK = 1 << 0;
        const LOCK_MASK = 1 << 1;
        const CONTROL_MASK = 1 << 2;
        const MOD1_MASK = 1 << 3;
        const MOD2_MASK = 1 << 4;
        const MOD3_MASK = 1 << 5;
        const MOD4_MASK = 1 << 6;
        const MOD5_MASK = 1 << 7;
        const BUTTON1_MASK = 1 << 8;
        const BUTTON2_MASK = 1 << 9;
        const BUTTON3_MASK = 1 << 10;
        const BUTTON4_MASK = 1 << 11;
        const BUTTON5_MASK = 1 << 12;
        // Bits 13 and 14 are used internally by XKB.
        // Bits 15 through 25 are unused.
        const SUPER_MASK = 1 << 26;
        const HYPER_MASK = 1 << 27;
        const META_MASK = 1 << 28;
        // Bit 29 is used internally.
        const RELEASE_MASK = 1 << 30;
        const MODIFIER_MASK = 0x5c001fff; // The Bitwise Or of all masks.
    }
}

/// replaces GdkInputCondition
pub enum InputCondition {
    InputRead = 1,
    InputWrite = 2,
    InputException = 4,
}

/// Replaces GdkStatus
pub enum Status {
    Ok = 0,
    Error = -1,
    ErrorParam = -2,
    ErrorFile = -3,
    ErrorMem = -4,
}

/// Replaces GdkGrabStatus.
///
/// Do not assume that these internally match the integer values returned by X11.
pub enum GrabStatus {
    Success,
    AlreadyGrabbed,
    InvalidTime,
    NotViewable,
    Frozen,
}

pub struct GdkPoint {
    pub x: i32,
    pub y: i32,
}

pub struct GdkRectangle {
    pub point: GdkPoint,
    pub width: i32,
    pub height: i32,
}

pub struct GdkSegment {
    pub point1: GdkPoint,
    pub point2: GdkPoint,
}

pub struct GdkSpan {
    pub point: GdkPoint,
    pub width: i32,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
