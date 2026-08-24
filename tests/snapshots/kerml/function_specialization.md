# META
~~~ini
description=Function and predicate definitions with specialization clauses
type=file
~~~
# SOURCE
~~~kerml
package FuncSpec {
    function F specializes Base::G { }
    function H :> Base::I, Base::J { }
    abstract function K :> Base::L;
    predicate P specializes Base::Q { }
    predicate R :> Base::S;
    function FI specializes Base::G intersects Base::H { }
    predicate PI specializes Base::Q intersects Base::R, Base::S { }
    inv I { not x }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/function_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 27) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 18) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 27) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 27) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 28) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 19) (end 5 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 28) (end 6 35))
      )
      (diagnostic
        (severity error)
        (code "single_type_relationship_operand")
        (source "semantic")
        (range (start 6 47) (end 6 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 47) (end 6 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 29) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 48) (end 7 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 57) (end 7 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 16) (end 8 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dfcdfda5c349979d632045feed31aec73306106fb0e6f1511d985d326a0970d9") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::F"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::G")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::FI"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::G")) (intersecting (reference "Base::H")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::H"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::I")) (specialization (reference "Base::J")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::I"))) (kind kerml-invariant) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::K"))) (kind kerml-function) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::L")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::P"))) (kind kerml-predicate) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Q")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind kerml-predicate) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::Q")) (intersecting (reference "Base::R")) (intersecting (reference "Base::S")))))
    (declaration (id (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::R"))) (kind kerml-predicate) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base::S")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::F"))) (kind specialization) (ordinal 0))
      (authored-target "Base::G")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::FI"))) (kind specialization) (ordinal 0))
      (authored-target "Base::G")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::FI"))) (kind intersecting) (ordinal 0))
      (authored-target "Base::H")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::H"))) (kind specialization) (ordinal 0))
      (authored-target "Base::I")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::H"))) (kind specialization) (ordinal 1))
      (authored-target "Base::J")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::I"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::K"))) (kind specialization) (ordinal 0))
      (authored-target "Base::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::P"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Q")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind specialization) (ordinal 0))
      (authored-target "Base::Q")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind intersecting) (ordinal 0))
      (authored-target "Base::R")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind intersecting) (ordinal 1))
      (authored-target "Base::S")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::R"))) (kind specialization) (ordinal 0))
      (authored-target "Base::S")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::I"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/function_specialization.md") (range (start 1 27) (end 1 34)) (probe (position 1 27))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::F"))) (kind specialization) (ordinal 0) (authored-target "Base::G")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 6 28) (end 6 35)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::FI"))) (kind specialization) (ordinal 0) (authored-target "Base::G")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 6 47) (end 6 54)) (probe (position 6 47))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::FI"))) (kind intersecting) (ordinal 0) (authored-target "Base::H")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 2 18) (end 2 25)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::H"))) (kind specialization) (ordinal 0) (authored-target "Base::I")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 2 27) (end 2 34)) (probe (position 2 27))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::H"))) (kind specialization) (ordinal 1) (authored-target "Base::J")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 8 16) (end 8 17)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::I"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 3 27) (end 3 34)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::K"))) (kind specialization) (ordinal 0) (authored-target "Base::L")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 4 28) (end 4 35)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::P"))) (kind specialization) (ordinal 0) (authored-target "Base::Q")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 7 29) (end 7 36)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind specialization) (ordinal 0) (authored-target "Base::Q")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 7 48) (end 7 55)) (probe (position 7 48))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind intersecting) (ordinal 0) (authored-target "Base::R")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 7 57) (end 7 64)) (probe (position 7 57))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::PI"))) (kind intersecting) (ordinal 1) (authored-target "Base::S")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/function_specialization.md") (range (start 5 19) (end 5 26)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/function_specialization.md") (qualified-name "FuncSpec::R"))) (kind specialization) (ordinal 0) (authored-target "Base::S")
      (outcome (status unresolved)))
    )
  )
)
~~~
