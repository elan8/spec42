# META
~~~ini
description=Generated KerML binary specialization uses each exact two-end predicate
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.4.2:checkAssociationBinarySpecialization
rule_id=kerml-1.0:8.3.4.4.3:checkAssociationStructureBinarySpecialization
blocked_by=lowering-gap-kerml-binary-end-collections
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package BinaryKerMLSpecializations {
    classifier Thing;
    assoc Link {
        end feature source : Thing;
        end feature target : Thing;
    }
    assoc struct LinkObject {
        end feature source : Thing;
        end feature target : Thing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "BinaryKerMLSpecializations::Link") (target "Links::BinaryLink") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "BinaryKerMLSpecializations::LinkObject") (target "Objects::BinaryLinkObject") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:71342d86510561531d0a597cd03690a3c505d59d90fac659e477ce519eddaf70") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link")))
      (type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject")))
      (type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject")))
      (type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (range (start 7 29) (end 7 34)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (range (start 8 29) (end 8 34)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::LinkObject::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_kerml_specializations.md") (qualified-name "BinaryKerMLSpecializations::Thing")))))
    )
  )
)
~~~
