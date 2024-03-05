# fizzbuzz_in_browser
First and foremost, one needs to have Rust installed on her/his computer in order to build this program. Here are some links to get you started, if you don't know how to do it:
https://doc.rust-lang.org/book/ch01-01-installation.html
https://www.youtube.com/watch?v=jzopcmeekQE

After you have installed everything, git clone this repository to a folder of your preference. Then, on the command line, type "cargo build" (without the double quotes) to build the program. Then type "cargo run" to run the program. Some output on your cmd or terminal will appear. Now move on to the "DESCRIPTION AND HOW TO USE IT" of this document.

=== DESCRIPTION AND HOW TO USE IT ===
 simple program written in the Rust programming language and using the Actix framework for web-development. In this program the user provides as input the starting number for the FizzBuzz game, as well as the end number, in the following format: http://127.0.0.1:8080/fizz_buzz/from={start}%to={end}. For example, if the user types 3 instead of {start} and 16 instead of {end}, he or she will end up with this string: http://127.0.0.1:8080/fizz_buzz/from=3%to=16 . Then all the user has to do is to copy and paste this text into her/his browser and press ENTER, and all the relevant FizzBuzz output will appear. In our example (start number being 3 and end number being 16), the following output will appear in the browser as simple text (without any styling):

fizz!
4
buzz!
fizz!
7
8
fizz!
buzz!
11
fizz!
13
14
fizzbuzz!
16

Observation 1: Error messages will appear if the user provides as input characters or words instead of numbers, or if the start number is greater than the end number.
Observation 2: The number 0 (zero) is divisible by 3, 5 and 15. However, if the user provides 0 as input, the number 0 will appear as text, not "fizz", "buzz" or "fizzbuzz".
Observation 3: The program also works with negative numbers.
