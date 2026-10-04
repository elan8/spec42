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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:20a785860760a1a3b8d101214a75fbeaaa3c57c83f1667dddfd5cafd66c9a1f3") (admitted (standard-library 94)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::a"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
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
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Left")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Right")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link::b")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/generated_conditional_binary_connection_specializations.md") (qualified-name "BinaryConnectionSpecializations::Link"))) (kind connection))
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
