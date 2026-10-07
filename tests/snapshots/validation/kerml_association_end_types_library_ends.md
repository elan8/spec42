# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationEndTypes counts the type an untyped end takes from the Links library end it redefines
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationEndTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.4.2:validateAssociationEndTypes
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package Associations {
    private import Links::*;

    struct Thing;
    struct Other;

    // Conforming: every owned end feature has exactly one type.
    assoc Typed {
        end feature source : Thing;
        end feature target : Thing;
    }

    // Conforming: an end with no authored type has the one type of the library end it
    // redefines (`Links::BinaryLink::source` / `target`).
    assoc Untyped {
        end feature source;
        end feature target;
    }

    assoc TwoTypes {
        end feature source : Thing;

        // Invalid: an owned end feature with two types.
        end feature target : Thing, Other;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_end_types_library_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 23 8) (end 23 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_end_types_library_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 23 8) (end 23 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c63884cf26cc723e245362b52679a40b898ff4a33581a0e64eebfb4dc22c56e4") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (path (named (kind package) (name "Associations")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Links") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (path (named (kind package) (name "Associations")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Links")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target"))) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes")))
      (type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes")))
      (type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")) (provenance authored))
      (type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed")))
      (type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed")))
      (type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Untyped")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 1 19) (end 1 27)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (path (named (kind package) (name "Associations")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Links")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 20 29) (end 20 34)) (probe (position 20 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 23 29) (end 23 34)) (probe (position 23 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 23 36) (end 23 41)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Other")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 8 29) (end 8 34)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types_library_ends.md") (range (start 9 29) (end 9 34)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types_library_ends.md") (qualified-name "Associations::Thing")))))
    )
  )
)
~~~
