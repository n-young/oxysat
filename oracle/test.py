import random
import os

def generate_test(clause_count, literals, max_literals_per_clause):
    clauses = []
    for i in range(clause_count):
        clauses.append(generate_clause(literals, max_literals_per_clause))

    out = ""
    for clause in clauses:
        out += clause + "\n"

    return out
    
def generate_clause(literals, max_literals_per_clause):
    # Generate all possible literals (positive and negative)
    all_literals = [str(i) for i in range(1,literals+1)] + [str(-i) for i in range(1,literals+1)]
    assert(0 not in all_literals)

    # We are going to pick a random amount of each literal to include in the clause.
    # This initializes a dict to keep track of the count of each.
    literal_count = {literal: 0 for literal in all_literals}
    for i in range(max_literals_per_clause):
        literal_count[random.choice(all_literals)] += 1

    out = ""
    for literal in all_literals:
        out += (f"{literal} ") * literal_count[literal]

    return out + "0"

def write_test(test_text, filename):
    with open(f"tmp/{filename}", "w") as f:
        f.write(test_text)

def read_

if __name__ == "__main__":
    # TODO: tweak this to change parameters
    test = generate_test(500, 100, 5)
    write_test(test, "test.cnf")


