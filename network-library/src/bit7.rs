//! Class to perform 7-bit integer decoding and encoding. Under '7-bit
//! -encoding' is meant a representation of an integer as a pure byte
//! buffer. The bytes are separated by their first bit, the 'flag' and
//! the rest, the 'mantisse'. If a 7-bit integer is read, bytes will be
//! read while the flag is set, and terminate with the last byte, which
//! sets this flag to zero. The mantisse is accumulated to create the
//! integer.

/// This magic number is `0111 1111` in binary and `0x7F` in hexadecimal.
/// Using the mathematical and operator 'and' on this number will give you
/// the 7-bit mantisse of a byte.
const LOWER7_MASK: u8 = 127;

/// This magic number is `1000 000` in binary and `0x80` in hexadecimal.
/// Using the mathematical and operator 'and' on this number will give you
/// the 7-bit follow flag of a byte.
const HIGH8_MASK: u8 = 128;

/// Reads the size of an integer in its' 7-bit encoded form. For example,
/// the `u16`` integer stored in binary as `00 (00 0000 1) (111 1110)` would
/// return 2, because it uses two bytes in 7-bit encoding: `1(000 0001) 0(111 1110)`
/// 
/// ```
/// use network_library::bit7::length_7bit;
/// 
/// // Value 0-127 expected to be stored in one byte
/// assert_eq!(length_7bit(0), 1);
/// assert_eq!(length_7bit(1), 1);
/// assert_eq!(length_7bit(126), 1);
/// assert_eq!(length_7bit(127), 1);
/// // Values 2^7 till 2^14 - 1 expected to be stored in two bytes
/// assert_eq!(length_7bit(128), 2);
/// assert_eq!(length_7bit(16383), 2);
/// // Values 2^14 till 2^21 - 1 expected to be stored in three bytes
/// assert_eq!(length_7bit(16384), 3);
/// assert_eq!(length_7bit(2097151), 3);
/// // Values 2^21 till 2^28 - 1 expected to be stored in four bytes
/// assert_eq!(length_7bit(2097152), 4);
/// assert_eq!(length_7bit(268435455), 4);
/// // Values above expected to be stored in five bytes
/// assert_eq!(length_7bit(268435456), 5);
/// ```
pub fn length_7bit<T : std::cmp::PartialEq<usize> + std::ops::ShrAssign<usize>>(mut value: T) -> usize {
    let mut size = 0;

    loop {
        value >>= 7usize;
        size += 1;

        if value == 0 {
            return size;
        }
    }
}

/// TODO This function is broken, the order of shifting seems incorrect, this function needs more edge case tests.
/// 
/// Reads in bytes of an integer in 7-bit encoding and decodes
/// the number from the generating bytes. The callback function
/// for providing the bytes will be called while the highest bit
/// is set.
///
/// ```text
/// 1111 0000 1010 1010 1000 0000 0000 0001 Reading In
/// ^--- ---- ^--- ---- ^--- ---- ^--- ----
///  111 0000  010 1010  000 0000  000 0001 Writing Out
/// ```
/// 
/// ```
/// use network_library::bit7::read_7bit;
/// 
/// // When the 7-bit follow flag is not set, returns the number.
/// assert_eq!(read_7bit(&|| 0), 0);
/// assert_eq!(read_7bit(&|| 1), 1);
/// assert_eq!(read_7bit(&|| 127), 127);
/// ```
pub fn read_7bit(callback: &dyn Fn() -> u8) -> usize {
    let mut value: usize = 0;
    let mut shift: usize = 7;
    const limit_bits: usize = std::mem::size_of::<usize>() * 7;

    loop {
        let byte = callback();
        value |= (byte & LOWER7_MASK) as usize;
        shift += 7;

        if byte & HIGH8_MASK == 0 || shift > limit_bits {
            return value;
        }

        value <<= 7;
    }
}

/// TODO add tests
/// 
/// Yield bytes encoding an integer into a 7-bit integer elsewhere.
/// This function will stream data to an function that accepts one
/// byte (`u8`) as parameter.
pub fn write_7bit(mut value: usize, callback: &dyn Fn(u8) -> ()) -> () {
    while value as u8 >= HIGH8_MASK {
        callback((((value as u8) & LOWER7_MASK) | HIGH8_MASK) as u8);
        value >>= 7;
    }
    callback(value as u8);
}

