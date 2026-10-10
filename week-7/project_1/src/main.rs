        use std::io;

    fn trapezium() ->f64 {
        let mut input = String::new();
        println!("Enter height");
        io::stdin().read_line(&mut input).expect("Invalid number");
        let height:f64 = input.trim().parse().expect("Failed to read a valid input");

        let mut input1 = String::new();
        println!("Enter base1");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let base1:f64 = input1.trim().parse().expect("Failed to read input");

        let mut input2 = String::new();
        println!("Enter base2");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let base2:f64 = input2.trim().parse().expect("Failed to read input");

        let area:f64 = height / 2.0 * (base1 + base2);
        return area;
    } 

    fn rhombus() ->f64 {
        let mut input = String::new();
        println!("Enter first diagonal");
        io::stdin().read_line(&mut input).expect("Invalid input");
        let diagonal1:f64 = input.trim().parse().expect("Failed to read input");

        let mut input1 = String::new();
        println!("Enter second diagonal");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let diagonal2:f64 = input1.trim().parse().expect("Failed to read input");

        let area:f64 = 0.5 * diagonal1 * diagonal2;
        return area;
    }

    fn parallelogram() ->f64 {
        let mut input = String::new();
        println!("Enter base");
        io::stdin().read_line(&mut input).expect("Invalid input");
        let base:f64 = input.trim().parse().expect("Failed to read input");

        let mut input1 = String::new();
        println!("Enter altitude");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let altitude:f64 = input1.trim().parse().expect("Failed to read input");

        let area:f64 = base * altitude;
        return area;
    }

    fn cube() ->f64 {
        let mut input = String::new();
        println!("Enter side");
        io::stdin().read_line(&mut input).expect("Invalid input");
        let side:f64 = input.trim().parse().expect("Failed to read input");

        let surface_area:f64 = 6.0 * side * side;
        return surface_area;
    }

    fn cylinder() ->f64 {
        let mut input = String::new();
        println!("Enter radius");
        io::stdin().read_line(&mut input).expect("Invalid input");
        let radius:f64 = input.trim().parse().expect("Failed to read input");
        
        let mut input1 = String::new();
        println!("Enter height");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let height:f64 = input1.trim().parse().expect("Failed to read input");

        let volume:f64 = (22.0 / 7.0) * radius * radius * height;
        return volume;
    }

    fn main() {
       println!("SHAPE CALCULATOR");
       println!("1 - Trapezium");
       println!("2 - Rhombus");
       println!("3 - Parallelogram");
       println!("4 - Cube");
       println!("5 - Cylinder");

       println!("Enter your choice:");
       let mut input = String::new();
       io::stdin().read_line(&mut input).expect("Invalid input");
       let choice:i32 = input.trim().parse().expect("Failed to read input");
       
       if choice == 1 {
        println!("Area of trapezium: {:?}", trapezium());
       }

       else if choice == 2 {
        println!("Area of Rhombus: {:?}", rhombus());
       } 

       else if choice == 3 {
        println!("Area of parallelogram: {:?}", parallelogram());
       }

       else if choice == 4 {
        println!("Area of cube: {:?}", cube());
       }

       else if choice == 5 {
        println!("Area of cylinder: {:?}", cylinder());
       }
       else {
        println!("Not a valid input");
       } 
       
    }
        

        
    




