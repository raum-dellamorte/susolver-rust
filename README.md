SuSolver
========

A Sudoku Solver written in Rust.

Solves logically using documented strategies .

Current State
-------------

It works!  ~~It only does simple elimination at the moment, but it's a working sudoku solver written in Rust! It should be able to solve any Sudoku marked as 'Easy'.~~

~~It now does simple elimination and Hidden Singles.  This was enough to solve the "Hard" puzzle I through at it.  Remaining strategies will be an ever growing pain.~~

Working Strategies:
- Hidden Singles
- Naked Pairs and Triplets
- Hidden Pairs and Triplets (Triplets not tested but should work)

**Road Map:** Check out [The Sudoku Wiki](http://www.sudokuwiki.org/sudoku.htm) for all the horrific strategies I have to teach a machine to do.  I write all my sudoku solving functions based on the descriptions of strategies listed there.  Also, I need to be able to select available puzzle files instead of hard coding which puzzle to load, a way of entering a puzzle from within the program and saving it, and a GUI to make both of those things easier for the user.  Also, I *could* make it a library for use in Python and Ruby or whatever.

Why?
----

Proof of Concept, that's why!

I initially wrote it to learn [Mirah](https://github.com/mirah/mirah), a JVM language with Ruby syntax, static typing, type inference, and no runtime library (meaning it compiles to pure java bytecode). This version is the most complete, including complex strategies like [Sword Fish](http://www.sudokuwiki.org/Sword_Fish_Strategy) and [X Cycles](http://www.sudokuwiki.org/X_Cycles), but I haven't touched it in a while.

Then I wrote it again in Haskell (for Science!) to learn the language and functional strategies. It was a good thing I did because Rust has a lot in common with Haskell. I think the Haskell version gets as far as [Naked Pairs](http://www.sudokuwiki.org/Naked_Candidates#NP).

Now I'm doing it again in Rust (also for Science!).

Haskell had a learning curve from Hell, and if you've ever looked at Haskell code without knowing Haskell you probably guessed that about it.  Rust has a much more friendly appearance and I think it inspires false confidence, only to have the compiler laugh derisively when you try to do something that seems like it should be simple.  It mockingly says "I had a vicious learning curve all along!  My concept of Ownership and Borrowing makes sense to the ears, yes?  But just you try and apply the concept!"  (Update: I'm mostly over that curve now)

It is a picky picky compiler, but there's a warm feeling in the confidence that once you get your code to compile you've got a memory safe executable with 99% of common bugs already squashed.

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
Current "Evil" difficulty output.
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
Running simpleElim | Running hiddenSingle | hiddenSingle 6 found for B1
Cell B1 solved as 6

Running simpleElim | Running hiddenSingle | hiddenSingle 9 found for C1
Cell C1 solved as 9

Running simpleElim | Running hiddenSingle | hiddenSingle 3 found for C4
Cell C4 solved as 3

Running simpleElim | Running hiddenSingle | hiddenSingle 9 found for D6
Cell D6 solved as 9

Running simpleElim | Running hiddenSingle | hiddenSingle 6 found for F5
Cell F5 solved as 6

Running simpleElim | Running hiddenSingle | hiddenSingle 9 found for H3
Cell H3 solved as 9

Running simpleElim | Running hiddenSingle | hiddenSingle 1 found for H1
Cell H1 solved as 1

Running simpleElim | Running hiddenSingle | hiddenSingle 6 found for I4
Cell I4 solved as 6

Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Naked Triplet<D1, D4, D5>: Eliminating [7, 4, 8] from D2
Naked Triplet<D1, D4, D5>: Eliminating [7, 4, 8] from D7
Naked Triplet<D1, D4, D5>: Eliminating [7, 4, 8] from D9
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Hidden Pair<D2, F3>[1, 3]: Eliminating other values.
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Hidden Pair<G8, G9>[5, 6]: Eliminating other values.
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Naked Pair<G8, G9>: Eliminating [6, 5] from H7
Naked Pair<G8, G9>: Eliminating [6, 5] from H8
Naked Pair<G8, G9>: Eliminating [6, 5] from H9
Running simpleElim | Running hiddenSingle | Running nakedPairsTrips | Running hiddenPairsTrips | Finished
Puzzle :
* - *   1 2 _   1 2 _     _ 2 _   * - *   1 2 _     _ 2 _   1 _ _   * - *
| 3 |   _ _ _   _ 5 _     4 5 _   | 9 |   4 _ _     4 5 6   4 5 6   | 8 |
* - *   7 _ _   7 _ _     _ _ _   * - *   _ _ _     7 _ _   _ _ _   * - *

+ - +   1 2 _   1 2 _     * - *   _ _ _   1 2 _     _ 2 3   * - *   1 2 3
| 6 |   _ _ _   _ 5 _     | 7 |   4 5 _   4 _ _     4 5 _   | 9 |   4 5 _
+ - +   _ 8 _   _ 8 _     * - *   _ _ _   _ _ _     _ _ _   * - *   _ _ _

+ - +   1 2 _   * - *     + - +   * - *   * - *     _ 2 _   1 _ _   1 2 _
| 9 |   _ _ _   | 4 |     | 3 |   | 8 |   | 6 |     _ 5 _   _ 5 _   _ 5 _
+ - +   7 _ _   * - *     + - +   * - *   * - *     7 _ _   _ _ _   7 _ _


_ _ _   1 _ 3   * - *     _ _ _   _ _ _   + - +     _ _ 3   * - *   1 _ 3
4 _ _   _ _ _   | 6 |     4 _ _   4 _ _   | 9 |     _ 5 _   | 2 |   _ 5 _
7 8 _   _ _ _   * - *     _ 8 _   7 _ _   + - +     _ _ _   * - *   _ _ _

_ 2 _   * - *   _ 2 _     * - *   * - *   * - *     _ _ _   * - *   _ _ _
4 _ _   | 9 |   _ _ _     | 1 |   | 3 |   | 5 |     4 _ 6   | 7 |   4 _ 6
_ 8 _   * - *   _ 8 _     * - *   * - *   * - *     _ 8 _   * - *   _ _ _

_ 2 _   * - *   1 _ 3     _ 2 _   + - +   _ 2 _     * - *   1 _ 3   1 _ 3
4 _ _   | 5 |   _ _ _     4 _ _   | 6 |   4 _ _     | 9 |   4 _ _   4 _ _
7 8 _   * - *   _ _ _     _ 8 _   + - +   7 8 _     * - *   _ 8 _   _ _ _


_ _ _   _ _ 3   _ _ 3     * - *   * - *   _ _ _     * - *   _ _ _   _ _ _
4 _ _   4 _ _   _ _ _     | 9 |   | 2 |   4 _ _     | 1 |   _ 5 6   _ 5 6
7 8 _   7 8 _   7 8 _     * - *   * - *   7 8 _     * - *   _ _ _   _ _ _

+ - +   * - *   + - +     _ _ _   _ _ _   * - *     _ 2 _   _ _ _   _ 2 _
| 1 |   | 6 |   | 9 |     4 5 _   4 5 _   | 3 |     4 _ _   4 _ _   4 _ _
+ - +   * - *   + - +     _ 8 _   7 _ _   * - *     7 8 _   _ 8 _   7 _ _

* - *   _ 2 3   _ 2 3     + - +   * - *   _ _ _     _ 2 3   _ _ 3   * - *
| 5 |   4 _ _   _ _ _     | 6 |   | 1 |   4 _ _     4 _ _   4 _ _   | 9 |
* - *   7 8 _   7 8 _     + - +   * - *   7 8 _     7 8 _   _ 8 _   * - *
```

