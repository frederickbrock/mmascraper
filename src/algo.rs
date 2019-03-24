
    use std::collections::HashMap;

    pub fn find_duplicate_words(str: String) -> HashMap<String,i64> {
        let comp = str.split_whitespace();
        let mut words: HashMap<String,i64> = HashMap::new();
        let mut dups: HashMap<String,i64>  = HashMap::new();

        for word in comp {
            let count = words.entry(String::from(word)).or_insert(0);
            if  *count >= 0 {
                *count += 1; 
                let duplicate_count = dups.entry(String::from(word)).or_insert(1);
                *duplicate_count += 1;
            }
        }

        dups
        
    }

    pub fn find_missing_numbers(slice: &[i64]) -> i64 {
        let mut front: i64 = 0;
        let mut back: i64 = (slice.len() as i64) - 1;
        let mut mid: i64 = 0;

        while (back - front) > 1 {
            mid = (front + back) / 2;
            println!("mid is {}, front is {}, back is: {}", mid,front,back);
            if (slice[front as usize] - front) != (slice[mid as usize] - mid){
                println!("assigning back");
                back = mid;
            }else if (slice[back as usize] - back) != (slice[mid as usize] - mid){
                println!("assigning front");
                front = mid;
            } 
        }

        return slice[(mid as usize)] + 1;
    }


#[cfg(tests)]
mod tests {
    use std::collections::HashMap;
    use learning_rust::*; 
    
    #[test]
    fn test_find_duplicate_words(){
        let test_string = "this old brown cow has brown eyes and a big brown butt that says boo";
        let dups: HashMap = find_duplicate_words(test_string);
        assert!((dubs != null),true);
    }

    #[test]
    fn test_findMissingNumbers(){
        let slice: [i64; 9] = [1,2,3,4,5,6,7,9,10];
        let result = find_missing_numbers(&slice);
        assert_eq!(result == 8, "the result should be 8");
    }
}
