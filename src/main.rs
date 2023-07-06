fn main() {    
    let mut x: i32 = 1;
    let result = loop {
        println!("{x}");
        x += 1;

        if x == 10 {
            break x * 2;
        };

    };
    println!("The result is {result}.");

}