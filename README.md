# okfrick

- has one memory cell
- has less than 5 characters hopefully
- works well
- is turing complete (possibly)

okfrick has only 5 characters
`+!?()`

these 5 characters are the only thing you need for wasting hours of your life coding useless things using this language.

the memory pointer has a size of u8 so you get sad faster.

lets begin by looking at how these characters work

---

`+` - It increments the pointer by one, incase the memory cell overflows (incrementing the cell when its value is 255) it goes to 0. 

`!` - it outputs the decimal of the memory cell as ASCII to stdout.

`?` - it gets the first char from input and overwrites its decimal value on the memory cell

`(` - starting of a loop (only enters if the value of the memory cell is **not** 0)

`)` - ending of the loop (only exists if the memory cell **is** 0)

---
Here you go, a very epic language.

Also heres a small tip which im sure most of you figured out already `(+)` resets the memory cell to 0.

ok now do epic projects and send me then and if u find a bug or an error open an issue or open a PR. 
