# FloraUML

> Generate UML diagrams based on an intuitive DSL.

Inspired by [PlantUML](https://github.com/plantuml/plantuml).
This is an early alpha version under heavy development.

## Usage

Flora currently always reads from stdin and writes to stdout.

### Input

Flora accepts a list of _class declarations_, each terminated by a semicolon, as its input.
Excess whitespace is ignored.

A _class declaration_ has the following structure: `class <name>;`

#### Example

```
class A;  class B;
class C;

class D;
```

### Output

The only supported output format at the moment is [DOT](https://www.graphviz.org/doc/info/lang.html).
DOT can be converted to SVG and other formats using [GraphViz](https://www.graphviz.org/).

Flora generates a graph with a node containing the class name for each class.
Currently, every class is connected to every other class via a solid line (i.e. the graph is fully connected).
