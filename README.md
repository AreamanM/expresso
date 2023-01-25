# expresso

expresso is split into 2 parts:

- [blend](blend): a commandline maths expression evaluator
- [cocoa](cocoa): a tiny library for evaluating maths expressions which `blend` uses.

The parser is implemented using the [Pratt parsing algorithm](https://en.wikipedia.org/wiki/Operator-precedence_parser#Pratt_parsing).

Since expresso relies on rust for it's calculations, it is subject to floating
point precision errors that rust is subject to.
