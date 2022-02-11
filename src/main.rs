fn main() {
    const MAX_POINTS: u32 = 100_000;

    var_test();

    let mut x = 5;
    println!("xの数字は{}", x);
    x = 6;
    println!("xの数字は{}", x);

    println!("{}", MAX_POINTS);

    let z = 5;

    let z = z + 1;

    let z = z * 2;

    println!("zの値は{}", z);

    let spaces = "           "; //変数は不変のまま

    let spaces = spaces.len(); //変数は不変のまま変更を加えられる

    println!("{}", spaces);
}

fn var_test() {
    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _deffrence = 95.5 - 4.3;
    let _product = 4 * 32;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let _c = "z";
    let _char = "≧";
    let _emoji = "😻";

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (alpha,beta,gamma) = _tup;

    println!("beta : {}",beta);

    let array = [1, 2, 3, 4, 5];

    let first = array[0];

    println!("array[0] : {}",first);
}
