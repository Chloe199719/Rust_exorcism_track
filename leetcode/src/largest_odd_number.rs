
    pub fn largest_odd_number(num: String) -> String {
        let mut String: String = String::new();
        let mut push_all = false;
        for i in num.chars().rev(){
            if push_all{
                String.push(i);
                continue;
            }
            if i.to_digit(10).unwrap() % 2 != 0{
                String.push(i);
                push_all = true;
            }
        };

        
       String.chars().rev().collect::<String>()
    
}