import random
import math
import sys
import subprocess

TEST_FILE_NAME = "tmp.cnf"
OUTPUT_FILE_NAME = "tmp.txt"
CLAUSE_COUNT = 10
LITERAL_COUNT = 5
MAX_LITERALS_PER_CLAUSE = 3


class Literal:
    def __init__(self, name, sign):
        self.name = name  # integer
        self.sign = sign  # boolean

    def __repr__(self):
        return ("-" if not self.sign else "") + str(self.name)

    def __eq__(self, other):
        if type(other) != Literal:
            return False
        return self.name == other.name and self.sign == other.sign

    def __hash__(self):
        return hash((self.name, self.sign))


class Clause:
    def __init__(self, id, literal_set):
        self.id = id
        self.literal_set = literal_set

    def __repr__(self):
        return f"{self.id}: {str(self.literal_set)}"

    def __eq__(self, other):
        if type(other) != Clause:
            return False
        return self.id == other.id


def generate_test(clause_count, literals, max_literals_per_clause):
    """ Generate a cnf file in non-serialized form"""
    clauses = [
        generate_clause(literals, max_literals_per_clause, i)
        for i in range(clause_count)
    ]
    return clauses


def generate_clause(literals, max_literals_per_clause, CID):
    """ Generate a single clause """
    # We are going to pick a random amount of each literal to include in the clause.
    # This initializes a dict to keep track of the count of each.
    literal_count = {literal: 0 for literal in literals}
    for i in range(math.ceil(random.random() * max_literals_per_clause)):
        literal_count[random.choice(literals)] += 1

    literal_set = []
    # Find the total number of each literal we decided on and add them to an array
    for literal in literals:
        for _ in range(literal_count[literal]):
            literal_set.append(literal)

    return Clause(CID, literal_set)


def write_test(test, filename):
    """ Serialize the test and write it to a file """
    out = ""
    for clause in test:
        out += " ".join([str(literal) for literal in clause.literal_set])
        out += " 0\n"

    with open(f"{filename}", "w") as f:
        f.write(out)


def run_test(input_file, output_file):
    """ Emit a shell command to run the solver and the emitted file """
    with open(f"{output_file}", "w") as f:
        subprocess.run(["cargo", "run", "--release", f"{input_file}"], stdout=f)
    print("FINISHED RUNNING SOLVER")


def read_output(filename):
    """ Read output file from solver and deserialize """
    lines = []
    with open(f"{filename}", "r") as f:
        lines = f.readlines()

    values = ""
    for line in lines:
        if line[0] == "v":
            values = line[1:]

    raw_literals = values.split(" ")
    raw_literals = [s.strip() for s in raw_literals]

    # This is a dict that maps the id of a variable (an integer greater than 0)
    # to its boolean assignment in the solution.
    variable_to_value_resolver = {}
    for literal in raw_literals:
        if literal == "" or literal == "0":
            continue
        if literal[0] == "-":
            variable_to_value_resolver[int(literal[1:])] = False
        elif literal[0] in "123456789":
            variable_to_value_resolver[int(literal)] = True
        else:
            print("ERROR: tried to parse literal: " + literal)

    return variable_to_value_resolver


def evaluate_solution(variable_to_value_resolver, test):
    """ Evaluates a solution to see if its valid """
    # If we take every clause and map it to its assigned boolean values,
    # it should be true that at least one value is true
    for clause in test:
        if not any(
            [
                variable_to_value_resolver[literal.name]
                if literal.sign
                else not variable_to_value_resolver[literal.name]
                for literal in clause.literal_set
            ]
        ):
            print("Clause not satisfied by assignment:")
            print("Clause:")
            print(clause)
            print("Assignment:")
            print(variable_to_value_resolver)
            sys.exit(1)

    print("SOLUTION IS VALID")


if __name__ == "__main__":
    # Generate all literals
    all_literals = [Literal(i, True) for i in range(1, LITERAL_COUNT + 1)] + [
        Literal(i, False) for i in range(1, LITERAL_COUNT + 1)
    ]

    for i in range(100):
        test = generate_test(CLAUSE_COUNT, all_literals, MAX_LITERALS_PER_CLAUSE)
        write_test(test, TEST_FILE_NAME)
        run_test(TEST_FILE_NAME, OUTPUT_FILE_NAME)
        variable_to_value_resolver = read_output(OUTPUT_FILE_NAME)
        if not variable_to_value_resolver:
            # If we couldn't parse the variable_to_value_resolver properly its because
            # the result was unsat
            print("UNSAT")
        else:
            evaluate_solution(variable_to_value_resolver, test)
