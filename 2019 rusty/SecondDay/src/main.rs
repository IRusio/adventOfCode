use std::fs;

fn main() {
    let filename = "./data.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split_content = contents.lines().next().unwrap().split(',');

    let mut data : Vec<u64> = split_content.map(|s| s.parse().unwrap()).collect();
    data[1] = 12;
    data[2] = 2;
    for i in (0 .. data.len()).step_by(4) {
        match data[i] {
            1 => {
                let field : usize = data[i+3] as usize;
                data[field as usize] = data[data[i+1] as usize] + data[data[i+2] as usize];
            }
            2 => {
                let field : usize = data[i+3] as usize;
                data[field] = data[data[i+1] as usize] * data[data[i+2] as usize];
            }
            99 => {
                break;
            }
            _ =>{
                println!("some error on position {}", i);
            }
        }
    }
    println!("{}", data[0]);
}