use std::io;

fn mdc(mut number_a: u32, mut number_b: u32) -> u32 {
    let mut aux;

    while number_b != 0 {
        aux = number_a % number_b;
        number_a = number_b;
        number_b = aux;
    }

    return number_a;
}
fn main() {
    let mut test_cases = String::new();

    io::stdin().read_line(&mut test_cases).expect("Failed to read line");

    let mut test_cases: i32 = test_cases.trim().parse().unwrap();

    while test_cases > 0 {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let mut inputs: Vec<u32> = Vec::new();

        for it in line.split(" ") {
            inputs.push(it.trim().parse::<u32>().unwrap());
        }

        println!("{}", mdc(inputs[0], inputs[1]));
        test_cases -= 1;
    }
}
