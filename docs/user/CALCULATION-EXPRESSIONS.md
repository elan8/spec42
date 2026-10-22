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

Spec42 resolves the callback parameter type and references inside the body. Local members
are lowered through the calculation-body dispatcher. Callback names shadow enclosing names
and do not become members of the enclosing calculation. The same scope handling applies to
`->collect`, `->forAll` and the braced dot shorthand forms. Bare calculation returns and
calculation-body aliases also use their canonical semantic owners.

Higher-order callback execution is not implemented. Expressions containing these callbacks
publish an explicit unsupported evaluation outcome, rather than classifying the call as if
its callback were absent. This includes the filtered `sum` above. Supported constant folding
of ordinary scalar expressions is separate from executing library functions.

## Coverage boundaries

The pinned parser has 25 calculation-body member variants, including parser recovery and a
delegation to action-body members. The calculation dispatcher explicitly rejects metadata
keyword usage; delegated members and nested expressions can have their own unsupported
cases. This is a dispatcher count, not a measure of full language conformance.

The calculation expression walker still explicitly rejects these whole AST forms:

- Infinity literals (`*`) in formula positions.
- Metadata classification (`@Metaclass`).
- Selector-based collect (`base.**selector`, distinct from braced `->collect`).
- Extent expressions (`all Type`).

Unary plus and complement are also outside the admitted operator set. Some admitted shapes
have restricted operands or only partial semantic representation. Indexed member access
(such as `edges#(i).vertices`) retains its owned index arguments and resolves members through
the index result, including nested indices, casts and conditional index operands. Resolving
the member definition does not evaluate an indexed occurrence. General invocation,
constructor and higher-order execution must not be inferred from successful reference
resolution. The [conformance matrix](../reference/CONFORMANCE-MATRIX.md) continues to mark
expressions and units as partial.

The body scope follows [KerML 1.0](https://www.omg.org/spec/KerML/1.0/PDF),
8.2.5.8.2–8.2.5.8.3 (`BodyArgumentMember`, `BodyExpression`, `ExpressionBody`). Regression
fixtures are `resolution/collection_expression_bodies.md` and
`resolution/collection_expression_body_errors.md` in the standalone snapshot corpus;
they cover accepted forms, shadowing, local members, unresolved names and scope isolation.

Remaining semantic-construction gaps are tracked in [#337](https://github.com/elan8/spec42/issues/337);
full collection callback evaluation is tracked in [#338](https://github.com/elan8/spec42/issues/338).
