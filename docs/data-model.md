# Data Model

`Graph`, `Vertices`, `Edges` and `DataFrame` are logical representation of the data, rather than phsical representation. The physical representation of the data is stored in the underlying storage engine. The transformation is lazy.

## Transformation

The operations can be characterized:

1. GraphBase to GraphBase
2. GraphBase to DataFrame

## SingleGraph

A logical representation of a single graph. It is a collection of vertices and edges.

## VertexSelectGraph

A `SingleGraph` with a vertex `Selector`.

## EdgeSelectGraph

A `SingleGraph` with a edge `Selector`.

## DataFrame

A DataFrame represents a logical set of rows with the same named columns, similar to a [Pandas DataFrame](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html) or [Spark DataFrame](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html).
