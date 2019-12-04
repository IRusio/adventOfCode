use std::fs;

fn get_calculation(mut data: Vec<usize>) -> usize {
    for i in (0..data.len()).step_by(4) {
        match data[i] {
            1 => {
                let field: usize = data[i + 3];
                data[field] = data[data[i + 1]] + data[data[i + 2]];
            }
            2 => {
                let field: usize = data[i + 3];
                data[field] = data[data[i + 1]] * data[data[i + 2]];
            }
            99 => {
                break;
            }
            _ => {
                println!("some error on position {}", i);
            }
        }
    }
    return data[0];
}

fn main() {
    let filename = "./data.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = contents.lines().next().unwrap().split(',');

    let mut data: Vec<usize> = split_content.map(|s| s.parse().unwrap()).collect();

    data[1] = 12;
    data[2] = 2;
    println!("{}", get_calculation(data.clone()));

    let correct_value: usize = 19690720;
    for i in 0..99 {
        for j in 0..99 {
            data[1] = i;
            data[2] = j;
            if get_calculation(data.clone()) == correct_value {
                println!("{} {} -> {}", i, j, 100 * i + j);
            }
        }
    }
}