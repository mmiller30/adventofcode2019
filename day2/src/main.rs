//references: https://github.com/hulufei/aoc-2019/blob/master/src/day2/mod.rs

use std::fs;

fn main() {

    /* Part 1
    let mut program = parse_input();
    println!("input: {:?}", program);

    run_with(&mut program, 12, 2);
    println!("input: {:?}", program);
    */

    // Part 2
    //fn main() -> usize {
        let program = parse_input();
        let mut noun = 0;
        let mut verb = 0;
    
        for i in 0..=99 {
            for j in 0..=99 {
                if run_with(&mut program.clone(), i, j) == 19_690_720 {
                    noun = i;
                    verb = j;
                    break;
                }
            }
        }
    
        let answer = 100 * noun + verb;
        println!("answer: {}", answer);

}

fn run(program: &mut [usize]) {
    for i in (0..program.len()).step_by(4) {
        let op = program[i];

        match op {
            // opcode1
            1 => program[program[i+3]] = program[program[i+1]] + program[program[i+2]],
            // opcode2
            2 => program[program[i+3]] = program[program[i+1]] * program[program[i+2]],
            // opcode99
            99 => break,
            //all other opcodes
            _ => panic!("Unknow opcode {}", op),
        }
    }
}

fn run_with(program: &mut [usize], noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;
    run(program);
    program[0]
    //println!("OpCode[0]: {}", program[0]);
}

fn parse_input() -> Vec<usize> {
    fs::read_to_string("input.csv")
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}