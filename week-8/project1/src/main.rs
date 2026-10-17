    use std::io;

    fn main() {
        let v = vec![("Office admin", "APS 1-2", "Intern"),
                    ("Office admin", "APS 3-5", "Administrator"),
                    ("Office admin", "APS 5-8", "Senior Administrator"),
                    ("Office admin", "EL1 8-10", "Office Manager"),
                    ("Office admin", "EL2 10-13", "Director"),
                    ("Office admin", "SES", "CEO"),

                    ("Academic", "APS 1-2", "-"),
                    ("Academic", "APS 3-5", "Research Assistant"),
                    ("Academic", "APS 5-8", "PhD Candidate"),
                    ("Academic", "EL1 8-10", "Post-Doc Researcher"),
                    ("Academic", "EL2 10-13", "Senior Lecturer"),
                    ("Academic", "SES", "Dean"),

                    ("Lawyer", "APS 1-2", "Paralegal"),
                    ("Lawyer", "APS 3-5", "Junior Associate"),
                    ("Lawyer", "APS 5-8", "Associate"),
                    ("Lawyer", "EL1 8-10", "Senior Associate 1-2"),
                    ("Lawyer", "EL2 10-13", "Senior Associate 3-4"),
                    ("Lawyer", "SES", "Partner"),

                    ("Teacher", "APS 1-2", "Placement"),
                    ("Teacher", "APS 3-5", "Classroom teacher"),
                    ("Teacher", "APS 5-8", "Senior teacher"),
                    ("Teacher", "EL1 8-10", "Leading teacher"),
                    ("Teacher", "EL2 10-13", "Deputy Principal"),
                    ("Teacher", "SES", "Principal")];

        let mut input1 = String::new();
        println!("Enter your Profession");
        io::stdin().read_line(&mut input1).expect("Invalid input");
        let profession:String = input1.trim().to_string();

        let mut input2 = String::new();
        println!("Enter APS Level");
        io::stdin().read_line(&mut input2).expect("Not a valid input");
        let aps_level:String = input2.trim().to_string();

        for val in &v {
            if val.0 == profession && val.1 == aps_level 
            { println!("{}", val.2);}
        } 

    }