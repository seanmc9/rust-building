use std::env;

fn factorial(num: u32) -> u32 {
    if num == 1 { return 1; }
    return num * factorial(num - 1);
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let num: u32 = match args[1].trim().parse() {
			Ok(num) => num,
                        Err(_) => todo!(),
                };
    println!("{}", factorial(num));
}
