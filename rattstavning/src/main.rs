use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
/**
 * Program to solve the Kattis problem "Rättstavning". See https://kth.kattis.com/problems/kth.adk.spelling
 *
 * Author: Jonathan Blomlöf, <jblomlof@kth.se>
 *
 * Using input templete provided in task.
 * See https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/dynprog-task-17
 */
/*
PLAN CHANGED. FUCK THIS METHOD
LEVENSTEIN HERE I COME.

IDEA - FOR Each comparison.
 LET S be the shorter string and L be the longer string.
 WE KNOW DE FACTO that WORST case will be EDITING_DISTANCE = len(L).

BUT WE TAKE THE LETTERS IN S.
AND OUR GOAL IS NOW TO FIT THEM IN ORDER IN L
FOR EXAMPLE
S = "hej"
L = "hejsan"
worst = len(L) = 6
but we see that h and e and j fits in order
thus its 3

But there can be other chars inbetween.

S = "tja"
L = "tjena"
Worst = 5
but tja exists in L in that order
thus its 2.
point being len() is constant
best being len(s)

well worst is a problem

In theory we do need to look at this for all possible sub combinations of S.
eg if s_normal = "hej"
s1 = "h"
s2 = "e"
s3 = "j"

s4 = "he"
s5 = "hj"
s6 = "ej"

"s7 = "hej"

eg. 2.pow(len(s)) - 1 // no need to check empty ("")
with order.

for instance if j's not found
why search "hej" j wont exist. ("he") might

so we search for induvidual letters first. // takes len(s) * len(l)
we prob want to keep track of whichs indexes each char exists in.

Now we can search through this list that has useful info about index

which takes len(s) at worst.
so its not terrible.

although levenstein-distance is prob just easier and faster.
 */
use std::io::{self, BufReader};

use std::mem::swap;
use std::time::Instant;
const INIT_CAP: usize = 100;

fn main() {
    //println!("dist {}", word_dist("hejzan", "helan", 2, 6).0);
    //return;
    // get standard input stream
    //let mut input = io::stdin().lock();
    let input = File::open("test.txt").unwrap();

    let mut buf = BufReader::new(input);

    //max 500_000 words for kattis-problem.
    //to be fast and dont care about RAM
    let start = Instant::now();
    let mut target_words: Vec<String> = Vec::with_capacity(500_000);
    loop {
        let mut line = String::new();
        buf.read_line(&mut line).unwrap();
        let _line = line.trim();
        if _line == "#" {
            break;
        }
        target_words.push(convert_to_one_byte_per_char(_line));
    }

    println!("RETRIEVING TOOK {} MS.", start.elapsed().as_millis());

    let mut _in = String::new();
    while buf.read_line(&mut _in).unwrap() > 0 {
        let now = Instant::now();
        let misspelled = convert_to_one_byte_per_char(_in.trim());

        let mut shortest_dist = usize::MAX;
        let mut words: Vec<&str> = Vec::with_capacity(INIT_CAP);

        for _target in target_words.iter_mut() {
            if _target.len().abs_diff(misspelled.len()) <= shortest_dist {
                //ok the length makes it so it might fit.
                let dist = word_dist(&misspelled, &_target, shortest_dist);
                match shortest_dist.cmp(&dist) {
                    Ordering::Equal => {
                        words.push(_target);
                    }
                    Ordering::Greater => {
                        words.clear();
                        shortest_dist = dist;
                        words.push(_target)
                    }
                    _ => (),
                }
            }
        }
        print!("{} ({}) ", convert_back(&misspelled), shortest_dist);
        for _w in words {
            print!("{} ", convert_back(_w));
        }
        println!("- {} ms.", now.elapsed().as_millis());

        _in.clear();
    }
    println!("PROGRAM TOOK {} MS", start.elapsed().as_millis());
}

/*
Function heavily build upon code and description in Wikipedia's article "Levenhtein distance"
See: https://en.wikipedia.org/wiki/Levenshtein_distance#Iterative_with_two_matrix_rows (Visited 2023-02-19)
 */
fn word_dist(word_one: &str, word_two: &str, max_limit: usize) -> usize {
    let mut upper_row = Vec::with_capacity(word_two.len() + 1);
    let mut lower_row = Vec::with_capacity(word_two.len() + 1);

    // now i know some shit right here.
    // word_two.len() can be bigger than the amount of chars
    // since åäö are two bytes long and the len() fn returns amount of bytes.
    // xdd
    /*assert!("å".len() == 2);
    assert!("ä".len() == 2);
    assert!("ö".len() == 2);*/

    //this is now solved thanks to god-ly convert_to_one_byte_per_char


    upper_row.push(0);
    lower_row.push(0);
    for i in 0..word_two.len() {
        upper_row.push(i + 1);
        lower_row.push(0);
    }

    // an offset to calculate lowest possible dist in a cycle.
    let offset = word_two.len() as isize - word_one.len() as isize;

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
            /*if _inner_i as isize - index as isize == offset && min > max_limit {
                //we are on the diagonal. This is our lowest possible value.
                // so if our min > max_limit, this word will not be better then our best.
                return usize::MAX;
            }*/
            lower_row[_inner_i + 1] = min;
        }

        //no way a swap actually exists. Nice!
        swap(&mut upper_row, &mut lower_row);
    }
    //returns distance and len of word.
    *upper_row.last().unwrap()
}

