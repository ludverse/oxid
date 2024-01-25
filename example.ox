fn to_number(x) {
    for i in 0..9 {
        let i = 2 - i;
        let number_value = 1;
        for j in 0..i {
            number_value = number_value * 10;
        };

        let shifted = x / number_value;

        let num = shifted % 10;
        let decimal = num % 1;
        let num = num - decimal;

        if num == 0 { out += "0" };
        if num == 1 { out += "1" };
        if num == 2 { out += "2" };
        if num == 3 { out += "3" };
        if num == 4 { out += "4" };
        if num == 5 { out += "5" };
        if num == 6 { out += "6" };
        if num == 7 { out += "7" };
        if num == 8 { out += "8" };
        if num == 9 { out += "9" };
    }
}

print(out)
