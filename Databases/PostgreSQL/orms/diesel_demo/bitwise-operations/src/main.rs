fn main() {
    let x: u8 = 15;
    let y: u8 = 'H' as u8; // ASCII value of 'H'
    // aligns each column to the left and sets fixed widths (9, 8, and 10 characters, respectively)
    // for neat output. This prepares the console for displaying variable names, their values,
    // and their binary representations in aligned columns.
    println!("{:<9}{:<8}{:<10}", "Variable", "Value", "Binary");
    // The format specifiers :-<9, :-<8, and :-<10 left-align the dashes and set fixed widths
    // for each column, matching the headers above. This helps keep the output neat and readable.
    println!("{:-<9}{:-<8}{:-<10}", "", "", "");
    println!("{:<9}{:<8}{:<10}", " x ", x, format!("{:08b}", x));
    println!("{:<9}{:<8}{:<10}", " y ", y, format!("{:08b}", y));
    println!("{:-<9}{:-<8}{:-<10}", "", "", "");
    println!("{:<9}{:<8}{:<10}", "x&y", x & y, format!("{:08b}", x & y));
    println!("{:<9}{:<8}{:<10}", "x|y", x | y, format!("{:08b}", x | y));
    println!("{:<9}{:<8}{:<10}", "x^y", x ^ y, format!("{:08b}", x ^ y));
    println!("{:<9}{:<8}{:<10}", "~x", !x, format!("{:08b}", !x & 0xFF));
    println!(
        "{:<9}{:<8}{:<10}",
        "x<<1",
        x << 1,
        format!("{:08b}", x << 1)
    );
    println!(
        "{:<9}{:<8}{:<10}",
        "x>>1",
        x >> 1,
        format!("{:08b}", x >> 1)
    );
}
