
# How to Create and Use rusthypergraph

## Simple Hypergraph

```python
import rusthypergraph as rhx

edge_list = [(1, 2), (2, 3), (4, 3, 5, 6, 8), (2, 3, 5, 6), (7, 4, 6)]
weighted = True
weights = [1.0, 2.0, 1.0, 3.0, 1]

hypergraph = rhx.Hypergraph(
    edge_list=edge_list,
    weighted=weighted,
    weights=weights
)
```

## How to Add the Node and the Edges

```python
hypergraph.add_node(node=22)

edges = [(1,2,3), (2,4,5,6), (5,6,7), (1,3), (1,7,6,4)]
weights = [1, 1, 4, 6, 2]

hypergraph.add_edges(edges=edges, weights=weights)
```

## How to Remove the Node or the Edge

```python
hypergraph.remove_node(node=3, keep_edges=False)
hypergraph.remove_edge(edge=(2, 3, 5, 6))
```

## Print Hypergraph

```python
print(hypergraph)
```

## How to Create a Subhypergraph

```python
orders = [2, 3]
subhy = hypergraph.subhypergraph_by_orders(orders=orders, keep_nodes=True)
print(subhy.get_nodes(metadata=False))
print(subhy.get_edges())
```
