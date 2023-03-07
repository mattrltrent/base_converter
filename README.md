
  
# Base converter CLI in rust

  

<img  src="https://github.com/mattrltrent/base_converter/blob/main/assets/demo.JPG?raw=true"  width=""  height=""  style="display: inline"/>

  

## Notes

  
  

Built for my personal use to help quickly convert between numbers with different bases. **Has some limitations since it's not really meant for widespread use :D**

  

Technically, yes, you can use `bc` to do this too — but this tool also walks you through how to get the solution with detailed steps... and it was kinda fun to make.

  
  

## Commands

  

  

-  `convert help` -> Opens help menu

  

-  `convert version` -> Lists the application version

  

-  `convert table` -> Displays the table used for converting between bases

  

-  `convert A B C` -> Converts string A of base B to a new string of base C

  

-  `convert A B C --explain` -> Does the same as above, **with the addition of an in-depth explanation**

## Installation

Again, the tool was designed for personal use so it's not perfect, but alas, it may be helpful for others.

1. Make sure you have Rust installed on your system (not technically needed, but makes the tool easier to distribute). If you don't, use [this link](https://www.rust-lang.org/tools/install).
2. Clone the repo: `git clone https://github.com/mattrltrent/base_converter`.
3. In the project directory, run `cargo build --release`.
4. In the project directory, run `cargo install --path .` (don't miss the ending `.`).
5. You should now be able to run the tool globally. To be sure, check if you can run `convert version`.