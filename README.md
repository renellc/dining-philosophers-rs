# dining-philosophers-rs
The classic synchronization problem implemented in Rust. This is was done to practice writing concurrent programs in Rust.

## Introduction
The Dining Philosophers Problem is a commonly used example problem to illustrate the synchronization issues one would face when writing concurrent programs. It was originally thought up of for a student exam by Edsger Dijkstra himself, and was later formulated by Tony Hoare into the version we know today. This problem serves as a classic exercise on how to deal with multiple threads competing for resources in memory.

## The Problem
A group of extremely reserved philosophers are to sit at a round table and eat bowls of pasta. Forks are set at the table between each pair of adjacent philosophers. Because they are all extremely reserved, each philosopher only either thinks to themselves or eats pasta at the table. However, one can only eat their bowl of pasta if they have a fork in each hand. Each fork can only be held by one philosopher at a time, and a fork cannot be used by another philosopher if it is already being used by another philosopher. A philosopher can only pick up the forks directly to their right and left when they are available and cannot reach across the table to retrieve a fork.

A philosopher alternates between thinking to themselves and eating pasta. When they decide to think to themselves after eating pasta, they must set both of their forks down on the table before proceeding to think. One can assume that they have an infinite amount of pasta to eat.

Is it possible to ensure that no philosophers will starve? A philosopher starves if it is never able to get forks to eat their bowl of pasta.

## Solution
The solution used here is explained within comments found in the source code. I found my solution is similar, if not exactly the same as the solution proposed by K. Mani Chandi and J. Misra (see [here](https://en.wikipedia.org/wiki/Dining_philosophers_problem#Chandy/Misra_solution)). I advise reading the wikipedia article on their solution, as it formalizes what I have done here in a digestible format.

## Running
Clone the repo and `cd` into the repo folder. `cargo build` and type `cargo run` to run the program with default parameters. To change the number of philosophers, type `cargo run -- num`, where `num` is the number of philosophers. Currently I have not added a parameter that allows you to change how long the simulation runs for. By default, the simulation runs for 3 minutes.

## Resources
More information on the dining philosophers problem can be found [here](https://en.wikipedia.org/wiki/Dining_philosophers_problem).
