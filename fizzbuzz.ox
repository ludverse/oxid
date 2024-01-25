let char_pos_out = 0;
fn char_pos(c: String) {
    if c == "0" { char_pos_out = 0; };
    if c == "1" { char_pos_out = 1; };
    if c == "2" { char_pos_out = 2; };
    if c == "3" { char_pos_out = 3; };
    if c == "4" { char_pos_out = 4; };
    if c == "5" { char_pos_out = 5; };
    if c == "6" { char_pos_out = 6; };
    if c == "7" { char_pos_out = 7; };
    if c == "8" { char_pos_out = 8; };
    if c == "9" { char_pos_out = 9; };
}

let mut cmp_out = true;
fn cmp(a: String, b: String) {
    char_pos(a);
    let a_pos = char_pos_out;

    char_pos(a);
    let b_pos = char_pos_out;


}

let mut sort_out = "";
fn sort(inp: String, len: Number) {
    for i in 0..len {
        sort_out = index_out;
    }
}

sort("51823", 5);
print(sort_out);
