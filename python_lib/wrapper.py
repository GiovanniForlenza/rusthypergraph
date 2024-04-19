from rusthypergraph import hoad_model

def hoad_model_wrapper(n = int, activities_per_order = dict, time = int):
    result = hoad_model()
    return result