pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    // convert output into char vector
    // let mut len: usize = input.chars().count();
    // let str_reverse = String::new();
    // //think abount of nil
    // if len == 0 {
    //     return str_reverse;
    // }
    // let mut byte_reverse: Vec<char> = vec!['\0';len];
    // for i in input.chars() {
    //     len = len-1;
    //     byte_reverse[len] = i;
        
    // }
    // let str_reverse: String = byte_reverse.into_iter().collect();
    // str_reverse
    input.chars().rev().collect()
}
