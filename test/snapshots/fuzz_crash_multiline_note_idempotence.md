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
  (document "fuzz_crash_multiline_note_idempotence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 3 1) (end 3 332))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 12 1) (end 12 2))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
MalformedMultilineNote,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ers'
    (malformed)))
~~~
# EXPECTED
~~~
tokenize.UnclosedMultilineNote
parse.expected_close_curly
~~~
# PROBLEMS
~~~
tokenize.UnclosedMultilineNote
parse.expected_close_curly
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5606128b9441d3d77b25f39fa390a7d83e22b38d8b04100d6cbe16a1738026c4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ers"))) (kind "package") (name "ers") (declared-name "ers") (range (start (line 0) (character 0)) (end (line 0) (character 390))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
