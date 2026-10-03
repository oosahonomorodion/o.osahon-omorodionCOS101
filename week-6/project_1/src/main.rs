    use std::io;

    fn main() {
       
       let p = " P - Poundo Yam/ Edinkaiko Soup - N3200";
       let f = " F - Fried Rice and Chicken - N3000";
       let a = " A - Amala & Ewedu Soup - N2500";
       let e = " E - Eba & Egusi Soup - N2000";
       let w = " W - White Rice & Stew - N2500";
       
       let mut input1 = String::new();

       println!("Hi there valued customer, what will you like to order today?");
       println!("Our today's special dishes are: 
                 {}
                 {}
                 {}
                 {}
                 {}", p,f,a,e,w);
       io::stdin().read_line(&mut input1).expect("Not a valid string");
             
       let mut input2 = String::new();

       println!("Ok Great! How much quantity do you want?");
       io::stdin().read_line(&mut input2).expect("Not a valid string");
       let quantity:i32 = input2.trim().parse().expect("Not a valid number");
  
       // Price of the food in the menu
      let mut price:i32 = 0;
      match input1.trim().to_lowercase().as_str() {
           "p" | "poundo yam/ edinkaiko soup" => price = 3200,
           "f" | "fried rice and chicken" => price = 3000,
           "a" | "amala & ewedu soup" => price = 2500,
           "e" | "eba & egusi soup" => price = 2000,
           "w" | "white rice & stew" => price = 2500,
           _ => println!("Invalid selection"), 
          }

      let _totalcharge:i32 = quantity * price;
      if _totalcharge > 10000 {
            let _total_charge = ( _totalcharge * 95) / 100;
            println!("Total Cost is: {}", _total_charge);
           }
           else {
            println!("Total Cost is: {}", _totalcharge); 
         }

         println!("Enjoy your meal Sir/Ma");

     }