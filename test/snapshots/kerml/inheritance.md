# META
~~~ini
description=KerML Simple Tests: Inheritance
type=file
~~~
# SOURCE
~~~kerml
package Inheritance {
	class A {
		feature f;
	}
	
	class B specializes A {
		
	}
		
	feature y: A {
		alias x for B::f;
		feature g redefines f;
	}
	
	alias z for y::g;
	
	feature w subsets y;
	
	alias us for w::g;
	
	feature yy: y;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inheritance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 1 1) (end 3 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 1) (end 3 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 1) (end 7 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 7 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 13) (end 14 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 16 1) (end 16 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 1) (end 16 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 14) (end 18 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 20 1) (end 20 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 1) (end 20 15))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:bcf46385f7f1e9a2b8e6d606e86f99b9d21f2aa81f60309b3bfba5794642d5e0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "w::g"))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "y::g"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind aliasBinding) (ordinal 0))
      (authored-target "w::g")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind aliasBinding) (ordinal 0))
      (authored-target "y::g")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inheritance.md") (range (start 18 14) (end 18 18)) (probe (position 18 14))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind aliasBinding) (ordinal 0) (authored-target "w::g")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 14 13) (end 14 17)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind aliasBinding) (ordinal 0) (authored-target "y::g")
      (outcome (status unresolved)))
  )
)
~~~
