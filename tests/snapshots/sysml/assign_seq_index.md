# META
~~~ini
description=Assign node with sequence indexing operator #()
type=file
~~~
# SOURCE
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign 'var' := data#(idx);
        assign a.b := items#(0);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/assign_seq_index.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 15) (end 2 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 20) (end 2 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 25) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 15) (end 3 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 24) (end 3 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 30) (end 3 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 15) (end 4 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 22) (end 4 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f48cf36000dd05e36f81d233a04f7fa6c90e07194251c45fbfb722d2c47d2ad6") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "seq")) (expressionOperand (reference "i")) (assignTarget (reference "x")))))
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "data")) (expressionOperand (reference "idx")) (assignTarget (reference "var")))))
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")) (memberAccessOperand (reference "a::b")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "seq")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "data")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "i")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 1))
      (authored-target "idx")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "a::b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "var")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (target (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A")))
    )
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A")))
    )
    (declaration (id (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/assign_seq_index.md") (qualified-name "AssignTest::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 2 20) (end 2 23)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "seq")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 3 24) (end 3 28)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "data")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 4 22) (end 4 27)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 2 25) (end 2 26)) (probe (position 2 25))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "i")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 3 30) (end 3 33)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 1) (authored-target "idx")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 4 15) (end 4 18)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "a::b")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 2 15) (end 2 16)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/assign_seq_index.md") (range (start 3 15) (end 3 20)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/assign_seq_index.md") (path (named (kind package) (name "AssignTest")) (named (kind action-def) (name "A")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "var")
      (outcome (status unresolved)))
    )
  )
)
~~~
