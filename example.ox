
for i in 1..31 {
    let out = "";

    if i % 3 == 0 { out += "Fizz" };
    if i % 5 == 0 { out += "Buzz" };

    if out == "" { out = i };

    println(out);
}

