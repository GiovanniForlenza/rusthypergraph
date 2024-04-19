from hypergraphx.core.temporal_hypergraph import TemporalHypergraph
from hypergraphx.generation.activity_driven import HOADmodel
from rusthypergraph import hoad_model
import time 

n = 10
activities_per_order = {1: [0.5]*n}
t = 100
# Misura il tempo di esecuzione del modello Rust
start_rust = time.time()
rust_model = hoad_model(n, activities_per_order, t)
end_rust = time.time()
rust_execution_time = end_rust - start_rust
print("Tempo di esecuzione del modello Rust:", rust_execution_time, "secondi")

# Misura il tempo di esecuzione del modello Python
start_python = time.time()
python_model = HOADmodel(n, activities_per_order=activities_per_order, time=t)
end_python = time.time()
python_execution_time = end_python - start_python
print("Tempo di esecuzione del modello Python:", python_execution_time, "secondi")