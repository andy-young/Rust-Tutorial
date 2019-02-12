use std::{i8, i16, i32, i64, u8, u32, u64, isize, usize, f32, f64 };

use std::io::stdin;

fn main() {

    // Ownership, Pointers, and why we use references =========================
    // One binding for each resource (anything not a primitive).

    // let vect1 = vec![1,2,3]; <-- will not work, but will with primitives
    // let vect2 = vect1;

    // println!("vect1[0] : {}", vect1[0]);

    let prim_val = 1;
    let prim_val2 = prim_val;

    println!("prim_val : {}", prim_val);

    // See more below...

    // Closures =================================================================

    // let sum_nums = |x: i32, y: i32| x + y;
    // println!("7 + 8 = {}", sum_nums(7, 8));

    // let num_ten = 10;

    // let add_10 = |x: i32| x + num_ten;
    // println!("5 + 10 = {}", add_10(5));

    // Functions ================================================================

    // Go out of main()...

    // Tuples ===================================================================
    // Fixed sized lists of many types, uses keys vs indices.

    // let rand_tuple = ("Andy", 39);

    // let rand_tuple_2: (&str, i8) = ("Andy", 39);

    // println!("Name : {}", rand_tuple_2.0);

  // Vectors ==================================================================

    // let mut vect1 = vec![1,2,3,4,5];

    // println!("Item 2 : {}", vect1[1]);

    // for i in &vect1 {
    //     println!("Vect: {}", i * 2);
    // }
    // vect1.push(6);
    // vect1.pop();

    // Arrays ===================================================================

    // let rand_array = [1, 2, 3];

    // println!("{}", rand_array[0]);

    // println!("{}", rand_array.len());

    // println!("Second 2 : {:?}", &rand_array[2..3]);

    // Number Guesser Game ======================================================

//   'outer: loop {
//       let number: i32 = 10;
//       println!("Pick a number");

//       loop {
//           let mut line = String::new();

//           let input = stdin().read_line(&mut line);

//           let guess: Option<i32> = input.ok().map_or(None, |_|
//           line.trim().parse().ok());

//           match guess {
//               None => println!("Enter a Number"),
//               Some(n) if n == number => {
//                   println!("You Guessed it");
//                   break 'outer;
//               }
//               Some(n) if n < number =>
//               println!("Too Low"),
//               Some(n) if n > number =>
//               println!("Too High"),
//               Some(_) => println!("Error")
//           }
//       }
//   }

    // Strings ================================================================

    // let rand_string = "I am a random string";

    // println!("Length : {}", rand_string.len());

    // let (first, second) = rand_string.split_at(6);
    // println!("First : {} Second : {}", first, second);

    // let count = rand_string.chars().count();
    // let mut chars = rand_string.chars();

    // let mut indiv_char = chars.next();

    // loop {
    //     match indiv_char {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     indiv_char = chars.next();
    // }

    // let mut iter = rand_string.split_whitespace();

    // let mut indiv_word = iter.next();

    // loop {
    //     match indiv_word {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     indiv_word = iter.next();
    // }

    // let rand_string2 = "I am a random string\nThere are other strings like it\nThis string is the best.";

    // let mut lines = rand_string2.lines();
    // let mut indiv_line = lines.next();

    // loop {
    //     match indiv_line {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    //     indiv_line = lines.next();
    // }

    // println!("Find Best : {}", rand_string2.contains("best"));


    // loop ===================================================================

    // let mut x = 1;

    // loop {
    //     if ((x % 2) == 0) {
    //         println!("{}", x);
    //         x += 1;
    //         continue;
    //     }
    //     if (x > 10) {
    //         break;
    //     }
    //     x += 1;
    //     continue;
    // }

    // while loop =============================================================

    // let mut y = 1;

    // while y <= 10{
    //     println!("{}", y);
    //     y += 1;
    // }

    // for loop ===============================================================

    // for z in 1..10 {
    //     println!("FOR : {}", z);
    // }

    // Conditionals ===========================================================

    // let age_old = 6;

    // if (age_old == 5) {
    //     println!("Go to kindergarten");
    // } else if (age_old > 5) && (age_old <= 18) {
    //     println!("Go to grade {}", (age_old - 5));
    // } else if (age_old <= 25) && (age_old > 18) {
    //     println!("Go to College");
    // } else {
    //     println!("Get a job.");
    // }

    // println!("!true = {}", !true);

    // println!("true || false = {}", true || false);

    // println!("false || true = {}", false || true);

    // println!("true && true = {}", true && true);

    // println!("true != false : {}", true != false);

    // // homemade ternary operator:
    // let can_vote = if (age_old >= 18) {true} else {false};

    // println!("Can Vote : {}", can_vote);

    // Math ===================================================================

    // let mut neg_4 = -4i32;

    // println!("abs(-4) = {}", neg_4.abs());
    // println!("4 ^ 6 = {}", 4i32.pow(6));

    // println!("sqrt 9 = {}", 9f64.sqrt());
    // println!("cbrt 27 = {}", 27f64.cbrt());

    // println!("Round 1.45 = {}", 1.45f64.round());
    // println!("Floor 1.45 = {}", 1.45fi64.floor());

    // println!("Ceiling 1.45 = {}", 1.45f64.ceil());
    // println!("e ^ 2 = {}", 2f64.exp());

    // println!("log(2) = {}", 2f64.ln());
    // println!("log10(2) = {}", 2f64.log10());

    // println!("90 to radians = {}", 90f64.to_radians());
    // println!("PI to Degrees = {}", 3.14f64.to_degrees());

    // println!("Max 4, 5 = {}", 4f64.max(5f64));
    // println!("Min 4, 5 = {}", 4f64.min(5f64));

    // println!("Sin 3.14 = {}", 3.14f64.sin());
    // println!("Cosine 3.14 = {}", 3.14f64.cos());

    // println!("5 + 4 = {}", 5 + 4);
    // println!("5 - 4 = {}", 5 - 4);
    // println!("5 * 4 = {}", 5 * 4);
    // println!("5 / 4 = {}", 5 / 4);
    // println!("5 % 4 = {}", 5 % 4);


    // let is_it_true: bool = true;

    // let let_x: char = 'x';

    // println!("It is {0} that {1} is {0}", is_it_true, let_x);

    // println!("{:.2}", 1.234);

    // println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    // println!("{ten:>ws$}", ten=10, ws=5);
    // println!("{ten:>0ws$}", ten=10, ws=5);

    // let num = 10;

    // let mut age: i32 = 39;


    // println!("I am {} years old", age);

    // let (f_name, l_name) = ("Andy", "Young");

    // println!("Max i8 {}", i8::MAX);
    // println!("Min i8 {}", i8::MIN);

    // println!("Max i16 {}", i16::MAX);
    // println!("Min i16 {}", i16::MIN);

    // println!("Max i32 {}", i32::MAX);
    // println!("Min i32 {}", i32::MIN);

    // println!("Max i64 {}", i64::MAX);
    // println!("Min i64 {}", i64::MIN);

    // println!("Max isize {}", isize::MAX);
    // println!("Min isize {}", isize::MIN);

    // println!("Max usize {}", usize::MAX);
    // println!("Min usize {}", usize::MIN);

    // println!("MAX f32 {}", f32::MAX);
    // println!("MIN f32 {}", f32::MIN);

    // println!("MAX f64 {}", f64::MAX);
    // println!("MIN f64 {}", f64::MIN);

    // Functions Continued ======================================================
    // say_hello("Andy");

    // println!("5 + 4 = {}", get_sum(5, 4));

    // let sum = get_sum;
    // println!("6 + 4 = {}", sum(6, 4));
}

    // fn say_hello(name: &str) {
    //     println!("Hello {}", name);
    // }

    // fn get_sum(num1: i32, num2: i32) -> i32 {
    //     num1 + num2
    // }

    // Functions End ============================================================

    // Ownership continued ======================================================

    fn sum_vects(v1: & Vec<i32>) -> i32 {
        let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
        return sum;
    }



