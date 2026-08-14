# META
~~~ini
description=Enum diagnostics use resolved semantic kind, not a type name spelling
type=file
~~~
# SOURCE
~~~sysml
package Demo {
    enum def StateCode {
        enum approved;
    }
    part def StatusNamedType;
    part def Base {
        attribute value : StatusNamedType;
    }
    part def Derived :> Base;
    part host : Derived {
        attribute value = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/enum_name_not_semantic.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:52a96ed63257e6aa657a59707b52382ade3d5e7c882c7a02722ca5d3a69a3191") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base::value"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StatusNamedType"))))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base"))))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StateCode"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StateCode::approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StatusNamedType"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Derived"))))
    (declaration (id (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host::value"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "StatusNamedType")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StatusNamedType")))))
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base")))))
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0))
      (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base::value"))) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StatusNamedType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host"))) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host::value"))) (value (kind string) (value "approved")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/enum_name_not_semantic.md") (range (start 6 26) (end 6 41)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0) (authored-target "StatusNamedType")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::StatusNamedType")))))
  )
  (query (document "memory://snapshot/enum_name_not_semantic.md") (range (start 8 24) (end 8 28)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Base")))))
  )
  (query (document "memory://snapshot/enum_name_not_semantic.md") (range (start 9 16) (end 9 23)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0) (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_name_not_semantic.md") (qualified-name "Demo::Derived")))))
  )
)
~~~
