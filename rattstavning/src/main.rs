/**
 * Program to solve the Kattis problem "Rättstavning". See https://kth.kattis.com/problems/kth.adk.spelling
 *
 * Author: Jonathan Blomlöf, <jblomlof@kth.se>
 *
 * Using input templete provided in task.
 * See https://github.com/IndaPlus22/AssignmentInstructions-Rust/tree/master/dynprog-task-17
 */
/*
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
use std::io;
use std::io::prelude::*;

fn main() {
    /*// get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    let mut lines = input.lock().lines().map(|_line| _line.ok().unwrap());
    // and get one line at a time,
    let next_line = lines.next().unwrap();

    // or loop all input lines,
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        // ...
    }

    // or read single line
    let mut line = String::new();
    input.read_line(&mut line).expect("IO Error");

    /* add code here ... */

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");*/

    println!("{}", word_dist("mskt", "maskot"));
}

fn word_dist(s_word: &str, l_word: &str) -> usize {
    if s_word.len() > l_word.len() {
        //flip em cuz favorable if s_word <= l_word
        return word_dist(l_word, s_word);
    }

    let mut res_dist = l_word.len();
    let mut chars_and_indecies: Vec<Vec<usize>> = vec![Vec::with_capacity(5); s_word.len()];

    for (index, char) in s_word.char_indices() {
        for (pos, cmp_char) in l_word.char_indices() {
            if char == cmp_char {
                chars_and_indecies[index].push(pos);
            }
        }
    }

    //println!("{:?}", chars_and_indecies);
    // stores where a insert would be possbile to get better result and what that result will be.

    let mut alternatives: Vec<(usize, isize)> = Vec::with_capacity(3);
    let mut ok_pos = l_word.len();
    for char_pos in chars_and_indecies.iter().rev() {
        if char_pos.len() > 0 {
            if char_pos[0] < ok_pos {
                //there exists and index find the highest.
                let mut temp = char_pos[0];
                for i in 0..char_pos.len()  {
                    temp = char_pos[i];
                    if i + 1 == char_pos.len() || char_pos[i + 1] > ok_pos {
                        break;
                    }
                }
                alternatives.push((ok_pos, -1));
                ok_pos = temp;
                //an alternative is if we can find 2 chars whom can only exist after our pos (ok_pos). so keep track of that

                res_dist -= 1
            } else {
                //this char can only exist after
                for (_i, alt) in alternatives.iter_mut().rev().enumerate() {
                    if char_pos[0] < alt.0 {
                        alt.1 += 1;

                        let mut temp = char_pos[0];
                        for i in 0..char_pos.len() - 1 {
                            temp = char_pos[i];
                            if i + 1 == char_pos.len() || char_pos[i + 1] > ok_pos {
                                break;
                            }
                        }
                        alt.0 = temp;

                        if alt.1 > _i as isize {
                            res_dist -= 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    res_dist
}
