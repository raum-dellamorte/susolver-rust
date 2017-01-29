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

