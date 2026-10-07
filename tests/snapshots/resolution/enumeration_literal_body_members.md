# META
~~~ini
description=An enumeration literal's body is walked like a part usage body: the literal owns the Documentation and the redefining members authored in it
type=file
~~~
# SOURCE
~~~sysml
package Demo {
	attribute def Level {
		attribute code : String;
	}
	enum def Kind specializes Level {
		secret {
			doc /* The secret level. */
			:>> code = "secr";
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/enumeration_literal_body_members.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 19) (end 2 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:28d3b3fc18ed330cbcfbd7c2d232d8fe7a4a8be135135c3ffcb84207ca783ba9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (kind enum-def) (membership (kind owning) (visibility default)) (facts (implied-modifiers abstract variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Level")))))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "The secret level. "))))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "code") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "code")))))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (kind kerml-literal-string) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (kind specialization) (ordinal 0))
      (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "code")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret"))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code"))) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (state literal) (value (kind string) (value "secr")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind")))
      (supertype (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret")))
      (featured-by (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret")))
      (supertype (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind::secret")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level")))
      (subtype (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code")))
      (featured-by (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level")))
      (subtype (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-literal-string) (ordinal 0))))) (outcome resolved) (literal (value (kind string) (value "secr"))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/enumeration_literal_body_members.md") (range (start 4 27) (end 4 32)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Kind"))) (kind specialization) (ordinal 0) (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level")))))
    )
  )
  (query (document "memory://snapshot/enumeration_literal_body_members.md") (range (start 7 7) (end 7 11)) (probe (position 7 7))
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (path (named (kind package) (name "Demo")) (named (kind enum-def) (name "Kind")) (named (kind enum-literal) (name "secret")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "code")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code")))))
    )
  )
  (query (document "memory://snapshot/enumeration_literal_body_members.md") (range (start 2 19) (end 2 25)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/enumeration_literal_body_members.md") (qualified-name "Demo::Level::code"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
