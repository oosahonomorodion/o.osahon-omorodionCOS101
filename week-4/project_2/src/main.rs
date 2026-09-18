    // Rust program to determine annual incentives for employees

    use std::io;

    fn main()
    {
        let mut input1 = String::new();
        let mut input2 = String::new();

       println!("Are you experienced? ");
       io::stdin().read_line(&mut input1).expect("Not a valid string");
       let experienced:String= input1.trim().parse().expect("Not a valid string");

       println!("Enter your age: ");
       io::stdin().read_line(&mut input2).expect("Not a valid string");
       let age:i64 = input2.trim().parse().expect("Not a valid number");

       if experienced == "yes"
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
       else if experienced == "no"
       {
           println!("Annual incentives is N100_000");
       }

    }