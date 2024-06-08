use std::io;

fn main() {

    let mut fib: Vec<u128> = vec![0, 1];
    
    'main: loop {
        println!("What fibonacci number do you want to see?");

        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed read");

        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("What is wrong with you");
                return
            }
        };
                
        for i in (fib.len() - 1)..n {
            match fib.get(n) {
                None => { },
                Some(_) => break
            }
            
            let a = match fib[i as usize].checked_add(fib[(i - 1) as usize]) {
                None => {
                    println!("Sorry, too big!");
                    break 'main
                },
                Some(x) => x
            };
            fib.push(a);
        }

        let rtn = fib[n];

        println!("Here's your fibonacci number: {}", rtn);
    }
}
