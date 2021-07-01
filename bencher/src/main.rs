extern crate termion;
mod tests;
use std::io::{stdout, stdin, Write};
use termion::{color, cursor, clear};

fn main() {
    let time_unit = from_terminal("[1: ns, 2: ms, 3: s, 4: m]\nSelect time unit: ", 4, 0) as u8;
    let factor = u128::pow(1000, time_unit.into()) / 1000;
    let tests_count = from_terminal("Tests count: ", u128::MAX, 0);

    println!("{}", clear::All);
    let mut bench_percent: u8;
    for test_index in 0..tests_count {
        bench_percent = (test_index as f64 / tests_count as f64 * 100.0) as u8;
        println!("{goto}{clear}Test count: {percent}% ({index} | {tests_count})\n",
        goto = cursor::Goto(1, 0),
        clear = clear::CurrentLine,
        percent = bench_percent,
        index = test_index,
        tests_count = tests_count);

        unsafe{
            let mut total_time: u128 = 0;
            let mut total_average: u128 = 0;
            for test in &mut tests::TESTS {
                test.exec();
                total_time += test.total_ns;
                total_average += test.avg;
            }

            for test in 0..tests::TESTS.len() {
                println!("{goto}{clear}{blue}Function: '{test_name}{blue}'\
                \n  {clear}{green}Total time percent: {time_percent}%{green}\
                \n  {clear}{red}Total average percent: {average_percent}%\
                \n  {clear}{yellow}Average: {time_average}",
                goto = cursor::Goto(0, (test * 4) as u16 + 4),
                clear = clear::CurrentLine,
                blue = color::Fg(color::Blue),
                test_name = &tests::TESTS[test].name,
                green = color::Fg(color::Green),
                time_percent = (*&tests::TESTS[test].total_ns as f64 / total_time as f64 * 100.0) as u8, 
                red = color::Fg(color::Red),
                average_percent = (*&tests::TESTS[test].avg as f64 / total_average as f64 * 100.0) as u8,
                yellow = color::Fg(color::Yellow),
                time_average = &tests::TESTS[test].avg / factor);
            }
        }
    }
}

fn from_terminal(text: &str, max: u128, default: u128) -> u128 {
    let mut result = default;
    while result == default {
        let mut input = String::from("");
        print!("{}", text);
        stdout()
            .flush()
            .unwrap_or(());

        stdin()
            .read_line(&mut input)
            .unwrap_or(0);
        
        result = input
            .trim()
            .parse::<u128>()
            .unwrap_or(default);

        if result > max {
            result = default;
        }

    }
    result
}
