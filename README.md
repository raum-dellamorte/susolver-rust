SuSolver
========

A Sudoku Solver written in Rust.

Solves logically using documented strategies .

Current State
-------------

It works!  It only does simple elimination at the moment, but it's a working sudoku solver written in Rust! It should be able to solve any Sudoku marked as 'Easy'.

Why?
----

Proof of Concept, that's why!

Initially written to learn Mirah, a JVM language, then in Haskell (for Science!), now I'm doing it again in Rust (also for Science!).

The Mirah version is the most complete, including complex strategies like [Sword Fish](http://www.sudokuwiki.org/Sword_Fish_Strategy) and [X Cycles](http://www.sudokuwiki.org/X_Cycles), but I haven't touched it in a while.

The Haskell version I wrote to learn Haskell and it was a good thing I did because Rust has a lot in common with Haskell. I think that version gets as far as [Naked Pairs](http://www.sudokuwiki.org/Naked_Candidates#NP).

Haskell had a learning curve from Hell, and if you've ever looked at Haskell code without knowing Haskell you probably guessed that about it.  Rust has a much more friendly appearance and I think it inspires false confidence, only to have the compiler laugh derisively when you try to do something simple like write a function to modify a value in a Struct.  It mockingly says "I had a vicious learning curve all along!  My concept of Ownership and Borrowing makes sense to the ears, yes?  But just you try and apply the concept!"

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