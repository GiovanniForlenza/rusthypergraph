import hypergraphx as hx

edge_list = [(1, 2), (2, 3), (4, 3, 5, 6, 8), (2, 3, 5, 6), (7, 4, 6)]
edge_list_2 = [(1, 2), (2, 3), (4, 3, 5, 6, 8), (2, 3, 5, 6), (7, 4, 6), (7, 5, 6)]
weighted = True
weights = [1.0, 2.0, 1.0, 3.0, 1]
weights_2 = [1.0, 2.0, 1.0, 3.0, 1, 9]
metadata = {0: {"a": "b"}, 1: {"a": "b"},2: "value1",3: "value1",4: "value1",5: "value1",6: "value1",7: "value1"}
metadata_err = {"key_2": "value1"}

hypergraph = hx.Hypergraph(
    edge_list=edge_list,
    weighted=weighted,
    weights=weights,
    # metadata= metadata
)

# print(hypergraph.get_nodes(metadata=True))
# edges = hypergraph.get_edges()
# for edge in edges:
#     meta = hypergraph.get_meta(edge)
#     print(meta)

# print(hypergraph.get_attr_meta((1,2), 0))

# GET
def print_attr_meta(hypergraph=hx.Hypergraph):
    result = hypergraph.get_attr_meta(3, "type")
    print(result)
    # Assert: verifica che il risultato sia quello atteso
    # assert result == node

def print_get_edges(hypergraph=hx.Hypergraph):
    # Stampa tutti gli edge
    edges = hypergraph.get_edges()
    print(edges)
    # Assert: verifica che il numero di edge restituiti sia corretto
    # assert edges == [(1, 2), (2, 3), (3, 4, 5, 6, 8), (2, 3, 5, 6), (4, 6, 7)]

    # Stampa gli edge con size=3, subhypergraph=True, up_to=True, keep_isolated_nodes=True
    subgraph = hypergraph.get_edges(size=3, subhypergraph=True, up_to=True, keep_isolated_nodes=True)
    print(subgraph)
    # Assert: verifica che il subgraph sia una istanza di Hypergraph
    # assert isinstance(subgraph, hx.Hypergraph)

def print_get_nodes(hypergraph=hx.Hypergraph):
    nodes = hypergraph.get_nodes()
    print(nodes)
    # Assert: verifica che il numero di nodi restituiti sia corretto
    # assert nodes == [1, 2, 3, 4, 5, 6, 8, 7]

    nodes_with_metadata = hypergraph.get_nodes(metadata=True)
    print(nodes_with_metadata)
    # Assert: verifica che i nodi con metadata siano corretti
    # assert nodes_with_metadata == [(1, {'type': 'node', 'name': 1}), (2, {'type': 'node', 'name': 2}), (3, {'type': 'node', 'name': 3}), (4, {'type': 'node', 'name': 4}), (5, {'type': 'node', 'name': 5}), (6, {'type': 'node', 'name': 6}), (8, {'type': 'node', 'name': 8}), (7, {'type': 'node', 'name': 7})]

def print_get_orders(hypergraph=hx.Hypergraph):
    orders = hypergraph.get_orders()
    print(orders)
    # Assert: verifica che il numero di ordini restituiti sia corretto
    # assert orders == [1, 1, 4, 3, 2]

def print_get_sizes(hypergraph=hx.Hypergraph):
    sizes = hypergraph.get_sizes()
    print(sizes)
    # Assert: verifica che il numero di dimensioni restituiti sia corretto
    # assert sizes == [2, 2, 5, 4, 3]

def print_get_weight(hypergraph=hx.Hypergraph):
    edge = [2, 3, 5, 6]
    weight = hypergraph.get_weight(edge=edge)
    print(weight)
    # Assert: verifica che il peso dell'edge sia corretto
    # assert weight == 3.0

def print_get_weights(hypergraph=hx.Hypergraph):
    order = 2
    weights = hypergraph.get_weights(order=order, size=None, up_to=False)
    print(weights)
    # Assert: verifica che il numero di pesi restituiti sia corretto
    # assert weights == [1]

