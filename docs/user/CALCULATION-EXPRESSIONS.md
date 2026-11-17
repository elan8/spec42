# Calculation expressions

Spec42 distinguishes parsing, semantic resolution and evaluation. A parsed expression can
resolve its references without being executable by the model-level evaluator. A clean
validation result therefore does not guarantee a computed value.

## Collection callbacks

A collection body has its own scope for parameters, local members and the result expression:

```sysml
package Calculations {
    private import ScalarValues::*;
    private import NumericalFunctions::*;

    calc def PositiveTotal {
        in contributions : Real[0..*];
        return result : Real = sum(contributions
            ->select { in value : Real; value > 0.0 });
    }
}
```

Spec42 resolves the callback parameter type and the references inside the body. Callback names
shadow enclosing names and do not become members of the enclosing calculation. The same scope
rules apply to `->collect`, `->forAll` and the braced dot shorthand forms.

Higher-order callback execution is not implemented. Expressions containing these callbacks
report an explicit unsupported evaluation outcome rather than being evaluated as if the callback
were absent. This includes the filtered `sum` above. Constant folding of ordinary scalar
expressions is supported; executing library functions is separate.

## What is not supported

These forms are rejected when a calculation is analysed:

- Infinity literals (`*`) in formula positions.
- Metadata classification (`@Metaclass`).
- Selector-based collect (`base.**selector`, distinct from braced `->collect`).
- Extent expressions (`all Type`).
- Unary plus and complement operators.

Some supported forms have restricted operands or only partial semantic representation. Indexed
member access (such as `edges#(i).vertices`) resolves members through the index result, including
nested indices, casts and conditional index operands, but Spec42 does not evaluate an indexed
occurrence. Do not infer that an invocation, constructor or higher-order call executes from the
fact that its references resolve. The [conformance matrix](../reference/CONFORMANCE-MATRIX.md)
marks expressions and units as partial.

The body scope follows [KerML 1.0](https://www.omg.org/spec/KerML/1.0/PDF), clauses
8.2.5.8.2–8.2.5.8.3. Remaining semantic gaps are tracked in
[#337](https://github.com/elan8/spec42/issues/337); full collection callback evaluation is tracked in
[#338](https://github.com/elan8/spec42/issues/338).
