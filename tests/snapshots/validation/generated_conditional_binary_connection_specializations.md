# META
~~~ini
description=Generated binary connection specialization uses the exact owned-end cardinality fact
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.13.3:checkConnectionDefinitionBinarySpecialization
blocked_by=library-gap-conditional-binary-specialization-anchors
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package BinaryConnectionSpecializations {
    part def Left;
    part def Right;
    connection def Link {
        end a : Left;
        end b : Right;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "BinaryConnectionSpecializations::Link") (target "Connections::BinaryConnections") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_binary_connection_specializations.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 3 4) (end 6 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:f4b0252d991b7580e89fa8b8182206a36e32820b9596fdac2a8a91720f82472c") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Left")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Right")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right"))) (kind part-def) (membership (kind owning) (visibility default)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (range (start 4 16) (end 4 20)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (kind featureTyping) (ordinal 0) (authored-target "Left")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (range (start 5 16) (end 5 21)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (kind featureTyping) (ordinal 0) (authored-target "Right")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))))
    )
  )
)
~~~
