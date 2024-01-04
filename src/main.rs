use std::io;

fn main() {
    println!("please enter your number: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = num.trim().parse().expect("Not a number");

    if num < 1 {
        println!("Enter a number bigger than 0");
        return;
    }

    fn gen_fibo(n: u32) -> u32 {
        if n == 1 {
            let nth:u32 = 0;
            nth
        } else if n == 2 {
            let nth:u32 = 1;
            nth
        } else {
            let nth: u32 = gen_fibo(n - 1) + gen_fibo(n - 2);
            nth
        }
    }    

    let gen_nth = gen_fibo(num);

    println!("The answer is {gen_nth}");
}

