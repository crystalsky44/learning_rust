# states you need to cover (set of an array)
states_needed = set(["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]) 

# hashmaps of stations and covered states pair
stations = {}
stations["kone"] = set(["id", "nv", "ut"])
stations["ktwo"] = set(["wa", "id", "mt"])
stations["kthree"] = set(["or", "nv", "ca"])
stations["kfour"] = set(["nv", "ut"])
stations["kfive"] = set(["ca", "az"])

# an array you store your final result, which will be a set of stations
final_stations = set()

# best station from the first loop(?)
best_station = None
# create a set of the states covered from the best_station
states_covered = set()
# 
for station, states_for_station in station.item():

