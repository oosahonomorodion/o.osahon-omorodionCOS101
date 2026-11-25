    use std::io::Read;

    fn administrator() {
       let mut file = std::fs::File::open("globacom_dbase.sql").expect("Failed to open file"); 
       let mut contents = String::new();
       file.read_to_string(&mut contents).expect("Failed to read input");
       print!("{}", contents);
           }  

    fn  project_manager() {
        let mut file = std::fs::File::open("project_tb.sql").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input");
        print!("{}", contents);
    }      

    fn employee() {
        let mut file = std::fs::File::open("staff_tb.sql").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input");
        print!("{}",contents);
    }

    fn customer() {
        let mut file = std::fs::File::open("customer_tb.sql").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input");
        print!("{}", contents);
    }

    fn vendor() {
        let mut file = std::fs::File::open("dataplan_tb.sql").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input");
        print!("{}", contents);
    }

    use std::io;

    fn main() {
        println!("Welcome to Globacom database \n");
        println!("Select your role \n");
        println!("-Administrator");
        println!("-Project Manager");
        println!("-Employee");
        println!("-Customer");
        println!("-Vendor");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let role = input.trim().to_string();

        if role == "Administrator" {
            administrator();
        } 
        else if role == "Project Manager" {
            project_manager();
        }
        else if role == "Employee" {
            employee();
        }
        else if role == "Customer" {
            customer();
        }
        else if role == "Vendor" {
            vendor();
        }
        else {
            println!("Not a role in this business, contact administrator");
        }

    }