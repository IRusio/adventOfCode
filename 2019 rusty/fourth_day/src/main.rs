use std::collections::HashMap;

fn is_pass_criteria(int_number: i32) -> HashMap<i32, i32> {
    let mut prev_number: i32 = 0;
    let mut repeats: HashMap<i32, i32> = HashMap::new();
    let numbers : Vec<i32> = int_number.to_string().chars().map(|n| n.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();

    for number in numbers{
        if number < prev_number {
            return HashMap::new();
        }
        if number == prev_number{
           if repeats.contains_key(&number)
           {
                if let Some(count) = repeats.get_mut(&number){
                    *count += 1;
                }
            }
           else {
               repeats.insert(number, 1);
           } 
        }

        prev_number = number;   
    }

    return repeats;
}

fn is_pass_criteria_first(int_number: i32) -> bool {
    let repeats = is_pass_criteria(int_number);
    return repeats.values().any(|n| *n >= 1i32);
}

fn is_pass_criteria_second(int_number: i32) -> bool {
    let repeats = is_pass_criteria(int_number);
    return repeats.values().any(|n| *n == 1i32);
}

fn main() {
    let mut counter_passing_first : i32 = 0;
    let mut counter_passing_second : i32 = 0;
    for itt in 248345 .. 746316 {
        if is_pass_criteria_first(itt){
            counter_passing_first+=1;
        }

        if is_pass_criteria_second(itt){
            counter_passing_second+=1;
        }
    }

    println!("{}", counter_passing_first);
    print!("{}", counter_passing_second);
    //println!("{}",is_pass_criteria(111111));
}
