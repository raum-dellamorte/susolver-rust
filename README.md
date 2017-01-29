SuSolver
========

A Sudoku Solver written in Rust.

Solves logically using documented strategies .

Current State
-------------

It works!  It only does simple elimination at the moment, but it's a working sudoku solver written in Rust! Should be able to solve any 

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
__3   ___   __3     *|*   __3   1_3     __3   1_3   *|*
__6   4_6   ___     -5-   ___   ___     4_6   4_6   -2-
_89   _89   __9     *|*   _8_   ___     789   78_   *|*

*|*   *|*   _23     *|*   __3   *|*     *|*   __3   __3
-1-   -7-   ___     -4-   ___   -6-     -5-   ___   ___
*|*   *|*   __9     *|*   _8_   *|*     *|*   _8_   _89

*|*   _2_   _23     *|*   *|*   123     __3   1_3   __3
-5-   4_6   ___     -7-   -9-   ___     4_6   4_6   4__
*|*   _8_   ___     *|*   *|*   ___     _8_   _8_   _8_


___   *|*   1__     1__   ___   1__     *|*   ___   ___
___   -3-   _5_     ___   456   _5_     -2-   456   45_
789   *|*   7_9     _89   78_   7__     *|*   78_   78_

___   ___   *|*     __3   *|*   __3     *|*   __3   __3
___   _5_   -4-     ___   -2-   _5_     -1-   _56   _5_
789   _89   *|*     _89   *|*   7__     *|*   78_   78_

_2_   12_   *|*     1_3   __3   1_3     __3   *|*   __3
___   _5_   -6-     ___   45_   _5_     4__   -9-   45_
78_   _8_   *|*     _8_   78_   7__     78_   *|*   78_


_23   _2_   _23     _23   *|*   *|*     __3   __3   *|*
___   _5_   _5_     ___   -1-   -8-     4__   45_   -6-
7_9   __9   7_9     ___   *|*   *|*     7_9   7__   *|*

__3   ___   *|*     *|*   __3   *|*     __3   *|*   *|*
___   _5_   -8-     -6-   _5_   -4-     ___   -2-   -1-
7_9   __9   *|*     *|*   7__   *|*     7_9   *|*   *|*

*|*   12_   123     _23   __3   *|*     __3   __3   __3
-4-   _56   _5_     ___   _5_   -9-     ___   _5_   _5_
*|*   ___   7__     ___   7__   *|*     78_   78_   78_
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