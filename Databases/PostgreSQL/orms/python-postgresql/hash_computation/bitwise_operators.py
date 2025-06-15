if __name__ == "__main__":
    x = 15
    # returns the Unicode code point (integer representation) of a given character. 
    # For example, ord('H') returns 72, which is the ASCII/Unicode value for the character H.
    y = ord('H')  # ASCII value of 'H'
    # :<9 left-aligns "Variable" in a 9-character wide field.
    print(f"{'Variable':<9}{'Value':<8}{'Binary':<10}")
    print(f"{'-' * 9}{'-' * 8}{'-' * 10}")
    print(f"{' x ':<9}{x:<8}{format(x, '08b'):<10}")
    print(f"{' y ':<9}{y:<8}{format(y, '08b'):<10}")
    print(f"{'-' * 9}{'-' * 8}{'-' * 10}")
    # Bitwise operations
    # AND operation (&) compares each bit of the first operand to the corresponding bit of the second operand.
    print(f"{'x&y':<9}{x & y:<8}{format(x & y, '08b'):<10}")
    # OR operation (|) compares each bit of the first operand to the corresponding bit of the second operand.
    print(f"{'x|y':<9}{x | y:<8}{format(x | y, '08b'):<10}")
    # XOR operation (^) compares each bit of the first operand to the corresponding bit of the second operand.
    print(f"{'x^y':<9}{x ^ y:<8}{format(x ^ y, '08b'):<10}")
    # NOT operation (~) inverts all bits of the operand.
    print(f"{'~x':<9}{~x:<8}{format(~x & 0xFF, '08b'):<10}")  # Masking with 0xFF to show only the last 8 bits
    # Left shift operation (<<) shifts bits to the left, filling with zeros on the right.
    print(f"{'x<<1':<9}{x << 1:<8}{format(x << 1, '08b'):<10}")
    # Right shift operation (>>) shifts bits to the right, filling with zeros on the left.
    print(f"{'x>>1':<9}{x >> 1:<8}{format(x >> 1, '08b'):<10}")
