import dimod
from dwave.system import LeapHybridCQMSampler


FILENAME = "problem.txt"

class Spot:
	EMPTY = 0
	QUEEN = 1
	EMPTY = 2
	def __init__(self, x, y, status):
		self.x = x
		self.y = y
		self.status = status


def read_problem(filename):
	with open(filename, "r") as f:
		lines = f.readlines()

	relations = {}
	n = 0
	for i, line in enumerate(lines):
		n += 1
		for j, c in enumerate(line.strip()):
			v = relations.get(c, [])
			v.append((i, j))
			relations[c] = v
		

	return n, relations


n, relations = read_problem(FILENAME)


cqm = dimod.ConstrainedQuadraticModel()

x = {(i, j): dimod.Binary(f'x_{i}_{j}') for i in range(n) for j in range(n)}

# One in each row and column
for i in range(n):
    cqm.add_constraint(sum(x[i, j] for j in range(n)) == 1, label=f'row_{i}')
    cqm.add_constraint(sum(x[j, i] for j in range(n)) == 1, label=f'col_{i}')	

# No adjacent queens (including diagonals)
for i in range(n):
    for j in range(n):
        for di in [-1, 0, 1]:
            for dj in [-1, 0, 1]:
                if di == 0 and dj == 0:
                    continue
                ni, nj = i + di, j + dj
                if 0 <= ni < n and 0 <= nj < n:
                    cqm.add_constraint(x[i, j] + x[ni, nj] <= 1, label=f'adj_{i}_{j}_{ni}_{nj}')

# One per relation
for c, poses in relations.items():
    # Ensure exactly one Crown in the region
    cqm.add_constraint(sum(x[i, j] for (i, j) in poses) == 1, label=f'region_{c}')


sampler = LeapHybridCQMSampler()
solutions = sampler.sample_cqm(cqm, time_limit=5)
feasaible = solutions.filter(lambda row: row.is_feasible)

if len(feasaible) == 0:
	print("No feasible solutions found.")
else:
	# schedule = [k for k, val in best_sample.items() if val == 1]

	best_sample = feasaible.first.sample
	# answer = [(k, val) for k, val in best_sample.items()]
	answer = {}
	for k, val in best_sample.items():
		x = k.split("_")[1]
		y = k.split("_")[2]
		key = (int(x), int(y)) # type: ignore
		answer[key] = val


	for i in range(n):
		for j in range(n):
			if (i, j) in answer:
				if answer[(i, j)] == 1:
					print("Q", end="")
				else:
					print(".", end="")
			else:
				print("x", end="")
		print()