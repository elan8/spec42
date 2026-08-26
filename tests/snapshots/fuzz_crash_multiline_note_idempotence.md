# META
~~~ini
description=Unclosed multiline note preserved as-is (non-idempotent for malformed input)
type=file
notes=Demonstrates design principle: formatter preserves malformed input as-is without modification. Unclosed multiline notes are output verbatim. This results in non-idempotent formatting (AST may differ on reparse) but preserves user information and avoids silent data loss. This aligns with industry practice where formatters do not accept malformed input.
~~~
# SOURCE
~~~sysml
package ers {
	//*>> baseTyclassifier A;,	classifier B;

	specializaaaaaaaaaaaaaaaaaaaaaaaaaaA specializes B;
	specialization swbclassifier B :> A;

	Uubclassifier C s cializes A;
	subclassifier C speciaer D disjoint fr_m C differecializes A, B;
		caassifier D disjoint fr_m C differences A, B;
	cla[sifie Conjugation {
er E specializes C intersects A, B;
	classifier F union^ A unions B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_crash_multiline_note_idempotence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 1) (end 12 0))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 12 1) (end 12 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5ef0e4760c2f7811cb9c38c75d1a1bbe739a0b9754b6b66dee31d68795cf20af") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_crash_multiline_note_idempotence.md") (qualified-name "ers"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
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
)
~~~
