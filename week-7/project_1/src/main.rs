    use std::io;

const PI: f64 = std::f64::consts::PI;

// Read a number from the user
fn read_number(message: &str) -> f64 {
    loop {
        println!("{}", message);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(number) if number.is_finite() && number >= 0.0 => {
                return number;
            }
            _ => {
                println!("Please enter a valid non-negative number.");
            }
        }
    }
}

// Calculate the area of a trapezium
fn trapezium_area() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter first base:");
    let base2 = read_number("Enter second base:");

    height / 2.0 * (base1 + base2)
}

// Calculate the area of a rhombus
fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter first diagonal:");
    let diagonal2 = read_number("Enter second diagonal:");

    0.5 * diagonal1 * diagonal2
}

// Calculate the area of a parallelogram
fn parallelogram_area() -> f64 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude:");

    base * altitude
}

// Calculate the surface area of a cube
fn cube_surface_area() -> f64 {
    let side = read_number("Enter side length:");

    6.0 * side * side
}

// Calculate the volume of a cylinder
fn cylinder_volume() -> f64 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");

    PI * radius * radius * height
}

fn main() {
    println!("===== SHAPE CALCULATOR =====");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");

    let choice = read_number("Enter your choice (1-5):");

    let result = match choice {
        1.0 => {
            println!("You selected Trapezium Area.");
            trapezium_area()
        }
        2.0 => {
            println!("You selected Rhombus Area.");
            rhombus_area()
        }
        3.0 => {
            println!("You selected Parallelogram Area.");
            parallelogram_area()
        }
        4.0 => {
            println!("You selected Cube Surface Area.");
            cube_surface_area()
        }
        5.0 => {
            println!("You selected Cylinder Volume.");
            cylinder_volume()
        }
        _ => {
            println!("Invalid choice. Please choose a number from 1 to 5.");
            return;
        }
    };

    println!("The answer is: {:.2}", result);
}

