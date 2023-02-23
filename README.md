# jblomlof-task-17
17 > 18

## Task solution
This will be my final attempt.  
It's fast enough for all but the last test on kattis see [proof](Proof/Proof.PNG).

I think the only way to make it faster is to make a trie of the targeted words and then that will make searching more efficient. But i CBA to do that this week.  

There are "two versions" One for kattis and one to run test files provided in task. `rattstavning` is for test. `for_kattis` is for kattis, duh. 

I did some last minute edits in `for_kattis` to probably make it faster, but the overall solution is the same.

`rattstavning` is full of multi-line comments, theese are either ideas i had that sucked, theese can be ignored, or meta-data about program and functions.

### To run incase you don't belive it's working
Be in the [test](rattstavning/test/) folder and run `cargo r --release`.  It will print all the results to the terminal aswell as some time diagnostics.  
The test data consists of all the test files provided in task-instructions-repo combined to one file (They seemed to have the same target_words so I only combined the misspelled words).