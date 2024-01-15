let x = 0;
let y = 1;
let z = 0;

for i in 0..6 {
    z = x + y;
    println(z);

    x = y;
    y = z;
};
