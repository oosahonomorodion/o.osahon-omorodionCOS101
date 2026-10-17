    fn main() {

        let candidates = vec![("Joan Obi", 8),
                              ("Mary Ota", 2),
                              ("Timons Law", 9),
                              ("Lawrence Daniel", 10),
                              ("Richard Daniels", 6),
                              ("Tony Adams", 2),
                              ("Chidi Edeh", 1),
                              ("Ikolo Emmanuel", 8),
                              ("Deji Adeyinka", 10),
                              ("Osagie Daniel", 16)];

        let mut highest = candidates[0].1;
        let mut person = candidates[0].0;
        
        for i in 0..candidates.len() {
            if candidates[i].1 > highest {
                highest = candidates[i].1;
                person = candidates[i].0;

            println!("{} has the most experience with {} years", candidates[i].0, candidates[i].1);

            }

        }      
        
    }