# FloraUML design

## Principles
To be implemented as soon as possible (priority over additional features) and adhered to going forwards.

* scales to large diagrams
* composable model
  * allows mixing UML diagram types
* blazing fast ™️
  * concurrent parsing
  * cacheable model
  * second unchanged run is noop and extremely fast
* one model, n views that generate n diagrams
* focus on UX
   * extensive documentation
   * helpful error messages (rustc-style)

### Non-goals
* support all UML diagram types
* support everything specified by the UML spec for a diagram type
* support old versions of GraphViz or DOT

## Scope for v1
* CLI (rough draft, shows features that should be supported, not concrete syntax)
  * `-i file.extension`
  * `-i directory/`
  * `-i -`
  * `-o file.extension`
  * `-o -`
  * `Object`
  * `-r RecursiveObject`
  * `-x ExcludedObject`
* Class diagrams
  * Class name
  * Class stereotypes
  * Attributes & Operations
  * Visibility
  * Inheritance, Association, Aggregation, Composition
  * Direction (incl. bi-directional)
  * Cardinality
  * Roles

## Generating output
Possible options
from most flexible to least flexible,
from most complex to least complex:
1. layout ourselves
2. layout via GraphViz, generate SVG ourselves (PlantUML)
3. generate DOT ([prototype](https://github.com/jeysal/rust-uml-to-dot-concept/))
  * might be sufficient for simple class diagrams, sooner or later switch to option 2 will become necessary

## Future
* [Asciidoctor Diagram](https://asciidoctor.org/docs/asciidoctor-diagram/) integration?
* Language Service
* Editor syntax themes (nice to have)
* styling (CSS-like?)
