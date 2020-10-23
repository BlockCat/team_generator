from itertools import combinations
from mip import *

# Objective: Maximize x_i
# Constraints:
# x_i in [0, 1]
# connected x_i and x_j:  x_i + x_j <= 1

range_count = 12
combos = [set(i) for i in combinations(range(range_count), 4)]
master = Model("Teams problem", sense=MAXIMIZE)
nodes = [master.add_var(var_type=BINARY, name=str(i).replace(' ', '')) for i in combos]
print("initiated nodes")
for (combo1, i) in zip(nodes, combos):    
    for (combo2, j) in zip(nodes, combos):        
        if (i == j):
            continue
        if len(i.intersection(j)) >= 2:
            assert len(i) == 4
            assert len(j) == 4
            master += combo1 + combo2 <= 1
            
master += xsum(node for node in nodes) >= 10
master.objective = maximize(xsum(node for node in nodes))
master.write("model_" + str(range_count) + ".lp")

status = master.optimize()

print(status)
print("There are: " + str(master.objective_value) + "teams")

f = open("result.txt", "w")
f.write("There are: " + str(master.objective_value) + " teams")
f.close()