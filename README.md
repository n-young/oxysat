# OxySat
Paul Biberstein + Nick Young

## Overview

This project was completed as part of a project for CSCI 1710 - Logic for Systems. See the assignment handout [here](https://docs.google.com/document/d/1MEuNvL6g4Vn_WUJXJBmvvqFjamjfDo0o0Ye-H2ZmaPg/preview).

OxySat is a SAT-solver written in Rust. It ingests input files in [DIMACS CNF format](https://people.sc.fsu.edu/~jburkardt/data/cnf/cnf.html), and outputs in DIMACS format as well. Usae is described below.

Our solver utilizes unit clause elimination and pure literal elimination to prune the search space. It also uses heuristics to decide which variable to branch on. Future work includes implementing multithreading. 

## How to run

Run `cargo run <file>` or `./run.sh <file>` to run.

Run `cargo test` to run unit tests.

Custom testing files are in `test/`.

Look in `oracle/` for the testing oracle README.