def print_is_uniform(hypergraph=hx.Hypergraph):
    is_uniform = hypergraph.is_uniform()
    print(is_uniform)
    # Assert: verifica che il risultato sia un booleano
    # assert uniform = False

def print_is_weighted(hypergraph=hx.Hypergraph):
    is_weighted = hypergraph.is_weighted()
    print(is_weighted)
    # Assert: verifica che il grafo sia pesato
    # assert is_weighted = True

def print_max_order(hypergraph=hx.Hypergraph):
    max_order = hypergraph.max_order()
    print(max_order)
    # Assert: verifica che l'ordine massimo sia corretto
    # assert max_order == 4

def print_max_size(hypergraph=hx.Hypergraph):
    max_size = hypergraph.max_size()
    print(max_size)
    # Assert: verifica che la dimensione massima sia corretta
    # assert max_size == 5

def print_num_edges(hypergraph=hx.Hypergraph):
    num_edges = hypergraph.num_edges(order=None, size=None, up_to=False)
    print(num_edges)
    # Assert: verifica che il numero di edge sia corretto
    # assert num_edges == 5

def print_num_nodes(hypergraph=hx.Hypergraph):
    num_nodes = hypergraph.num_nodes()
    print(num_nodes)
    # Assert: verifica che il numero di nodi sia corretto
    # assert num_nodes == 8

def print_get_meta(hypergraph = hx.Hypergraph):
    meta = hypergraph.get_meta(obj=3)
    print(meta)
    # assert : {'type': 'node', 'name': 3}

def print_get_incident_edges(hypergraph = hx.Hypergraph):
    edge = hypergraph.get_incident_edges(node = 5, order = None, size = None)
    print(edge)
    # assert : [(2, 3, 5, 6), (3, 4, 5, 6, 8)]

def print_get_neighbors(hypergraph = hx.Hypergraph):
    neighbors = hypergraph.get_neighbors(node= 8)
    print(neighbors)
    # assert : {3, 4, 5, 6}

def print_get_mapping(hypergraph = hx.Hypergraph):
    leb = hypergraph.get_mapping()
    mapping = dict(zip(leb.classes_, leb.transform(leb.classes_)))
    print(mapping)
    # assert : {1: 0, 2: 1, 3: 2, 4: 3, 5: 4, 6: 5, 7: 6, 8: 7}

# SET

def print_set_meta(hypergraph = hx.Hypergraph):
    hypergraph.set_meta(obj=1 , attr={"test": "prova"})
    print(hypergraph.get_attr_meta(obj= 1, attr= "test"))
    # assert : prova


def print_set_weight(hypergraph = hx.Hypergraph):
    hypergraph.set_weight(edge = (1,2), weight=3)
    print(hypergraph.get_weight(edge=(1,2)))
    # assert : 3

# CHECK

def print_check_edge(hypergraph = hx.Hypergraph):
    print(hypergraph.check_edge((2, 3, 5, 6)))
    print(hypergraph.check_edge((2, 6)))
    # assert : 1- True 2- False

def print_check_node(hypergraph = hx.Hypergraph):
    print(hypergraph.check_node(5))
    print(hypergraph.check_node(22))
    # assert : 1- True 2- False

# ADD

def print_add_edge(weighted = weighted):
    hypergraph = hx.Hypergraph()
    hypergraph.add_edge(edge=(22,33))
    print(hypergraph.get_edges())
    # assert : [(22, 33)]

def print_add_edges():
    hypergraph = hx.Hypergraph(weighted=True)
    edges = [(1,2,3),(2,4,5,6),(5,6,7),(1,3),(1,7,6,4)]
    weights = [1,1,4,6,2]
    hypergraph.add_edges(edge_list=edges, weights=weights)
    print(hypergraph.get_edges())
    # assert : [(1,2,3),(2,4,5,6),(5,6,7),(1,3),(1,7,6,4)]

def print_add_node():
    hypergraph = hx.Hypergraph()
    hypergraph.add_node(node= 2)
    print(hypergraph.get_nodes())
    # assert : [2]

