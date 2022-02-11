fn main() {
    flow();
    loop_fn();

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

    let y = {
        //評価値を返すものを"式"と呼ぶ
        let x = 3;
        x + 1
    };

    println!("y : {}", y);

    let x = five();

    println!("x : {}", x);

    let x = plus_one(x);

    println!("x : {}", x);
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

    let (alpha, beta, gamma) = _tup;

    println!("alpha : {}, beta : {}, gamma : {}", alpha, beta, gamma);

    let array = [1, 2, 3, 4, 5];

    let first = array[0];

    println!("array[0] : {}", first);
}

fn five() -> i32 {
    // i32は戻り値の型指定
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //文末に ; があると文として処理され、戻り値が得られない
}

fn flow() {
    let number = 7;

    if number < 5 {
        println!("条件はTrue");
    } else {
        println!("条件はFalse");
    }

    let number = 3;

    if number != 0 {
        println!("数字は0以外のなにかです");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // ifは式なので右辺に入れられる

    println!("number : {}", number);
}

fn loop_fn() {
    loop {
        println!("again!");
        break;
    }

    let mut number =3;

    while number !=0 {
        println!("{}!",number);

        number = number-1;
        
    }
    println!("発射!!");

    let array = [10,20,30,40,50];

    for element in array.iter(){
        println!("数字は{}",element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
