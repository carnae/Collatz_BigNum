use malachite::Natural;
use malachite::num::arithmetic::traits::DivisibleBy;
use std::time::{Instant};
use std::str::FromStr;
use malachite::num::basic::traits::{One,Two};
use std::fs::File;
use std::io::Write;


fn main() {
    let mut Hailstones = File::create("Hailstones.txt").expect("creation failed");
    let mut low_bound = Natural::from_str("2361183241434822606848").unwrap(); //variable declaration
    let up_bound = Natural::from_str("2361183241434823606848").unwrap();
    let f3 = Natural::from_str("3").unwrap();
    let mut x = Natural::from_str("0").unwrap();
    let start = Instant::now(); //timer start   


    while low_bound != up_bound {

        x =  low_bound.clone();
        //println!("{x}");

        while x != Natural::ONE{
            Hailstones.write(x.to_string().as_bytes()).expect("write failed");
            Hailstones.write(" ".as_bytes()).expect("write failed");

            if (&x).divisible_by(&Natural::TWO) == true{

                x = x / &Natural::TWO

            }

            else{

                x = (&x * &f3) + &Natural::ONE;

            }

        }
        Hailstones.write("\n".as_bytes()).expect("write failed");
        low_bound = &low_bound + Natural::ONE;
    }
    let duration = start.elapsed(); //timer stop

    println!("Time elapsed in expensive_function() is: {:?}", duration); //timer print 
    
}

