use std::fs;

fn main() {
    day1();
    day2();
}

fn run_instruction(int_codes: &mut Vec<i32>, instruction: usize) -> (i32, i32, i32, i32){
    let op_code = int_codes[instruction];

    if op_code == 99 {
        return (op_code, 0, 0, 0)
    }

    let first_loc: usize = int_codes[instruction + 1] as usize;
    let first_value: i32 = int_codes[first_loc];

    let second_loc: usize = int_codes[instruction + 2] as usize;
    let second_value: i32 = int_codes[second_loc];

    let destination: usize = int_codes[instruction + 3] as usize;

    if op_code == 1 { // addition
        int_codes[destination] = first_value + second_value;
    }

    if op_code == 2 { // multiplication
        int_codes[destination] = first_value * second_value;
    }

    return (op_code, first_value, second_value, int_codes[destination])
}

fn get_int_codes() -> Vec<i32>{
    let codes = fs::read_to_string("input.txt").unwrap();
    
    let int_codes_strs = codes.split(",").collect::<Vec<&str>>();
    let mut int_codes: Vec<i32> = Vec::new();

    for int_code_str in int_codes_strs{
        int_codes.push(int_code_str.parse().unwrap());
    }

    return int_codes;
}

fn day1(){
    let mut int_codes = get_int_codes();

    let mut instruction: usize = 0;
    let mut complete: bool = false;

    int_codes[1] = 12;
    int_codes[2] = 2;

    while !complete {
        let (op_code, _first, _second, _sum) = run_instruction(&mut int_codes, instruction);
        if op_code == 99{
            complete = true;
        }
        instruction += 4;
    }

    println!("Part 1: {}", int_codes[0]);
}

fn day2(){
    let mut int_codes = get_int_codes();

    let mut instruction: usize = 0;
    let mut complete: bool = false;

    let mut noun: i32 = 0;
    let mut verb: i32 = 0;

    int_codes[1] = noun;
    int_codes[2] = verb;

    while !complete {
        
        let (op_code, _first, _second, _sum) = run_instruction(&mut int_codes, instruction);

        //println!("opcode: {}, first: {}, second: {}", op_code, first, second);
        instruction += 4;

        if op_code == 99 {
            if int_codes[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                complete = true;
            }
            else {
                println!("noun: {}, verb: {}, output: {}", noun, verb, int_codes[0]);
                int_codes = get_int_codes();
                instruction = 0;
                if verb == 99{
                    noun += 1;
                    verb = 0;
                }
                else{
                    verb += 1;
                }

                int_codes[1] = noun;
                int_codes[2] = verb;
            }
        }
    }
}