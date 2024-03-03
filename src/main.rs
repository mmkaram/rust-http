use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        loop {
            let mut l = String::new();
            rdr.read_line(&mut l).unwrap();
            if l.trim().is_empty() {
                break;
            }
            print!("{l}");
        }
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
    }
}

fn fibonacci(n: u32) -> Vec<u64> {
    let mut sequence= Vec::new();
    
    // Handle base cases
    if n >= 1 {
        sequence.push(0);
    }
    if n >= 2 {
        sequence.push(1);
    }

    // Generate Fibonacci sequence up to the nth number
    for i in 2..n {
        let next_number = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next_number);
    }

    println!("{:?}", sequence);
    sequence
}


#[cfg(test)]
mod http_tests {
    use crate::fibonacci;

    #[test]
    fn is_two_equal_to_two () {
        assert_eq!(2, 2);
    }
    
    #[test]
    fn is_fibo_correct(){
        assert_eq!(fibonacci(7), [0, 1, 1, 2, 3, 5, 8])
    }
}