
for i in 0..50 {
    let mut out = "";

    if i % 3 == 0 { out += "Fizz" };
    if i % 5 == 0 { out += "Buzz" };

    if !out { out = i.to_string() };

    $process.stdout.write(out);
};

