

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
pub fn length_7bit<T : std::cmp::PartialEq<i32> + std::ops::ShrAssign<i32>>(mut value: T) -> usize {
    let mut size = 0;

    loop {
        value >>= 7;
        size += 1;

        if value == 0 {
            return size;
        }
    }
}

