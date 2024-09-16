
export namespace NetworkFile {
    /**
     * This magic number is `0111 1111` in binary and `0x7F` in hexadecimal.
     */
    const LOWER7_MASK = 127;

    /**
     * This magic number is `1000 000` in binary and `0x80` in hexadecimal.
     */
    const HIGH8_MASK = 128;

    /**
     * Class to perform 7-bit integer decoding and encoding. Under '7-bit
     * -encoding' is meant a representation of an integer as a pure byte
     * buffer. The bytes are separated by their first bit, the 'flag' and
     * the rest, the 'mantisse'. If a 7-bit integer is read, bytes will be
     * read while the flag is set, and terminate with the last byte, which
     * sets this flag to zero. The mantisse is accumulated to create the
     * integer.
     */
    export class Bit7Integer {
        /**
         * Reads the size of an integer in its' 7-bit encoded form.
         *
         * ```
         * 1(000 0001) 0(111 1110)
         * ```
         */
        public static length7BitInt(value: number): number {
            let size = 0;
            do (value >>= 7), size++
            while (value > 0);
            return size;
        }

        /**
         * Reads in bytes of an integer in 7-bit encoding and decodes
         * the number from the generating bytes. The callback function
         * for providing the bytes will be called while the highest bit
         * is set.
         *
         * @example
         *
         * ```
         * 1111 0000 1010 1010 1000 0000 0000 0001 Reading In
         * ^--- ---- ^--- ---- ^--- ---- ^--- ----
         *  111 0000  010 1010  000 0000  000 0001 Writing Out
         * ```
         */
        public static read7BitInt(generate: () => number): number {
            let value = 0,
                shift = 0,
                byte = 0;

            do byte = generate(),
                value |= (byte & LOWER7_MASK) << shift,
                shift += 7
            while (byte & HIGH8_MASK && shift <= 35);

            return value;
        }

        /**
         * Yield bytes encoding an integer into a 7-bit integer.
         */
        public* write7BitInt(value: number) {
            let tmp = 0,
                size = 0;

            do tmp = (value & LOWER7_MASK) | HIGH8_MASK,
                value >>= 7,
                size++,
                yield tmp
            while (value >= HIGH8_MASK && size <= 5);

            return size;
        }
    }
}
