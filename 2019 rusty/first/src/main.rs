use std::fs;

fn CalculateFuel(mut mass: i32) -> i32 {
    let result = (mass/3) as i32 -2;
    return if result > 0 {result} else {0};
}

fn Calculate(mut mass: i32) -> i32 {
    let mut fuelSum : i32 = 0;
    while mass > 0 {
        let fuel = CalculateFuel(mass);
        mass = fuel;
        fuelSum += mass;
    }
    return fuelSum;
}
fn main() {
    let filename = "./data.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut neededFuel = 0;

    for line in contents.lines() {
        let mass : i32 = line.parse().unwrap();
        neededFuel += CalculateFuel(mass);
    }

    //TaskOne
    println!("{}", neededFuel);

    let mut needed_fuel_task_two = 0;

    for line in contents.lines() {
        let mass : i32 = line.parse().unwrap();
        needed_fuel_task_two += Calculate(mass);
    }

    println!("{}", needed_fuel_task_two);
    println!("{}", Calculate(100756));



}
