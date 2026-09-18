    // Rust program to determine annual incentives for employees

    use std::io;

    fn main()
    {
        let mut experienced = String::new();
        let mut input1 = String::new();

       println!("Are you experienced? (yes/no)");
       io::stdin().read_line(&mut experienced).expect("Not a valid string");

       println!("Enter your age: ");
       io::stdin().read_line(&mut input1).expect("Not a valid string");
       let age:i64 = input1.trim().parse().expect("Not a valid number");

       if experienced.trim() == "yes"
       {
       if age >= 40 {
           println!("Annual incentives is N1_560_000");
       }
       else if age >= 30 && age <= 39 {
           println!("Annual incentives is N1_480_000");
       }
       else if age < 28 {
           println!("Annual incentives is N1_300_000");
       }
       }
       else
       {
           println!("Annual incentives is N100_000");
       }

    }