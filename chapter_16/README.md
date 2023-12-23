# Fearless Concurrency

<!--toc:start-->

- [Fearless Concurrency](#fearless-concurrency)
  - [Here are the topics we’ll cover in this chapter](#here-are-the-topics-well-cover-in-this-chapter)
  <!--toc:end-->

Handling concurrent programming safely and efficiently is another of Rust’s
major goals. Concurrent programming, where different parts of a program execute
independently, and parallel programming, where different parts of a program
execute at the same time, are becoming increasingly important as more computers
take advantage of their multiple processors. Historically, programming in these
contexts has been difficult and error prone: Rust hopes to change that.

## Here are the topics we’ll cover in this chapter

- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library

### Send and Sync

The Send marker trait indicates that ownership of values of the type
implementing Send can be transferred between threads.
Almost every Rust type is Send, but there are some exceptions, including Rc\<T\>
