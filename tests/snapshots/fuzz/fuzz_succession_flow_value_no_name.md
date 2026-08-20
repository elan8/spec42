# META
~~~ini
description=Fuzz: succession flow with value expression but no name preserves value in formatting
type=file
~~~
# SOURCE
~~~sysml
package P {
    class Container {
        step a1 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow=sf from a1.y to a2.x;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_succession_flow_value_no_name.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 18) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 27) (end 3 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 8) (end 4 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 13) (end 4 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 18) (end 4 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 21) (end 4 25))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 5 8) (end 6 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:d48ca10a6f48f735e0a0bb7d958924cd9a3d725bd7fff02b464f78b88fd73630") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "flow")) (expressionOperand (reference "to")) (memberAccessOperand (reference "a1::y")) (memberAccessOperand (reference "a2::x")))))
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "a2")))))
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action1")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind expressionOperand) (ordinal 0))
      (authored-target "flow")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind expressionOperand) (ordinal 1))
      (authored-target "to")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "a1::y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "a2::x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1")))))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "a2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action1")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1")))
      (featured-by (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 4 8) (end 4 12)) (probe (position 4 8))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind expressionOperand) (ordinal 0) (authored-target "flow")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 4 18) (end 4 20)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind expressionOperand) (ordinal 1) (authored-target "to")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 4 13) (end 4 17)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind memberAccessOperand) (ordinal 0) (authored-target "a1::y")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 4 21) (end 4 25)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container"))) (kind memberAccessOperand) (ordinal 1) (authored-target "a2::x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 3 19) (end 3 21)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1")))))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 3 27) (end 3 29)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Container")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "a2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (range (start 2 18) (end 2 25)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/fuzz_succession_flow_value_no_name.md") (qualified-name "P::Container::a1"))) (kind featureTyping) (ordinal 0) (authored-target "Action1")
      (outcome (status unresolved)))
    )
  )
)
~~~
