/**
 * Program to solve the Kattis problem "Rättstavning". See https://kth.kattis.com/problems/kth.adk.spelling
 *
 * Author: Jonathan Blomlöf, <jblomlof@kth.se>
 *
 * Using input templete provided in task.
 * See https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/dynprog-task-17
 */
use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;
use std::mem::swap;
const INIT_CAP: usize = 100;

#[allow(unused_must_use)]
fn main() {
    // get standard input stream
    let mut input = io::stdin().lock();

    let mut target_words: Vec<(String, usize)> = Vec::with_capacity(500_000);

    loop {
        let mut line = String::with_capacity(80);
        input.read_line(&mut line);
        line = line.trim().to_string();
        if line == "#" {
            break;
        }
        let mut sum = 0;
        for _ in line.chars() {
            sum += 1;
        }
        target_words.push((line, sum));
    }

    let mut _in = String::with_capacity(80);
    let mut words: Vec<&str> = Vec::with_capacity(INIT_CAP);
    let mut shortest_dist;

    while input.read_line(&mut _in).unwrap() > 0 {
        let misspelled = _in.trim();
        let mut _len_of_misspelled = 0;
        for _ in misspelled.chars() {
            _len_of_misspelled += 1;
        }
        shortest_dist = usize::MAX;
        words.clear();

        for _target in target_words.iter() {
            if _target.1.abs_diff(_len_of_misspelled) <= shortest_dist {
                //ok the length makes it so it might fit.
                let dist = word_dist(
                    misspelled,
                    &_target.0,
                    shortest_dist,
                    _len_of_misspelled,
                    _target.1,
                );

                match shortest_dist.cmp(&dist) {
                    Ordering::Equal => {
                        words.push(&_target.0);
                    }
                    Ordering::Greater => {
                        words.clear();
                        shortest_dist = dist;
                        words.push(&_target.0);
                    }
                    _ => (),
                }
            }
        }

        print!("{} ({}) ", misspelled, shortest_dist);
        for _w in words.iter() {
            print!("{} ", _w);
        }
        //dont think kattis needs this.
        //println!();

        _in.clear();
    }
    //flush
    println!()
}

/*
Function heavily build upon code and description in Wikipedia's article "Levenhtein distance"
See: https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_two_matrix_rows (Visited 2023-02-19)
 */
fn word_dist(
    word_one: &str,
    word_two: &str,
    max_limit: usize,
    size_of_one: usize,
    size_of_two: usize,
) -> usize {
    let mut upper_row = Vec::with_capacity(size_of_two + 1);
    let mut lower_row = vec![0; size_of_two + 1];

    for i in 0..(size_of_two + 1) {
        upper_row.push(i);
    }

    // an offset to calculate lowest possible dist in a cycle.
    let offset = size_of_two as isize - size_of_one as isize;

    for (index, char) in word_one.chars().enumerate() {
        let _to_look = offset + index as isize;
        if _to_look >= 0 && upper_row[_to_look as usize] > max_limit {
            return usize::MAX;
        }
        lower_row[0] = index + 1;

        for (_inner_i, inner_char) in word_two.chars().enumerate() {
            let cost_of_deletion = upper_row[_inner_i + 1] + 1;
            let cost_of_insertion = lower_row[_inner_i] + 1;
            let cost_of_subbing = upper_row[_inner_i] + {
                if char == inner_char {
                    0
                } else {
                    1
                }
            };
            let min = cost_of_deletion.min(cost_of_insertion).min(cost_of_subbing);
            lower_row[_inner_i + 1] = min;
        }

        //no way a swap actually exists. Nice!
        swap(&mut upper_row, &mut lower_row);
    }
    //returns distance
    *upper_row.last().unwrap()
}

fn convert_to_one_byte_per_char(word: &str) -> (String, bool) {
    let mut _ret = String::with_capacity(40);
    for char in word.chars() {
        match char {
            'å' => _ret.push('{'), // 123 as char
            'ä' => _ret.push('|'), // 124 as char
            'ö' => _ret.push('}'), // 125 as char
            _ => _ret.push(char),
        }
    }
    let _b = _ret.len() == word.len();
    (_ret, _b)
}

fn convert_back(word: &str) -> String {
    let mut _ret = String::with_capacity(40);
    for char in word.chars() {
        match char {
            '{' => _ret.push('å'), // 123 as char
            '|' => _ret.push('ä'), // 124 as char
            '}' => _ret.push('ö'), // 125 as char
            _ => _ret.push(char),
        }
    }
    _ret
}
