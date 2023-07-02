use std::collections::HashMap;

pub fn largest_good_integer(num: String) -> String {
      // Step 1: If the length is less than 3, return an empty string
      if num.len() < 3 {
        return "".to_string();
    }
    
    // Step 2: Initialize max_good_integer to empty string
    let mut max_good_integer = "".to_string();

    // Step 3: Iterate through the string
    let bytes = num.as_bytes();
    for i in 0..bytes.len() - 2 {
        // Step 4: Check if the substring consists of the same character repeated 3 times
        if bytes[i] == bytes[i + 1] && bytes[i] == bytes[i + 2] {
            // Compare the substring lexicographically with max_good_integer
            let good_integer = num[i..i+3].to_string();
            if good_integer > max_good_integer {
                max_good_integer = good_integer;
            }
        }
    }
    
    // Step 5: Return the result
    max_good_integer
}



//Dosent work correctly cause it dosent check for in row and just checks for 3 of the same number also less efficent
// pub fn largest_good_integer1(num: String) -> String {
//     if num.len() < 3 {
//         return "".to_string();
//     }
//     let mut has_map: HashMap<char, u32> = HashMap::new();

//     for char in num.chars() {
//         if has_map.contains_key(&char) {
//             let count = has_map.get(&char).unwrap();
//             has_map.insert(char, count + 1);
//         } else {
//             has_map.insert(char, 1);
//         }
//     }
//     println!("{:?}", has_map);
//     let mut high_est =  -1;
//     for (x,y) in has_map.iter() {
//         if *y == 3 {
//             if x.to_digit(10).unwrap() as i32 > high_est  {
//                 high_est = x.to_digit(10).unwrap() as i32;
//             }
//         }
//     }
//     if high_est != -1 {
//         return high_est.to_string().repeat(3)
//     }

//     return  "".to_string();

// }