    // Rust program to find roots of quadratic equation

    use std::io;

    fn main()
    {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();

        println!("Enter a: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        println!("Enter b: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let b:f64 = input2.trim().parse().expect("Not a valid number");

        println!("Enter c: ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let c:f64 = input3.trim().parse().expect("Not a valid number");

        let d = b*b - 4.0*a*c;

        if d > 0.0
        {
            println!("Two distinct roots");
        }
        else if d == 0.0
        {
            println!("Exactly one real root");
        }
        else if d < 0.0
        {
            println!("No real root");
        }
    }