# Architecture
1. Geometry Model
	- What objects are connected
	- Consists of points and hyperedges of facts connecting them
2. Inference Engine
	- What can be proven by the current facts (which include numeric values)
	- Uses facts and numerical data to produce facts
3. Constraint Engine
	- Given the numerical data and facts, what numerical data can be added
4. Proof Graph
	- Keeps a record of what interaction caused what
	- Is a graph data structure