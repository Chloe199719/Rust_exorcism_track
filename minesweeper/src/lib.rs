pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),            (0, 1),
        (1, -1),  (1, 0),   (1, 1),
    ];
    let mut result = vec![];
    let rows: i32 = minefield.len() as i32;
    let cols: i32 = minefield[0].len() as i32;
    for (i, row) in minefield.iter().enumerate() {
        
        let mut  annotate_row =  String::new();
        for (j, ch) in row.chars().enumerate() {
            if ch == '*' {
            
                annotate_row.push(ch);
           
                continue;
            }
            let mut count = 0;
            for (x, y) in offsets.iter() {
                let x = i as i32 + x;
                let y = j as i32 + y;
                if x >= 0 && x < rows && y >= 0 && y < cols {
                    if minefield[x as usize].chars().nth(y as usize).unwrap() == '*' {
                        count += 1;
                    }
                }
                
            }
            if count > 0 {
                annotate_row.push_str(&count.to_string());
            } else {
                annotate_row.push( ' ');
            }
        }
        result.push(annotate_row.clone());
    // println!("minefield: {:?}", minefield);
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
}
    result
}