fn convert_to_one_byte_per_char(word: &str) -> String {
    let mut _ret = String::new();
    for char in word.chars() {
        match char {
            'å' => _ret.push('{'), // 123 as char
            'ä' => _ret.push('|'), // 124 as char
            'ö' => _ret.push('}'), // 125 as char
            _ => _ret.push(char),
        }
    }
    _ret
}

fn convert_back(word: &str) -> String {
    let mut _ret = String::new();
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
/*
THIS IS CLEARLY NOT WORKING. IT WAS FUN TRYING BUT IM JUST GONNA DO LEVENSTEIN METHOD INSTEAD
BECAUSE ITS FAST AND EAZY TO IMPLEMENT.


fn word_dist(s_word: &str, l_word: &str) -> usize {
    //im crying its so stupid.
    let l_len = {
        let mut sum = 0;
        for _ in l_word.chars() {
            sum += 1;
        }
        sum
    };
    let s_len = {
        let mut sum = 0;
        for _ in s_word.chars() {
            sum += 1;
        }
        sum
    };

    if s_len > l_len {
        //flip em cuz favorable if s_word <= l_word
        return word_dist(l_word, s_word);
    }

    //worst result is changing/adding every char.
    //bruh it counts ö as size 2.
    //let res = l_len;
    let mut chars_and_indecies: Vec<Vec<usize>> = vec![Vec::with_capacity(5); s_len];

    for (index, char) in s_word.chars().enumerate() {
        for (pos, cmp_char) in l_word.chars().enumerate() {
            if char == cmp_char {
                //println!("{} {}, {} {}", char, cmp_char, index, pos);
                chars_and_indecies[index].push(pos);
            }
        }
    }

    //println!("{:?}", chars_and_indecies);
    // stores where a insert would be possbile to get better result and what that result will be.
    // so each index in chars_and_indecies stores all positions in _l where the char s[index] corresponds.
    let mut alternatives: Vec<(usize, isize)> = Vec::with_capacity(3);
    let mut ok_pos = l_len;
    let mut prev_index = s_len - 1;
    let mut popped = 0;
    let mut carry_over = 0;
    let mut double_encountered = false;
    let mut removed_char = false;

    for (index, char_pos) in chars_and_indecies.iter().enumerate().rev() {
        if char_pos.len() > 0 {
            if char_pos[0] < ok_pos {
                // we can fit this char in the word meaning we can potentially reduce "char-operations".

                //there exists and index find the highest.
                let mut temp = 0;
                let mut index_for_temp = 0;
                for i in (0..char_pos.len()).rev() {
                    temp = char_pos[i];
                    index_for_temp = i;
                    if temp < ok_pos {
                        break;
                    }
                }

                //removed_char = false;
                // now if there are more letters in between our chars (in s_word) than in l_word we would need to pop em.
                if prev_index - index > ok_pos - temp {
                    popped += prev_index - index - 1;
                    if index_for_temp > 0 && char_pos[index_for_temp - 1] == temp - 1 {
                        popped -= 1;
                        temp -= 1;
                    }
                    removed_char = true;
                    //println!("- {}, {}", popped, prev_index - index - 1);
                } else {
                    removed_char = false;
                }
                double_encountered = false;
                //println!("{} {}, {} {}, {} {}", carry_over + 1, popped, temp, ok_pos, index, prev_index);

                alternatives.push((ok_pos, -1));

                carry_over += 1;

                prev_index = index;
                ok_pos = temp;
                //an alternative is if we can find 2 chars whom can only exist after our pos (ok_pos). so keep track of that
            } else if char_pos[char_pos.len() - 1] == ok_pos {
                //edge case. The first occurence is not before, but the last ocuurence is not later
                // thus we cant use it.

                //this means we got a double char but we can only use one of them.
                // this also means we should deal with which letter we want to use.
                // I need to deal with this
                /*
                TODO
                 */

                //i think this works.
                double_encountered = true;
                //println!("HOW AM I HERE");
                if removed_char {
                    //println!("BUT NOT HERE");
                    popped -= 1;
                    double_encountered = false;
                }

                continue;
            } else {
                //this char can only exist after
                for (_i, alt) in alternatives.iter_mut().rev().enumerate() {
                    //println!("{} bruh {}", char_pos[0], alt.0);
                    if char_pos[0] < alt.0 {
                        alt.1 += 1;

                        let mut temp = 0;
                        for i in (0..char_pos.len()).rev() {
                            temp = char_pos[i];
                            if temp < alt.0 {
                                break;
                            }
                        }
                        alt.0 = temp;
                        //println!("{}: {} -- {}", temp, alt.1, _i);
                        if alt.1 > _i as isize {
                            //println!("HERE - {} {:?}", temp, char_pos);
                            ok_pos = temp;
                            prev_index = index;
                            carry_over += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    //println!("{} , {} , {}", l_len, carry_over, popped);
    //res_dist - (carry_over - popped)
    l_len - (carry_over - popped)
}
*/
