use rug::{Float, Integer};

fn factorial(number:Integer) {
    new_factorial = Integer::new();
    for k in 1..(n+1) {
        new_factorial *= k;
    }
    return new_factorial;
}

fn main() {
    // let mut k = Integer::new();

    // println!("Welcome to Pi Calculator");

    // loop {
    //     let mut numerator = Float::new(10);
    //     let mut denominator = Float::new(10);
    //     let mut pi = Float::with_val(1000,0);

    //     let numerator = (-1).pow(k) * (factorial(6 * k)) * (545140134 * k + 13591409);
    //     let denominator = (factorial(3 * k)) * ((factorial(k)).pow(3)) * ((640320).pow(3*k));

    //     pi += numerator / denominator;

    //     k += 1;
    //     if k == 1000 {
    //         break
    //     }
    // }
    factorial(5)
}