def print_add_nodes():
    hypergraph = hx.Hypergraph()
    nodes = [2,4,5,6,7,8]
    hypergraph.add_nodes(node_list=nodes)
    print(hypergraph.get_nodes())
    # assert : [2, 4, 5, 6, 7, 8]

# REMOVE

def print_remove_edge(hypergraph = hx.Hypergraph):
    hypergraph.remove_edge(edge = (2, 3, 5, 6))
    print(hypergraph.get_edges())
    # assert : [(1, 2), (2, 3), (3, 4, 5, 6, 8), (4, 6, 7)]

def print_remove_edges(hypergraph = hx.Hypergraph):
    edge_list = [(1, 2), (2, 3)]
    hypergraph.remove_edges(edge_list = edge_list)
    print(hypergraph.get_edges())
    # assert : [(3, 4, 5, 6, 8), (4, 6, 7)]

def print_remove_node(hypergraph = hx.Hypergraph, keep_edges = bool):
    hypergraph.remove_node(node = 3, keep_edges = keep_edges)
    print(hypergraph.get_nodes(metadata = False))
    # assert : [1, 2, 4, 5, 6, 8, 7]

def print_remove_nodes(hypergraph = hx.Hypergraph, keep_edges = bool):
    nodes = [6,7]
    hypergraph.remove_nodes(node_list = nodes, keep_edges = keep_edges)
    print(hypergraph.get_nodes(metadata = False))
    #assert : [1, 2, 4, 5, 8]

# OTHER

def print_distribution_sizes(hypergraph = hx.Hypergraph):
    print(hypergraph.distribution_sizes())
    #assert : {2: 2, 5: 1, 4: 1, 3: 1}

def print_subhypergraph(hypergraph = hx.Hypergraph):
    nodes = [1, 2, 4]
    subhy = hypergraph.subhypergraph(nodes = nodes)
    print(subhy.get_nodes(metadata = False))
    print(subhy.get_edges())
    # assert : node [1, 2, 4] edge [(1, 2)]

def print_subhypergraph_by_order(hypergraph = hx.Hypergraph):
    orders = [2, 3]
    subhy = hypergraph.subhypergraph_by_orders(orders=orders, keep_nodes=False)
    print(subhy.get_nodes(metadata = False))
    print(subhy.get_edges())


# Eseguiamo tutte le funzioni di test
# 
print_attr_meta(hypergraph=hypergraph)
# print_get_edges(hypergraph=hypergraph)
# print_get_nodes(hypergraph=hypergraph)
# print_get_orders(hypergraph=hypergraph)
# print_get_sizes(hypergraph=hypergraph)
# print_get_weight(hypergraph=hypergraph)
# print_get_weights(hypergraph=hypergraph)
# print_is_uniform(hypergraph=hypergraph)
# print_is_weighted(hypergraph=hypergraph)
# print_max_order(hypergraph=hypergraph)
# print_max_size(hypergraph=hypergraph)
# print_num_edges(hypergraph=hypergraph)
# print_num_nodes(hypergraph=hypergraph)

# print_get_meta(hypergraph=hypergraph)
# print_get_incident_edges(hypergraph=hypergraph)
# print_get_neighbors(hypergraph=hypergraph)
# print_get_mapping(hypergraph=hypergraph)

# print_set_meta(hypergraph=hypergraph)
# print_set_weight(hypergraph=hypergraph)

# print_check_edge(hypergraph=hypergraph)
# print_check_node(hypergraph=hypergraph)

# print_add_edge(weighted=weighted)
# print_add_edges()
# print_add_node()
# print_add_nodes()

# print_remove_edge(hypergraph=hypergraph)
# print_remove_edges(hypergraph=hypergraph)
# print_remove_node(hypergraph=hypergraph, keep_edges=False)
# print_remove_nodes(hypergraph=hypergraph, keep_edges=False)

# print_distribution_sizes(hypergraph=hypergraph)
# print_subhypergraph(hypergraph= hypergraph)
# print_subhypergraph_by_order(hypergraph=hypergraph)