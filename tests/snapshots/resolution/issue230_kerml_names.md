# META
~~~ini
description=Issue 230: KerML selects its own reserved vocabulary: SysML-only port and item are legal names while from is malformed unless quoted.
type=file
~~~
# SOURCE
~~~kerml
package KermlNames {
    feature port;
    feature item;
    feature from;
    feature 'from';
    feature retained;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_kerml_names.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 3 12) (end 3 16))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_kerml_names.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 3 12) (end 3 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:48b63a69bca7313038307d1c154312858462824f8d2316b2a17016dd5f91d0c6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (qualified-name "KermlNames"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (path (named (kind package) (name "KermlNames")) (malformed-name (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (qualified-name "KermlNames::from"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (qualified-name "KermlNames::item"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (qualified-name "KermlNames::port"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_kerml_names.md") (qualified-name "KermlNames::retained"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
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
