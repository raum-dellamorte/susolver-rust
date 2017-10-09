SuSolver
========

A Sudoku Solver written in Rust.

Solves logically using documented strategies .

Current State
-------------

It works!  ~~It only does simple elimination at the moment, but it's a working sudoku solver written in Rust! It should be able to solve any Sudoku marked as 'Easy'.~~

~~It now does simple elimination and Hidden Singles.  This was enough to solve the "Hard" puzzle I through at it.  Remaining strategies will be an ever growing pain.~~

*BUG FIX* The code I used to determine the block number and the row and col inside a block was wrong which prevented the Pointing Pairs function from working.  The moment I eliminated that bug, the Pointing Pairs function did it's job and the entire puzzle was solved mostly with simple elimination from there on out.

Working Strategies:
- Hidden Singles
- Naked Pairs and Triplets
- Hidden Pairs and Triplets (Triplets not tested but should work)
- *NEW* Pointing Pairs
- *NEW* Box Line Reduction

**Road Map:** Check out [The Sudoku Wiki](http://www.sudokuwiki.org/sudoku.htm) for all the horrific strategies I have to teach a machine to do.  They already did it, but so what, it's still an excellent excercise in logic.  I write all my sudoku solving functions based on the descriptions of strategies listed there.  Also, I need to be able to select available puzzle files instead of hard coding which puzzle to load, a way of entering a puzzle from within the program and saving it, and a GUI to make both of those things easier for the user.  Also, I *could* make it a library for use in Python and Ruby or whatever.

Why?
----

Proof of Concept, that's why!

I initially wrote it in Ruby, which was so slow!  That led me on a quest for a faster language.  So my first rewrite of the SuSolver I wrote to learn [Mirah](https://github.com/mirah/mirah), a JVM language with Ruby syntax, static typing, type inference, and no runtime library (meaning it compiles to pure java bytecode), and it was faster than the Ruby version, but still not fast enough.  Not to mention that Mirah is hard to debug at this point in the game. This version is the most complete, including complex strategies like [Sword Fish](http://www.sudokuwiki.org/Sword_Fish_Strategy) and [X Cycles](http://www.sudokuwiki.org/X_Cycles), but I haven't touched it in a while.

Then I wrote it again in Haskell (for Science!) to learn the language and functional strategies. It was a good thing I did because Rust has a lot in common with Haskell. I think the Haskell version gets as far as [Naked Pairs](http://www.sudokuwiki.org/Naked_Candidates#NP).  The purely functional aspect was what slowed development the most for me, as every bit of functionality I wanted to add required more thought to express in lamda calculus.  Information doesn't stop and wait for you inside a variable, it must constantly be passed up and down through functions, only stopping when the last function finishes.

Now I'm doing it again in Rust (also for Science!).

Haskell had a learning curve from Hell, and if you've ever looked at Haskell code without knowing Haskell you probably guessed that about it.  Rust has a much more friendly appearance and I think it inspires false confidence, only to have the compiler laugh derisively when you try to do something that seems like it should be simple.  It mockingly says "I had a vicious learning curve all along!  My concept of Ownership and Borrowing makes sense to the ears, yes?  But just you try and apply the concept!"  (Update: I'm mostly over that curve now) (Update 2: I used VS Code with the Rust Language Server (RLS) to write the Pointing Pairs function and it was a considerable code-writing speed boost.  RLS lints the code and tells you what won't compile as you type which kept me from forgetting all the ampersands and asterisks the compiler demands.  I can't wait for RLS to come to Atom.)

It is a picky picky compiler, but there's a warm feeling in the confidence that once you get your code to compile you've got a memory safe executable with 99% of common bugs already squashed. *Update* And now with RLS, I only have to compile when I want to make sure my actual logic is good for what I'm doing instead of the compile-attempt/debug cycle I had to deal with before, not to mention the code completion.

Example Time!
-------------

The last thing I added before my initial commit was a function to render the puzzle to the terminal with pencil marks.

Console output for a puzzle it can't YET solve with simple elimination:
```
Puzzle :
_ _ 3   _ _ _   _ _ 3     * | *   _ _ 3   1 _ 3     _ _ 3   1 _ 3   * | *
_ _ 6   4 _ 6   _ _ _     - 5 -   _ _ _   _ _ _     4 _ 6   4 _ 6   - 2 -
_ 8 9   _ 8 9   _ _ 9     * | *   _ 8 _   _ _ _     7 8 9   7 8 _   * | *

* | *   * | *   _ 2 3     * | *   _ _ 3   * | *     * | *   _ _ 3   _ _ 3
- 1 -   - 7 -   _ _ _     - 4 -   _ _ _   - 6 -     - 5 -   _ _ _   _ _ _
* | *   * | *   _ _ 9     * | *   _ 8 _   * | *     * | *   _ 8 _   _ 8 9

* | *   _ 2 _   _ 2 3     * | *   * | *   1 2 3     _ _ 3   1 _ 3   _ _ 3
- 5 -   4 _ 6   _ _ _     - 7 -   - 9 -   _ _ _     4 _ 6   4 _ 6   4 _ _
* | *   _ 8 _   _ _ _     * | *   * | *   _ _ _     _ 8 _   _ 8 _   _ 8 _


_ _ _   * | *   1 _ _     1 _ _   _ _ _   1 _ _     * | *   _ _ _   _ _ _
_ _ _   - 3 -   _ 5 _     _ _ _   4 5 6   _ 5 _     - 2 -   4 5 6   4 5 _
7 8 9   * | *   7 _ 9     _ 8 9   7 8 _   7 _ _     * | *   7 8 _   7 8 _

_ _ _   _ _ _   * | *     _ _ 3   * | *   _ _ 3     * | *   _ _ 3   _ _ 3
_ _ _   _ 5 _   - 4 -     _ _ _   - 2 -   _ 5 _     - 1 -   _ 5 6   _ 5 _
7 8 9   _ 8 9   * | *     _ 8 9   * | *   7 _ _     * | *   7 8 _   7 8 _

_ 2 _   1 2 _   * | *     1 _ 3   _ _ 3   1 _ 3     _ _ 3   * | *   _ _ 3
_ _ _   _ 5 _   - 6 -     _ _ _   4 5 _   _ 5 _     4 _ _   - 9 -   4 5 _
7 8 _   _ 8 _   * | *     _ 8 _   7 8 _   7 _ _     7 8 _   * | *   7 8 _


_ 2 3   _ 2 _   _ 2 3     _ 2 3   * | *   * | *     _ _ 3   _ _ 3   * | *
_ _ _   _ 5 _   _ 5 _     _ _ _   - 1 -   - 8 -     4 _ _   4 5 _   - 6 -
7 _ 9   _ _ 9   7 _ 9     _ _ _   * | *   * | *     7 _ 9   7 _ _   * | *

_ _ 3   _ _ _   * | *     * | *   _ _ 3   * | *     _ _ 3   * | *   * | *
_ _ _   _ 5 _   - 8 -     - 6 -   _ 5 _   - 4 -     _ _ _   - 2 -   - 1 -
7 _ 9   _ _ 9   * | *     * | *   7 _ _   * | *     7 _ 9   * | *   * | *

* | *   1 2 _   1 2 3     _ 2 3   _ _ 3   * | *     _ _ 3   _ _ 3   _ _ 3
- 4 -   _ 5 6   _ 5 _     _ _ _   _ 5 _   - 9 -     _ _ _   _ 5 _   _ 5 _
* | *   _ _ _   7 _ _     _ _ _   7 _ _   * | *     7 8 _   7 8 _   7 8 _
```

This was the input read from a txt file:
```
0 0 0  5 0 0  0 0 2 
1 7 0  4 0 6  5 0 0 
5 0 0  7 9 0  0 0 0 

0 3 0  0 0 0  2 0 0 
0 0 4  0 2 0  1 0 0 
0 0 6  0 0 0  0 9 0 

0 0 0  0 1 8  0 0 6 
0 0 8  6 0 4  0 2 1 
4 0 0  0 0 9  0 0 0
```

**Update**
Current "Evil" difficulty output.  I forgot this "Evil" puzzle only gets up to Pointing Pairs, which I've now implemented.
Thus, it now solves this puzzle and I'll have to switch to something more challenging to continue development.
```
savedSudoku-Evil01.txt contains:
3 0 0  0 9 0  0 0 8
0 0 0  7 0 0  0 9 0
0 0 4  0 8 6  0 0 0

0 0 6  0 0 0  0 2 0
0 9 0  1 3 5  0 7 0
0 5 0  0 0 0  9 0 0

0 0 0  9 2 0  1 0 0
0 6 0  0 0 3  0 0 0
5 0 0  0 1 0  0 0 9
Running simpleElim | A2 drop 3 | A2 drop 9 | A2 drop 8 | A2 drop 4 | A2 drop 5 | A2 drop 6
 | A3 drop 3 | A3 drop 9 | A3 drop 8 | A3 drop 4 | A3 drop 6 | A4 drop 3 | A4 drop 9
 | A4 drop 8 | A4 drop 7 | A4 drop 6 | A4 drop 1 | A6 drop 3 | A6 drop 9 | A6 drop 8
 | A6 drop 7 | A6 drop 6 | A6 drop 5 | A7 drop 3 | A7 drop 9 | A7 drop 8 | A7 drop 1
 | A8 drop 3 | A8 drop 9 | A8 drop 8 | A8 drop 2 | A8 drop 7 | B1 drop 3 | B1 drop 7
 | B1 drop 9 | B1 drop 4 | B1 drop 5 | B2 drop 3 | B2 drop 7 | B2 drop 9 | B2 drop 4
 | B2 drop 5 | B2 drop 6 | B3 drop 3 | B3 drop 7 | B3 drop 9 | B3 drop 4 | B3 drop 6 
 | B5 drop 9 | B5 drop 7 | B5 drop 8 | B5 drop 6 | B5 drop 3 | B5 drop 2 | B5 drop 1 
 | B6 drop 9 | B6 drop 7 | B6 drop 8 | B6 drop 6 | B6 drop 5 | B6 drop 3 | B7 drop 8 
 | B7 drop 7 | B7 drop 9 | B7 drop 1 | B9 drop 8 | B9 drop 7 | B9 drop 9 | C1 drop 3 
 | C1 drop 4 | C1 drop 8 | C1 drop 6 | C1 drop 5 | C2 drop 3 | C2 drop 4 | C2 drop 8 
 | C2 drop 6 | C2 drop 9 | C2 drop 5 | C4 drop 9 | C4 drop 7 | C4 drop 4 | C4 drop 8 
 | C4 drop 6 | C4 drop 1 | C7 drop 8 | C7 drop 9 | C7 drop 4 | C7 drop 6 | C7 drop 1 
 | C8 drop 8 | C8 drop 9 | C8 drop 4 | C8 drop 6 | C8 drop 2 | C8 drop 7 | C9 drop 8 
 | C9 drop 9 | C9 drop 4 | C9 drop 6 | D1 drop 3 | D1 drop 6 | D1 drop 2 | D1 drop 9 
 | D1 drop 5 | D2 drop 6 | D2 drop 2 | D2 drop 9 | D2 drop 5 | D4 drop 7 | D4 drop 6 
 | D4 drop 2 | D4 drop 1 | D4 drop 3 | D4 drop 5 | D4 drop 9 | D5 drop 9 | D5 drop 8 
 | D5 drop 6 | D5 drop 2 | D5 drop 1 | D5 drop 3 | D5 drop 5 | D6 drop 6 | D6 drop 2 
 | D6 drop 1 | D6 drop 3 | D6 drop 5 | D7 drop 6 | D7 drop 2 | D7 drop 7 | D7 drop 9 
 | D7 drop 1 | D9 drop 8 | D9 drop 6 | D9 drop 2 | D9 drop 7 | D9 drop 9 | E1 drop 3 
 | E1 drop 6 | E1 drop 9 | E1 drop 1 | E1 drop 5 | E1 drop 7 | E3 drop 4 | E3 drop 6 
 | E3 drop 9 | E3 drop 1 | E3 drop 3 | E3 drop 5 | E3 drop 7 | E7 drop 2 | E7 drop 9 
 | E7 drop 1 | E7 drop 3 | E7 drop 5 | E7 drop 7 | E9 drop 8 | E9 drop 2 | E9 drop 9 
 | E9 drop 1 | E9 drop 3 | E9 drop 5 | E9 drop 7 | F1 drop 3 | F1 drop 6 | F1 drop 9 
 | F1 drop 5 | F3 drop 4 | F3 drop 6 | F3 drop 9 | F3 drop 5 | F4 drop 7 | F4 drop 1 
 | F4 drop 3 | F4 drop 5 | F4 drop 9 | F5 drop 9 | F5 drop 8 | F5 drop 1 | F5 drop 3 
 | F5 drop 5 | F5 drop 2 | F6 drop 6 | F6 drop 1 | F6 drop 3 | F6 drop 5 | F6 drop 9 
 | F8 drop 9 | F8 drop 2 | F8 drop 7 | F8 drop 5 | F9 drop 8 | F9 drop 2 | F9 drop 7 
 | F9 drop 5 | F9 drop 9 | G1 drop 3 | G1 drop 9 | G1 drop 2 | G1 drop 1 | G1 drop 6 
 | G1 drop 5 | G2 drop 9 | G2 drop 5 | G2 drop 2 | G2 drop 1 | G2 drop 6 | G3 drop 4 
 | G3 drop 6 | G3 drop 9 | G3 drop 2 | G3 drop 1 | G3 drop 5 | G6 drop 6 | G6 drop 5 
 | G6 drop 9 | G6 drop 2 | G6 drop 1 | G6 drop 3 | G8 drop 9 | G8 drop 2 | G8 drop 7 
 | G8 drop 1 | G9 drop 8 | G9 drop 9 | G9 drop 2 | G9 drop 1 | H1 drop 3 | H1 drop 6 
 | H1 drop 5 | H3 drop 4 | H3 drop 6 | H3 drop 3 | H3 drop 5 | H4 drop 7 | H4 drop 1 
 | H4 drop 9 | H4 drop 2 | H4 drop 6 | H4 drop 3 | H5 drop 9 | H5 drop 8 | H5 drop 3 
 | H5 drop 2 | H5 drop 6 | H5 drop 1 | H7 drop 9 | H7 drop 1 | H7 drop 6 | H7 drop 3 
 | H8 drop 9 | H8 drop 2 | H8 drop 7 | H8 drop 1 | H8 drop 6 | H8 drop 3 | H9 drop 8 
 | H9 drop 1 | H9 drop 6 | H9 drop 3 | H9 drop 9 | I2 drop 9 | I2 drop 5 | I2 drop 6 
 | I2 drop 1 | I3 drop 4 | I3 drop 6 | I3 drop 5 | I3 drop 1 | I3 drop 9 | I4 drop 7 
 | I4 drop 1 | I4 drop 9 | I4 drop 2 | I4 drop 3 | I4 drop 5 | I6 drop 6 | I6 drop 5 
 | I6 drop 9 | I6 drop 2 | I6 drop 3 | I6 drop 1 | I7 drop 9 | I7 drop 1 | I7 drop 5 
 | I8 drop 9 | I8 drop 2 | I8 drop 7 | I8 drop 1 | I8 drop 5
Running hiddenSingle
hiddenSingle 6 found for B1
Cell B1 solved as 6
Running simpleElim | B7 drop 6 | B9 drop 6
Running hiddenSingle
hiddenSingle 9 found for C1
Cell C1 solved as 9
Running simpleElim | H1 drop 9
Running hiddenSingle
hiddenSingle 3 found for C4
Cell C4 solved as 3
Running simpleElim | C7 drop 3 | C8 drop 3 | C9 drop 3
Running hiddenSingle
hiddenSingle 9 found for D6
Cell D6 solved as 9
Running simpleElim | Running hiddenSingle
hiddenSingle 6 found for F5
Cell F5 solved as 6
Running simpleElim | F4 drop 6 | F8 drop 6 | F9 drop 6
Running hiddenSingle
hiddenSingle 9 found for H3
Cell H3 solved as 9
Running simpleElim | Running hiddenSingle
hiddenSingle 1 found for H1
Cell H1 solved as 1
Running simpleElim | D1 drop 1 | F1 drop 1
Running hiddenSingle
hiddenSingle 6 found for I4
Cell I4 solved as 6
Running simpleElim | I7 drop 6 | I8 drop 6
Running hiddenSingle | Running nakedPairsTrips
Naked Triplet<D1, D4, D5>: Eliminating [7, 8, 4] from D2
Naked Triplet<D1, D4, D5>: Eliminating [7, 8, 4] from D7
Naked Triplet<D1, D4, D5>: Eliminating [7, 8, 4] from D9
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips
Hidden Pair<D2, F3>[1, 3]: Eliminating other values.
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips
Hidden Pair<G8, G9>[5, 6]: Eliminating other values.
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips
Naked Pair<G8, G9>: Eliminating [6, 5] from H7
Naked Pair<G8, G9>: Eliminating [6, 5] from H8
Naked Pair<G8, G9>: Eliminating [6, 5] from H9
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs
Pointing Pair<Block 5, BRow 3>: Eliminating 2 from F1.
Running simpleElim | Running hiddenSingle
hiddenSingle 2 found for E1
Cell E1 solved as 2
Running simpleElim | E3 drop 2
Cell E3 solved as 8
 | B3 drop 8 | D1 drop 8 | E7 drop 8 | F1 drop 8 | G3 drop 8 | I3 drop 8
Running hiddenSingle
hiddenSingle 8 found for B2
Cell B2 solved as 8
Running simpleElim | G2 drop 8 | I2 drop 8
Running hiddenSingle
hiddenSingle 8 found for D4
Cell D4 solved as 8
Running simpleElim | F4 drop 8 | F6 drop 8 | H4 drop 8
Running hiddenSingle
hiddenSingle 8 found for F8
Cell F8 solved as 8
Running simpleElim | H8 drop 8
Cell H8 solved as 4
 | A8 drop 4 | H4 drop 4
Cell H4 solved as 5
 | A4 drop 5 | H5 drop 5 | H5 drop 4
Cell H5 solved as 7
 | D5 drop 7
Cell D5 solved as 4
 | B5 drop 4
Cell B5 solved as 5
 | B3 drop 5 | B7 drop 5 | B9 drop 5 | D1 drop 4
Cell D1 solved as 7
 | F1 drop 7
Cell F1 solved as 4
 | F4 drop 4
Cell F4 solved as 2
 | A4 drop 2
Cell A4 solved as 4
 | A6 drop 4 | A7 drop 4 | B6 drop 4 | F6 drop 4 | F6 drop 2
Cell F6 solved as 7
 | F9 drop 4 | G1 drop 7 | G1 drop 4
Cell G1 solved as 8
 | G6 drop 7 | G6 drop 8
Cell G6 solved as 4
 | G2 drop 4 | H7 drop 7 | H7 drop 4 | H9 drop 7 | H9 drop 4
Cell H9 solved as 2
 | B9 drop 2 | C9 drop 2 | H7 drop 2
Cell H7 solved as 8
 | I6 drop 7 | I6 drop 4
Cell I6 solved as 8
 | I7 drop 8 | I7 drop 4 | I7 drop 2 | I8 drop 8 | I8 drop 4
Cell I8 solved as 3
 | I2 drop 3 | I3 drop 3 | I7 drop 3
Cell I7 solved as 7
 | A7 drop 7 | C7 drop 7 | I2 drop 7 | I3 drop 7
Cell I3 solved as 2
 | A3 drop 2 | B3 drop 2
Cell B3 solved as 1
 | A2 drop 1 | A3 drop 1 | B6 drop 1
Cell B6 solved as 2
 | A6 drop 2
Cell A6 solved as 1
 | A8 drop 1 | B7 drop 2 | B9 drop 1 | C2 drop 1 | F3 drop 1
Cell F3 solved as 3
 | D2 drop 3
Cell D2 solved as 1
 | D9 drop 1 | F9 drop 3
Cell F9 solved as 1
 | C9 drop 1 | G3 drop 3
Cell G3 solved as 7
 | A3 drop 7
Cell A3 solved as 5
 | A7 drop 5 | A8 drop 5
Cell A8 solved as 6
 | A7 drop 6
Cell A7 solved as 2
 | A2 drop 2
Cell A2 solved as 7
 | C2 drop 7
Cell C2 solved as 2
 | C7 drop 2
Cell C7 solved as 5
 | C8 drop 5
Cell C8 solved as 1
 | C9 drop 5
Cell C9 solved as 7
 | D7 drop 5
Cell D7 solved as 3
 | B7 drop 3
Cell B7 solved as 4
 | B9 drop 4
Cell B9 solved as 3
 | D9 drop 3
Cell D9 solved as 5
 | E7 drop 4
Cell E7 solved as 6
 | E9 drop 6
Cell E9 solved as 4
 | G2 drop 7
Cell G2 solved as 3
 | G8 drop 6
Cell G8 solved as 5
 | G9 drop 5
Cell G9 solved as 6
 | I2 drop 2
Cell I2 solved as 4

Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs | Finished
Puzzle :
* - *   + - +   + - +     + - +   * - *   + - +     + - +   + - +   * - *
| 3 |   | 7 |   | 5 |     | 4 |   | 9 |   | 1 |     | 2 |   | 6 |   | 8 |
* - *   + - +   + - +     + - +   * - *   + - +     + - +   + - +   * - *

+ - +   + - +   + - +     * - *   + - +   + - +     + - +   * - *   + - +
| 6 |   | 8 |   | 1 |     | 7 |   | 5 |   | 2 |     | 4 |   | 9 |   | 3 |
+ - +   + - +   + - +     * - *   + - +   + - +     + - +   * - *   + - +

+ - +   + - +   * - *     + - +   * - *   * - *     + - +   + - +   + - +
| 9 |   | 2 |   | 4 |     | 3 |   | 8 |   | 6 |     | 5 |   | 1 |   | 7 |
+ - +   + - +   * - *     + - +   * - *   * - *     + - +   + - +   + - +


+ - +   + - +   * - *     + - +   + - +   + - +     + - +   * - *   + - +
| 7 |   | 1 |   | 6 |     | 8 |   | 4 |   | 9 |     | 3 |   | 2 |   | 5 |
+ - +   + - +   * - *     + - +   + - +   + - +     + - +   * - *   + - +

+ - +   * - *   + - +     * - *   * - *   * - *     + - +   * - *   + - +
| 2 |   | 9 |   | 8 |     | 1 |   | 3 |   | 5 |     | 6 |   | 7 |   | 4 |
+ - +   * - *   + - +     * - *   * - *   * - *     + - +   * - *   + - +

+ - +   * - *   + - +     + - +   + - +   + - +     * - *   + - +   + - +
| 4 |   | 5 |   | 3 |     | 2 |   | 6 |   | 7 |     | 9 |   | 8 |   | 1 |
+ - +   * - *   + - +     + - +   + - +   + - +     * - *   + - +   + - +


+ - +   + - +   + - +     * - *   * - *   + - +     * - *   + - +   + - +
| 8 |   | 3 |   | 7 |     | 9 |   | 2 |   | 4 |     | 1 |   | 5 |   | 6 |
+ - +   + - +   + - +     * - *   * - *   + - +     * - *   + - +   + - +

+ - +   * - *   + - +     + - +   + - +   * - *     + - +   + - +   + - +
| 1 |   | 6 |   | 9 |     | 5 |   | 7 |   | 3 |     | 8 |   | 4 |   | 2 |
+ - +   * - *   + - +     + - +   + - +   * - *     + - +   + - +   + - +

* - *   + - +   + - +     + - +   * - *   + - +     + - +   + - +   * - *
| 5 |   | 4 |   | 2 |     | 6 |   | 1 |   | 8 |     | 7 |   | 3 |   | 9 |
* - *   + - +   + - +     + - +   * - *   + - +     + - +   + - +   * - *
```

