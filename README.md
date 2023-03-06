
# Base converter CLI in rust

<img src="https://github.com/mattrltrent/base_converter/blob/main/assets/ex.JPG?raw=true" width="" height="" style="display: inline"/>

## Notes

  

I built this one evening because I wanted a quick way to convert between bases for uni classes. Conversions work with number having a base 2 through 36.

  

Technically, yes, you can use `bc` to do this too — but my program walks you through how to get the solution with detailed steps, and it was kinda fun to make.

  

Currently, runs with `cargo run <args>`, but I'll clean this up soon. Haven't really polished it either yet, heh.

  

## Commands (assuming `convert` is root command)

  

-  `convert help` -> Opens help menu

-  `convert table` -> Displays the table used for converting between bases

-  `convert A B C` -> Converts string A of base B to a new string of base C

-  `convert A B C --explain` -> Does the same as above, **with the addition of an in-depth explanation**