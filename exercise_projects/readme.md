This folder will hold some sample project ideas I got from LLMs to exercise the book's chapters.

## Temperature Converter:
Description: A command-line program that converts a temperature from Fahrenheit to Celsius and vice-versa.

This project is great for practicing functions, variable binding, and basic arithmetic.

### Concepts You'll Use:
Functions (one for f_to_c, one for c_to_f)
Variables and mutability
Basic arithmetic (+, -, *, /)
User input (from the guessing game chapter)
Control flow (if or match to choose conversion type)

### Simple Steps:
Print a menu asking the user which conversion they want to do.
Read their choice.
Prompt them to enter the temperature.
Read the temperature (remember: input is a String, you need to parse it to a number, e.g., f64).
Use an if or match statement to call the correct conversion function.
Print the result.

## Project 2: Fibonacci Number Generator
This project focuses on control flow (loops) and simple algorithms.
Description: Generate the nth number in the Fibonacci sequence. The sequence is defined as:
F₀ = 0
F₁ = 1
Fₙ = Fₙ₋₁ + Fₙ₋₂

### Concepts You'll Use:
Loops (for or while)
Variables and mutability (you'll need to swap values)
User input and parsing

### Simple Steps:
Prompt the user for a number n.
Handle the base cases (if n is 0 or 1).
For any larger n, use a loop to calculate the value iteratively.
You'll need to track two previous numbers (e.g., a and b).
In each loop iteration, calculate the next number and update your tracked values.