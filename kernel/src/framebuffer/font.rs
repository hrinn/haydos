/// 8x16 Bitmap font for framebuffer writing
pub static FONT: [u8; 1536] = [
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 33 0x21 '!'
    0b00000000, 0b00000000, 0b00011000, 0b00111100, 0b00111100, 0b00111100, 0b00011000, 0b00011000,
    0b00011000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 34 0x22 '"'
    0b00000000, 0b01100110, 0b01100110, 0b01100110, 0b00100100, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 35 0x23 '#'
    0b00000000, 0b00000000, 0b00000000, 0b01101100, 0b01101100, 0b11111110, 0b01101100, 0b01101100,
    0b01101100, 0b11111110, 0b01101100, 0b01101100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 36 0x24 '$'
    0b00011000, 0b00011000, 0b01111100, 0b11000110, 0b11000010, 0b11000000, 0b01111100, 0b00000110,
    0b00000110, 0b10000110, 0b11000110, 0b01111100, 0b00011000, 0b00011000, 0b00000000, 0b00000000,
    // 37 0x25 '%'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11000010, 0b11000110, 0b00001100, 0b00011000,
    0b00110000, 0b01100000, 0b11000110, 0b10000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 38 0x26 '&'
    0b00000000, 0b00000000, 0b00111000, 0b01101100, 0b01101100, 0b00111000, 0b01110110, 0b11011100,
    0b11001100, 0b11001100, 0b11001100, 0b01110110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 39 0x27 '''
    0b00000000, 0b00110000, 0b00110000, 0b00110000, 0b01100000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 40 0x28 '('
    0b00000000, 0b00000000, 0b00001100, 0b00011000, 0b00110000, 0b00110000, 0b00110000, 0b00110000,
    0b00110000, 0b00110000, 0b00011000, 0b00001100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 41 0x29 ')'
    0b00000000, 0b00000000, 0b00110000, 0b00011000, 0b00001100, 0b00001100, 0b00001100, 0b00001100,
    0b00001100, 0b00001100, 0b00011000, 0b00110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 42 0x2a '*'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01100110, 0b00111100, 0b11111111,
    0b00111100, 0b01100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 43 0x2b '+'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b01111110,
    0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 44 0x2c ','
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00011000, 0b00011000, 0b00011000, 0b00110000, 0b00000000, 0b00000000, 0b00000000,
    // 45 0x2d '-'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111110,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 46 0x2e '.'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 47 0x2f '/'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000010, 0b00000110, 0b00001100, 0b00011000,
    0b00110000, 0b01100000, 0b11000000, 0b10000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 48 0x30 '0'
    0b00000000, 0b00000000, 0b00111000, 0b01101100, 0b11000110, 0b11000110, 0b11010110, 0b11010110,
    0b11000110, 0b11000110, 0b01101100, 0b00111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 49 0x31 '1'
    0b00000000, 0b00000000, 0b00011000, 0b00111000, 0b01111000, 0b00011000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b01111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 50 0x32 '2'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b00000110, 0b00001100, 0b00011000, 0b00110000,
    0b01100000, 0b11000000, 0b11000110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 51 0x33 '3'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b00000110, 0b00000110, 0b00111100, 0b00000110,
    0b00000110, 0b00000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 52 0x34 '4'
    0b00000000, 0b00000000, 0b00001100, 0b00011100, 0b00111100, 0b01101100, 0b11001100, 0b11111110,
    0b00001100, 0b00001100, 0b00001100, 0b00011110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 53 0x35 '5'
    0b00000000, 0b00000000, 0b11111110, 0b11000000, 0b11000000, 0b11000000, 0b11111100, 0b00000110,
    0b00000110, 0b00000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 54 0x36 '6'
    0b00000000, 0b00000000, 0b00111000, 0b01100000, 0b11000000, 0b11000000, 0b11111100, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 55 0x37 '7'
    0b00000000, 0b00000000, 0b11111110, 0b11000110, 0b00000110, 0b00000110, 0b00001100, 0b00011000,
    0b00110000, 0b00110000, 0b00110000, 0b00110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 56 0x38 '8'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 57 0x39 '9'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b11000110, 0b01111110, 0b00000110,
    0b00000110, 0b00000110, 0b00001100, 0b01111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 58 0x3a ':'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000,
    0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 59 0x3b ';'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000,
    0b00000000, 0b00011000, 0b00011000, 0b00110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 60 0x3c '<'
    0b00000000, 0b00000000, 0b00000000, 0b00000110, 0b00001100, 0b00011000, 0b00110000, 0b01100000,
    0b00110000, 0b00011000, 0b00001100, 0b00000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 61 0x3d '='
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111110, 0b00000000, 0b00000000,
    0b01111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 62 0x3e '>'
    0b00000000, 0b00000000, 0b00000000, 0b01100000, 0b00110000, 0b00011000, 0b00001100, 0b00000110,
    0b00001100, 0b00011000, 0b00110000, 0b01100000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 63 0x3f '?'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b00001100, 0b00011000, 0b00011000,
    0b00011000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 64 0x40 '@'
    0b00000000, 0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b11011110, 0b11011110,
    0b11011110, 0b11011100, 0b11000000, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 65 0x41 'A'
    0b00000000, 0b00000000, 0b00010000, 0b00111000, 0b01101100, 0b11000110, 0b11000110, 0b11111110,
    0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 66 0x42 'B'
    0b00000000, 0b00000000, 0b11111100, 0b01100110, 0b01100110, 0b01100110, 0b01111100, 0b01100110,
    0b01100110, 0b01100110, 0b01100110, 0b11111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 67 0x43 'C'
    0b00000000, 0b00000000, 0b00111100, 0b01100110, 0b11000010, 0b11000000, 0b11000000, 0b11000000,
    0b11000000, 0b11000010, 0b01100110, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 68 0x44 'D'
    0b00000000, 0b00000000, 0b11111000, 0b01101100, 0b01100110, 0b01100110, 0b01100110, 0b01100110,
    0b01100110, 0b01100110, 0b01101100, 0b11111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 69 0x45 'E'
    0b00000000, 0b00000000, 0b11111110, 0b01100110, 0b01100010, 0b01101000, 0b01111000, 0b01101000,
    0b01100000, 0b01100010, 0b01100110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 70 0x46 'F'
    0b00000000, 0b00000000, 0b11111110, 0b01100110, 0b01100010, 0b01101000, 0b01111000, 0b01101000,
    0b01100000, 0b01100000, 0b01100000, 0b11110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 71 0x47 'G'
    0b00000000, 0b00000000, 0b00111100, 0b01100110, 0b11000010, 0b11000000, 0b11000000, 0b11011110,
    0b11000110, 0b11000110, 0b01100110, 0b00111010, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 72 0x48 'H'
    0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11111110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 73 0x49 'I'
    0b00000000, 0b00000000, 0b00111100, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 74 0x4a 'J'
    0b00000000, 0b00000000, 0b00011110, 0b00001100, 0b00001100, 0b00001100, 0b00001100, 0b00001100,
    0b11001100, 0b11001100, 0b11001100, 0b01111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 75 0x4b 'K'
    0b00000000, 0b00000000, 0b11100110, 0b01100110, 0b01100110, 0b01101100, 0b01111000, 0b01111000,
    0b01101100, 0b01100110, 0b01100110, 0b11100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 76 0x4c 'L'
    0b00000000, 0b00000000, 0b11110000, 0b01100000, 0b01100000, 0b01100000, 0b01100000, 0b01100000,
    0b01100000, 0b01100010, 0b01100110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 77 0x4d 'M'
    0b00000000, 0b00000000, 0b11000110, 0b11101110, 0b11111110, 0b11111110, 0b11010110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 78 0x4e 'N'
    0b00000000, 0b00000000, 0b11000110, 0b11100110, 0b11110110, 0b11111110, 0b11011110, 0b11001110,
    0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 79 0x4f 'O'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 80 0x50 'P'
    0b00000000, 0b00000000, 0b11111100, 0b01100110, 0b01100110, 0b01100110, 0b01111100, 0b01100000,
    0b01100000, 0b01100000, 0b01100000, 0b11110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 81 0x51 'Q'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b11010110, 0b11011110, 0b01111100, 0b00001100, 0b00001110, 0b00000000, 0b00000000,
    // 82 0x52 'R'
    0b00000000, 0b00000000, 0b11111100, 0b01100110, 0b01100110, 0b01100110, 0b01111100, 0b01101100,
    0b01100110, 0b01100110, 0b01100110, 0b11100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 83 0x53 'S'
    0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110, 0b01100000, 0b00111000, 0b00001100,
    0b00000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 84 0x54 'T'
    0b00000000, 0b00000000, 0b01111110, 0b01111110, 0b01011010, 0b00011000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 85 0x55 'U'
    0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 86 0x56 'V'
    0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b01101100, 0b00111000, 0b00010000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 87 0x57 'W'
    0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110, 0b11000110, 0b11010110, 0b11010110,
    0b11010110, 0b11111110, 0b11101110, 0b01101100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 88 0x58 'X'
    0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b01101100, 0b01111100, 0b00111000, 0b00111000,
    0b01111100, 0b01101100, 0b11000110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 89 0x59 'Y'
    0b00000000, 0b00000000, 0b01100110, 0b01100110, 0b01100110, 0b01100110, 0b00111100, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 90 0x5a 'Z'
    0b00000000, 0b00000000, 0b11111110, 0b11000110, 0b10000110, 0b00001100, 0b00011000, 0b00110000,
    0b01100000, 0b11000010, 0b11000110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 91 0x5b '['
    0b00000000, 0b00000000, 0b00111100, 0b00110000, 0b00110000, 0b00110000, 0b00110000, 0b00110000,
    0b00110000, 0b00110000, 0b00110000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 92 0x5c '\'
    0b00000000, 0b00000000, 0b00000000, 0b10000000, 0b11000000, 0b11100000, 0b01110000, 0b00111000,
    0b00011100, 0b00001110, 0b00000110, 0b00000010, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 93 0x5d ']'
    0b00000000, 0b00000000, 0b00111100, 0b00001100, 0b00001100, 0b00001100, 0b00001100, 0b00001100,
    0b00001100, 0b00001100, 0b00001100, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 94 0x5e '^'
    0b00010000, 0b00111000, 0b01101100, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 95 0x5f '_'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b00000000, 0b00000000,
    // 96 0x60 '`'
    0b00000000, 0b00110000, 0b00011000, 0b00001100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 97 0x61 'a'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111000, 0b00001100, 0b01111100,
    0b11001100, 0b11001100, 0b11001100, 0b01110110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 98 0x62 'b'
    0b00000000, 0b00000000, 0b11100000, 0b01100000, 0b01100000, 0b01111000, 0b01101100, 0b01100110,
    0b01100110, 0b01100110, 0b01100110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 99 0x63 'c'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000000,
    0b11000000, 0b11000000, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 100 0x64 'd'
    0b00000000, 0b00000000, 0b00011100, 0b00001100, 0b00001100, 0b00111100, 0b01101100, 0b11001100,
    0b11001100, 0b11001100, 0b11001100, 0b01110110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 101 0x65 'e'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11111110,
    0b11000000, 0b11000000, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 102 0x66 'f'
    0b00000000, 0b00000000, 0b00011100, 0b00110110, 0b00110010, 0b00110000, 0b01111000, 0b00110000,
    0b00110000, 0b00110000, 0b00110000, 0b01111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 103 0x67 'g'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01110110, 0b11001100, 0b11001100,
    0b11001100, 0b11001100, 0b11001100, 0b01111100, 0b00001100, 0b11001100, 0b01111000, 0b00000000,
    // 104 0x68 'h'
    0b00000000, 0b00000000, 0b11100000, 0b01100000, 0b01100000, 0b01101100, 0b01110110, 0b01100110,
    0b01100110, 0b01100110, 0b01100110, 0b11100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 105 0x69 'i'
    0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b00000000, 0b00111000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 106 0x6a 'j'
    0b00000000, 0b00000000, 0b00000110, 0b00000110, 0b00000000, 0b00001110, 0b00000110, 0b00000110,
    0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b01100110, 0b01100110, 0b00111100, 0b00000000,
    // 107 0x6b 'k'
    0b00000000, 0b00000000, 0b11100000, 0b01100000, 0b01100000, 0b01100110, 0b01101100, 0b01111000,
    0b01111000, 0b01101100, 0b01100110, 0b11100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 108 0x6c 'l'
    0b00000000, 0b00000000, 0b00111000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 109 0x6d 'm'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11101100, 0b11111110, 0b11010110,
    0b11010110, 0b11010110, 0b11010110, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 110 0x6e 'n'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11011100, 0b01100110, 0b01100110,
    0b01100110, 0b01100110, 0b01100110, 0b01100110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 111 0x6f 'o'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 112 0x70 'p'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11011100, 0b01100110, 0b01100110,
    0b01100110, 0b01100110, 0b01100110, 0b01111100, 0b01100000, 0b01100000, 0b11110000, 0b00000000,
    // 113 0x71 'q'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01110110, 0b11001100, 0b11001100,
    0b11001100, 0b11001100, 0b11001100, 0b01111100, 0b00001100, 0b00001100, 0b00011110, 0b00000000,
    // 114 0x72 'r'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11011100, 0b01110110, 0b01100110,
    0b01100000, 0b01100000, 0b01100000, 0b11110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 115 0x73 's'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111100, 0b11000110, 0b01100000,
    0b00111000, 0b00001100, 0b11000110, 0b01111100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 116 0x74 't'
    0b00000000, 0b00000000, 0b00010000, 0b00110000, 0b00110000, 0b11111100, 0b00110000, 0b00110000,
    0b00110000, 0b00110000, 0b00110110, 0b00011100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 117 0x75 'u'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11001100, 0b11001100, 0b11001100,
    0b11001100, 0b11001100, 0b11001100, 0b01110110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 118 0x76 'v'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b11000110, 0b01101100, 0b00111000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 119 0x77 'w'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11010110,
    0b11010110, 0b11010110, 0b11111110, 0b01101100, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 120 0x78 'x'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11000110, 0b01101100, 0b00111000,
    0b00111000, 0b00111000, 0b01101100, 0b11000110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 121 0x79 'y'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11000110, 0b11000110, 0b11000110,
    0b11000110, 0b11000110, 0b11000110, 0b01111110, 0b00000110, 0b00001100, 0b11111000, 0b00000000,
    // 122 0x7a 'z'
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111110, 0b11001100, 0b00011000,
    0b00110000, 0b01100000, 0b11000110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 123 0x7b '{'
    0b00000000, 0b00000000, 0b00001110, 0b00011000, 0b00011000, 0b00011000, 0b01110000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00001110, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 124 0x7c '|'
    0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 125 0x7d '}'
    0b00000000, 0b00000000, 0b01110000, 0b00011000, 0b00011000, 0b00011000, 0b00001110, 0b00011000,
    0b00011000, 0b00011000, 0b00011000, 0b01110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 126 0x7e '~'
    0b00000000, 0b01110110, 0b11011100, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    // 127 0x7f ''
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00010000, 0b00111000, 0b01101100, 0b11000110,
    0b11000110, 0b11000110, 0b11111110, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
];