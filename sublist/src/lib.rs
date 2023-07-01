#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // _first_list
    //     .iter()
    //     .zip(_second_list.iter())
    //     .fold(Comparison::Equal, |acc, (x, y)| {
    //         if x == y {
    //             acc
    //         } else {
    //             Comparison::Unequal
    //         }
    //     })
    // ;
    if _first_list == _second_list {
        return Comparison::Equal;
    }else if is_sub(_first_list, _second_list){
        return Comparison::Sublist;
    }
    else if is_sub(_second_list, _first_list){
        return Comparison::Superlist;
    }
    else{
        return Comparison::Unequal;
    }
}

pub fn is_sub<T:PartialEq> (list : &[T], second_list : &[T]) -> bool  {
    let a = list.len();
    let b = second_list.len();
    if a > b{
        return false;
    }
    for i in 0..=b-a{
        if list == &second_list[i..i+a]{
            return true;
        }
    }
    return false;
}