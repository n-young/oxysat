# Oracle

This testing oracle produces cnf files with various numbers of clauses and literals, runs the solver and the cnf file, and checks that the resulting assignment actually produces a true result.

## Notes
* Literals and Clauses are represented with classes that were taken from the Python stencil code.
* Parameters like how many clauses to generate, how many literals to include per clause, and how many literals to have total, are customizable at the top of the file.
* By default the oracle checks 100 tests.