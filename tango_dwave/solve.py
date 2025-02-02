import dimod
from dwave.system import LeapHybridCQMSampler


N = 6 # 6x6 grid
FILENAME = "problem.txt"


class Spot:
	MOON = 0
	SUN = 1
	EMPTY = 2
	def __init__(self, x, y, status):
		self.x = x
		self.y = y
		self.status = status

class Link:
	X = 0
	EQ = 1
	def __init__(self, x1, y1, x2, y2, link_type):
		# self.x1 = x1 - 1
		# self.y1 = y1 - 1
		# self.x2 = x2 - 1
		# self.y2 = y2 - 1
		self.x1 = y1 - 1
		self.y1 = x1 - 1
		self.x2 = y2 - 1
		self.y2 = x2 - 1
		self.link_type = link_type


def read_problem(filename):
	with open(filename, "r") as f:
		lines = f.readlines()

	grid = []
	for i in range(N):
		grid.append([])
		spots = lines[i].strip().split(",")
		for j in range(N):
			if spots[j] == "M":
				grid[i].append(Spot(i, j, Spot.MOON))
			elif spots[j] == "S":
				grid[i].append(Spot(i, j, Spot.SUN))
			else:
				grid[i].append(Spot(i, j, Spot.EMPTY))

	links = []
	for line in lines[N:]:
		x1, y1, x2, y2, link_type = line.strip().split(",")
		if link_type == "X":
			links.append(Link(int(x1), int(y1), int(x2), int(y2), Link.X))
		else:
			links.append(Link(int(x1), int(y1), int(x2), int(y2), Link.EQ))
	
	return grid, links



grid, links = read_problem(FILENAME)


cqm = dimod.ConstrainedQuadraticModel()

x = {(i, j): dimod.Binary(f'x_{i}_{j}') for i in range(N) for j in range(N)}

# Initial placements
for i in range(N):
	for j in range(N):
		if grid[i][j].status != Spot.EMPTY:
			cqm.add_constraint(
				x[i, j] == grid[i][j].status,
				label=f'spot_{i}_{j}'
			)

# Row and column balance
for i in range(N):
    cqm.add_constraint(sum(x[i, j] for j in range(N)) == N // 2, label=f'row_balance_{i}')
    cqm.add_constraint(sum(x[j, i] for j in range(N)) == N // 2, label=f'col_balance_{i}')

# Add adjacency constraints for rows
for i in range(N):
    for j in range(N - 2):
        cqm.add_constraint(x[i, j] + x[i, j + 1] + x[i, j + 2] <= 2, label=f'row_adj_{i}_{j}')
        cqm.add_constraint((1 - x[i, j]) + (1 - x[i, j + 1]) + (1 - x[i, j + 2]) <= 2, label=f'row_adj_moon_{i}_{j}')

# Add adjacency constraints for columns
for j in range(N):
    for i in range(N - 2):
        cqm.add_constraint(x[i, j] + x[i + 1, j] + x[i + 2, j] <= 2, label=f'col_adj_{i}_{j}')
        cqm.add_constraint((1 - x[i, j]) + (1 - x[i + 1, j]) + (1 - x[i + 2, j]) <= 2, label=f'col_adj_moon_{i}_{j}')

# Add link constraints
for link in links:
	if link.link_type == Link.X:
		cqm.add_constraint(
			x[link.x1, link.y1] + x[link.x2, link.y2] == 1,
			label=f'link_{link.x1}_{link.y1}_{link.x2}_{link.y2}'
		)
	else:
		cqm.add_constraint(
			x[link.x1, link.y1] - x[link.x2, link.y2] == 0,
			label=f'link_{link.x1}_{link.y1}_{link.x2}_{link.y2}'
		)
		


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


	for i in range(N):
		for j in range(N):
			if (i, j) in answer:
				if answer[(i, j)] == 1:
					print("S", end="")
				else:
					print("M", end="")
			else:
				print(" ", end="")
		print()