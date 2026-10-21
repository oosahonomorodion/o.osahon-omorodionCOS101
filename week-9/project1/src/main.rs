    use std::io::Write;

    fn main() {

       let categories = "Lager,Stout,Non-Alcoholic\n";
       let drinks_1 = "33 Export,Legend,Maltina\n";
       let drinks_2 = "Desperados,Turbo King,Amstel Maltina\n";
       let drinks_3 = "Goldberg,Williams,Malta Gold\n";
       let drinks_4 = "Gulder,,Fayrouz\n";
       let drinks_5 = "Heineken,,,\n";
       let drinks_6 = "Star,,,";

        let mut file = std::fs::File::create("nb.csv").expect("create failed"); 
        file.write_all("Categories of drinks produced in Nigerian Breweries\n"
            .as_bytes()).expect("write failed");
        file.write_all(categories.as_bytes()).expect("write failed");
        file.write_all(drinks_1.as_bytes()).expect("write failed");
        file.write_all(drinks_2.as_bytes()).expect("write failed");
        file.write_all(drinks_3.as_bytes()).expect("write failed");
        file.write_all(drinks_4.as_bytes()).expect("write failed");
        file.write_all(drinks_5.as_bytes()).expect("write failed");
        file.write_all(drinks_6.as_bytes()).expect("write failed");
        println!("\nData written to file." );                
    }