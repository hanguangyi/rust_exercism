use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    
    if _first_list.eq(_second_list){
        Comparison::Equal
    }else if _first_list.len()< _second_list.len() && contains(_second_list, _first_list){
        Comparison::Sublist
    }else if _first_list.len()> _second_list.len() && contains(_first_list, _second_list){
        Comparison::Superlist
    }else{
        Comparison::Unequal
    }
}

fn contains<T>(_super_list: &[T],_sub_list: &[T]) -> bool
    where T: PartialEq + Debug{
    let mut i: usize = 0;
    if _sub_list.len()==0 && _super_list.len()>0{
        return true;
    }else if _super_list.len() == 0{
        return false;
    }
    while i < _super_list.len(){
        if i>_super_list.len()-_sub_list.len(){
            return false;
        }else if _super_list[i] == _sub_list[0]{
            let mut sub_index = 1;
            let mut super_index = i;
            while sub_index < _sub_list.len(){
                super_index += 1;
                if _super_list[super_index] != _sub_list[sub_index]{
                    break;
                }
                sub_index += 1;
            }
            if sub_index == _sub_list.len() {
                return true;
            }        
        }
        i += 1;
    }
    return false;
        
}
