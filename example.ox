let LOOK_LEN = 9;
let mut out = "";

fn to_number(x: Number) {
    for i in 0..LOOK_LEN {
        let i = LOOK_LEN - 1 - i;

        let pos_value = 1;
        for j in 0..i {
            pos_value = pos_value * 10;
        };

        let shifted = x / pos_value;

        let num = shifted % 10;
        let d_from_floor = num % 1;
        let floored = num - d_from_floor;

        if num == 0 { out += "0"; };
        if num == 1 { out += "1"; };
        if num == 2 { out += "2"; };
        if num == 3 { out += "3"; };
        if num == 4 { out += "4"; };
        if num == 5 { out += "5"; };
        if num == 6 { out += "6"; };
        if num == 7 { out += "7"; };
        if num == 8 { out += "8"; };
        if num == 9 { out += "9"; };
    }
}

to_number(342323);
print(out);
