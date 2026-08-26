# META
~~~ini
description=Generated binary connection specialization uses the exact owned-end cardinality fact
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.13.3:checkConnectionDefinitionBinarySpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package BinaryConnectionSpecializations {
    occurrence def Left;
    occurrence def Right;
    connection def Link {
        end occurrence a : Left;
        end occurrence b : Right;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "BinaryConnectionSpecializations::Link") (target "Connections::BinaryConnection") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_binary_connection_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:4bf6795c0b937802d1c81aa4f4594c1c409dd2bf9e3ce61264096f69f3b5e05a") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Left")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Right")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Left")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Right")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (positional-ends (authored 0) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (range (start 4 27) (end 4 31)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind featureTyping) (ordinal 0) (authored-target "Left")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (range (start 5 27) (end 5 32)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind featureTyping) (ordinal 0) (authored-target "Right")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))))
    )
  )
)
~~~
