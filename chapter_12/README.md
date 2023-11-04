## An I/O Project: Building a Command Line Program

we’ll make our own version of the classic command line search tool grep (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

Our grep project will combine a number of concepts you’ve learned so far:

1. Organizing code (using what you learned about modules in Chapter 7)
2. Using vectors and strings (collections, Chapter 8)
3. Handling errors (Chapter 9)
4. Using traits and lifetimes where appropriate (Chapter 10)
5. Writing tests (Chapter 11)

We’ll also briefly introduce closures, iterators, and trait objects, which Chapters 13 and 17 will cover in detail.
