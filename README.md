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
- Pointing Pairs
- Box Line Reduction
- X-Wings
- *NEW* Y-Wings

Next:
- Simple Colouring

Simple Colouring was tough the last time I implemented it.  The word Simple is misleading as far as algorithms are concerned.  You trace a chain of connected cells to prove that a value can't be in one of those cells.  Nevertheless, the rust version solves nearly instantaneously while there was a noticable lag in all my previous versions in other languages, which I believe the JVM versions (Mirah and Kotlin) were the fastest.

I love Rust :)

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

This puzzle is an example to test the Y-Wings algorithm.  It finds the Y-Wing, but gives up when it runs into an example of Simple Colouring as there's no method for that yet.
```
savedSudoku-X-Wing01.txt contains:
0 9 3  0 0 4  5 6 0
0 6 0  0 0 3  1 4 0
0 0 4  6 0 8  3 0 9

9 8 1  3 4 5  0 0 0
3 4 7  2 8 6  9 5 1
6 5 2  0 7 0  4 8 3

4 0 6  0 0 2  8 9 0
0 0 0  4 0 0  0 1 0
0 2 9  8 0 0  0 3 4
Running simpleElim | A1 drop 9 | A1 drop 3 | A1 drop 4 | A1 drop 5 | A1 drop 6 | A4 drop 9 | A4 drop 3 | A4 drop 4 | A4
drop 5 | A4 drop 6 | A4 drop 8 | A4 drop 2 | A5 drop 9 | A5 drop 3 | A5 drop 4 | A5 drop 5 | A5 drop 6 | A5 drop 8 | A5
drop 7 | A9 drop 9 | A9 drop 3 | A9 drop 4 | A9 drop 5 | A9 drop 6 | A9 drop 1 | B1 drop 9 | B1 drop 3 | B1 drop 6 | B1
drop 1 | B1 drop 4 | B3 drop 9 | B3 drop 3 | B3 drop 6 | B3 drop 1 | B3 drop 4 | B3 drop 7 | B3 drop 2 | B4 drop 4 | B4
drop 6 | B4 drop 3 | B4 drop 1 | B4 drop 8 | B4 drop 2 | B5 drop 4 | B5 drop 6 | B5 drop 3 | B5 drop 1 | B5 drop 8 | B5
drop 7 | B9 drop 5 | B9 drop 6 | B9 drop 3 | B9 drop 1 | B9 drop 4 | B9 drop 9 | C1 drop 9 | C1 drop 3 | C1 drop 6 | C1
drop 4 | C1 drop 8 | C2 drop 9 | C2 drop 3 | C2 drop 6 | C2 drop 4 | C2 drop 8 | C2 drop 5 | C2 drop 2 | C5 drop 4 | C5
drop 3 | C5 drop 6 | C5 drop 8 | C5 drop 9 | C5 drop 7 | C8 drop 5 | C8 drop 6 | C8 drop 1 | C8 drop 4 | C8 drop 8 | C8
drop 3 | C8 drop 9 | D7 drop 5 | D7 drop 1 | D7 drop 3 | D7 drop 9 | D7 drop 8 | D7 drop 4 | D8 drop 6 | D8 drop 4 | D8
drop 9 | D8 drop 8 | D8 drop 1 | D8 drop 3 | D8 drop 5 | D9 drop 9 | D9 drop 8 | D9 drop 1 | D9 drop 3 | D9 drop 4 | D9
drop 5 | F4 drop 6 | F4 drop 3 | F4 drop 4 | F4 drop 5 | F4 drop 2 | F4 drop 8 | F4 drop 7 | F6 drop 4 | F6 drop 3 | F6
drop 8 | F6 drop 5 | F6 drop 2 | F6 drop 6 | F6 drop 7 | G2 drop 9 | G2 drop 6 | G2 drop 8 | G2 drop 4 | G2 drop 5 | G2
drop 2 | G4 drop 6 | G4 drop 3 | G4 drop 2 | G4 drop 4 | G4 drop 8 | G4 drop 9 | G5 drop 4 | G5 drop 8 | G5 drop 7 | G5
drop 6 | G5 drop 2 | G5 drop 9 | G9 drop 9 | G9 drop 1 | G9 drop 3 | G9 drop 4 | G9 drop 6 | G9 drop 2 | G9 drop 8 | H1
drop 9 | H1 drop 3 | H1 drop 6 | H1 drop 4 | H1 drop 1 | H1 drop 2 | H2 drop 9 | H2 drop 6 | H2 drop 8 | H2 drop 4 | H2
drop 5 | H2 drop 1 | H2 drop 2 | H3 drop 3 | H3 drop 4 | H3 drop 1 | H3 drop 7 | H3 drop 2 | H3 drop 6 | H3 drop 9 | H5
drop 4 | H5 drop 8 | H5 drop 7 | H5 drop 2 | H5 drop 1 | H6 drop 4 | H6 drop 3 | H6 drop 8 | H6 drop 5 | H6 drop 6 | H6
drop 2 | H6 drop 1 | H7 drop 5 | H7 drop 1 | H7 drop 3 | H7 drop 9 | H7 drop 4 | H7 drop 8 | H9 drop 9 | H9 drop 1 | H9
drop 3 | H9 drop 8 | H9 drop 4 | I1 drop 9 | I1 drop 3 | I1 drop 6 | I1 drop 4 | I1 drop 2 | I1 drop 8 | I5 drop 4 | I5
drop 8 | I5 drop 7 | I5 drop 2 | I5 drop 9 | I5 drop 3 | I6 drop 4 | I6 drop 3 | I6 drop 8 | I6 drop 5 | I6 drop 6 | I6
drop 2 | I6 drop 9 | I7 drop 5 | I7 drop 1 | I7 drop 3 | I7 drop 9 | I7 drop 4 | I7 drop 8 | I7 drop 2
Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs
Pointing Pair<A4, B4>: Eliminating 7 from <G4>.
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs | Running boxLineReduction | Running xwings
X-Wing<C1, C5, I1, I5>: Eliminating 5 from <B1, H1, B5, G5, H5>
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs | Running boxLineReduction | Running xwings | Running ywings
Y-Wing<G4<A4, G9>>: Eliminating 7 from <A9>
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs | Running boxLineReduction | Running xwings | Running ywings
Y-Wing<G4<G9, I6>>: Eliminating 7 from <I7>

Cell I7 solved as 6
Running simpleElim | D7 drop 6 | H7 drop 6 | H9 drop 6 | I5 drop 6
Running hiddenSingle
hiddenSingle 6 found for D9
Cell D9 solved as 6
Running simpleElim | Running hiddenSingle
hiddenSingle 6 found for H5
Cell H5 solved as 6
Running simpleElim | Running hiddenSingle
hiddenSingle 9 found for B5
Cell B5 solved as 9
Running simpleElim | B4 drop 9
Running hiddenSingle
hiddenSingle 9 found for F4
Cell F4 solved as 9
Running simpleElim | F6 drop 9
Cell F6 solved as 1
 | I6 drop 1
Cell I6 solved as 7
 | H6 drop 7
Cell H6 solved as 9
 | I1 drop 7
Running hiddenSingle
hiddenSingle 3 found for G5
Cell G5 solved as 3
Running simpleElim | G2 drop 3
Running hiddenSingle
hiddenSingle 3 found for H2
Cell H2 solved as 3
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Running pointingPairs | Running boxLineReduction | Running xwings | Running ywings | Finished
Puzzle :
1 2 _   * - *   * - *     1 _ _   1 2 _   * - *     * - *   * - *   _ 2 _
_ _ _   | 9 |   | 3 |     _ _ _   _ _ _   | 4 |     | 5 |   | 6 |   _ _ _
7 8 _   * - *   * - *     7 _ _   _ _ _   * - *     * - *   * - *   _ 8 _

_ 2 _   * - *   _ _ _     _ _ _   + - +   * - *     * - *   * - *   _ 2 _
_ _ _   | 6 |   _ 5 _     _ 5 _   | 9 |   | 3 |     | 1 |   | 4 |   _ _ _
7 8 _   * - *   _ 8 _     7 _ _   + - +   * - *     * - *   * - *   7 8 _

1 2 _   1 _ _   * - *     * - *   1 2 _   * - *     * - *   _ 2 _   * - *
_ 5 _   _ _ _   | 4 |     | 6 |   _ 5 _   | 8 |     | 3 |   _ _ _   | 9 |
7 _ _   7 _ _   * - *     * - *   _ _ _   * - *     * - *   7 _ _   * - *


* - *   * - *   * - *     * - *   * - *   * - *     _ 2 _   _ 2 _   + - +
| 9 |   | 8 |   | 1 |     | 3 |   | 4 |   | 5 |     _ _ _   _ _ _   | 6 |
* - *   * - *   * - *     * - *   * - *   * - *     7 _ _   7 _ _   + - +

* - *   * - *   * - *     * - *   * - *   * - *     * - *   * - *   * - *
| 3 |   | 4 |   | 7 |     | 2 |   | 8 |   | 6 |     | 9 |   | 5 |   | 1 |
* - *   * - *   * - *     * - *   * - *   * - *     * - *   * - *   * - *

* - *   * - *   * - *     + - +   * - *   + - +     * - *   * - *   * - *
| 6 |   | 5 |   | 2 |     | 9 |   | 7 |   | 1 |     | 4 |   | 8 |   | 3 |
* - *   * - *   * - *     + - +   * - *   + - +     * - *   * - *   * - *


* - *   1 _ _   * - *     1 _ _   + - +   * - *     * - *   * - *   _ _ _
| 4 |   _ _ _   | 6 |     _ 5 _   | 3 |   | 2 |     | 8 |   | 9 |   _ 5 _
* - *   7 _ _   * - *     _ _ _   + - +   * - *     * - *   * - *   7 _ _

_ _ _   + - +   _ _ _     * - *   + - +   + - +     _ 2 _   * - *   _ 2 _
_ _ _   | 3 |   _ 5 _     | 4 |   | 6 |   | 9 |     _ _ _   | 1 |   _ 5 _
7 8 _   + - +   _ 8 _     * - *   + - +   + - +     7 _ _   * - *   7 _ _

1 _ _   * - *   * - *     * - *   1 _ _   + - +     + - +   * - *   * - *
_ 5 _   | 2 |   | 9 |     | 8 |   _ 5 _   | 7 |     | 6 |   | 3 |   | 4 |
_ _ _   * - *   * - *     * - *   _ _ _   + - +     + - +   * - *   * - *
```

