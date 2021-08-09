fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let fx = 2.0; // f64
    let fy: f32 = 3.0; // f32

    // 足し算
    let sum = 5 + 10;

    // 引き算
    let difference = 95.5 - 4.3;

    // 掛け算
    let product = 4 * 30;

    // 割り算
    let quotient = 56.7 / 32.2;

    // 余り
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // 明示的型注釈付きで

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let arr = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let arr2 = [1, 2, 3, 4, 5];

    let first = arr2[0];
    let second = arr2[1];

    // let b = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = b[index];

    // println!("The value of element is: {}", element);   // 要素の値は{}です
}
