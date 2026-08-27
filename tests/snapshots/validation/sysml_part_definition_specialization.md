# META
~~~ini
description=SysML checkPartDefinitionSpecialization publishes the canonical Parts::Part specialization for every PartDefinition that lacks an effective authored specialization
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Model {
    import Parts::*;

    // The semantic rule supplies an implied specialization to Parts::Part.
    part def Component;

    // An authored equivalent specialization is retained and does not gain an implied duplicate.
    part def Equivalent specializes Part;

    // A more-specific authored specialization reaches Parts::Part transitively.
    part def Vehicle specializes Part;
    part def Specific specializes Vehicle;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind specialization)
    (source "Model::Component")
    (target "Parts::Part")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind specialization)
    (source "Model::Equivalent")
    (target "Parts::Part")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind specialization)
    (source "Model::Equivalent")
    (provenance implied)
    (outcome absent))
  (relationship
    (kind specialization)
    (source "Model::Specific")
    (target "Model::Vehicle")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind specialization)
    (source "Model::Specific")
    (provenance implied)
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_part_definition_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:960ebb842210c249a1a3448d6d456adcc30eff7ad8ed9501f6c041e63124858b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Parts") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts")))))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")))))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific"))) (target (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_part_definition_specialization.md") (range (start 1 11) (end 1 19)) (probe (position 1 11))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts")))))
    )
  )
  (query (document "memory://snapshot/sysml_part_definition_specialization.md") (range (start 7 36) (end 7 40)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Equivalent"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")))))
    )
  )
  (query (document "memory://snapshot/sysml_part_definition_specialization.md") (range (start 11 34) (end 11 41)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Specific"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/sysml_part_definition_specialization.md") (range (start 10 33) (end 10 37)) (probe (position 10 33))
    (reference (id (source (node (document "memory://snapshot/sysml_part_definition_specialization.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")))))
    )
  )
)
~~~
