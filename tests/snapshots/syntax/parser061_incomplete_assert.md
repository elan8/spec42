# META
~~~ini
description=Parser v0.61.0 diagnoses an incomplete bare assertion while preserving the following valid inline assertion
type=file
~~~
# SOURCE
~~~sysml
part def T {
    assert not ;
    assert constraint { true }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser061_incomplete_assert.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser061_incomplete_assert.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:936cc744213304088220c1cf0270b100831ab6904e5da658bb9c505fae84137b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (qualified-name "T"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/parser061_incomplete_assert.md") (qualified-name "T"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser061_incomplete_assert.md") (qualified-name "T")))
    )
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/parser061_incomplete_assert.md") (path (named (kind part-def) (name "T")) (anonymous (kind assert-constraint) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (outcome resolved) (result-typing (status unsupported)) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
