# backwards

A backwards programming language. If you can understand this, your a god.

## Purpose

This is not to be intended as a real programming language, but I'm not stopping you for using it. Spread the love. This is for me to learn how a programming language is created. I'll be writing a lexer, parser, type checking and interpreter. This maybe be integrated into my operating system, DeanOS.

## Features/How to

- The function body will be exaluated before the arguments, like in Haskell
- Statements end in a semi-colon
- Assignment is left to right, so asigning 5 to a variable, you would do:
```backwards
5 := five;
```
- Function applications are arguments then function name:
```backwards
(5)fib;
```
- Code is executed upwards, so the following will do an assignment then print:
```backwards
(five)print;
5 := five;
```
So u will read the program from bottome to top
- Indendting is reverces where instead of indenting a line within a if or for... statment, u will have to indent the whole program:
```backwards
 {
return n;
 } if n < 2
```
- To make it even more backwards, the indenting doubles with each indent. It starts off at 1 then 2 then 3 and so on, so here is an example:
```backwards
        { // 8 spaces
    { // 4 spaces
  { // 2 spaces
 { // 1 spaces
("two")print;
 } if i > 1
  } if i > 3
    } (0 := int: i, i < 5, i++) for
        } ()main
```
