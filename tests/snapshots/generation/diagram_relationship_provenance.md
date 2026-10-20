# META
~~~ini
description=General view distinguishes authored and implied relationship provenance
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=diagram_relationship_provenance.md
viewQualifiedName=ProvenanceExample::selected
~~~
# SOURCE
~~~sysml
package ProvenanceExample {
    private import StandardViewDefinitions::*;
    part def Base { attribute mass : Real; }
    part def Child :> Base { attribute mass = 1200; }
    view selected : GeneralView { expose Child; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_relationship_provenance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 37) (end 2 41))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 3 29) (end 3 51))
        (related-information
          (related
            (uri "memory://snapshot/diagram_relationship_provenance.md")
            (range (start 2 20) (end 2 42))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:01bbce4d805c388be2382bcd7d87105d428e4471829d0ec736c2c2ce8068957b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Child")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1200)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")) (scopes any feature))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind part-def) (name "Child")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1200))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 2 37) (end 2 41)) (probe (position 2 37))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 3 22) (end 3 26)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 20) (end 4 31)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 41) (end 4 46)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:c136d09375a74020f98a76967103b1242dcaf22c5881c25e16f0df252fcadbc4",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_relationship_provenance.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        22,
        3,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        39,
        3,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        46,
        3,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        17
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "ProvenanceExample::Base"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "ProvenanceExample::Child"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "ProvenanceExample::Child::mass"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "ProvenanceExample::Child::mass::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "ProvenanceExample::selected"
    },
    {
      "kind": "source-anchor",
      "source": 3,
      "ownerQualifiedName": "ProvenanceExample::Child::mass",
      "metaclass": "LiteralInteger",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 3,
      "ownerQualifiedName": "ProvenanceExample::Child::mass::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "specializes",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 2
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "general-view",
    "name": "selected",
    "source": 4
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "general-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 1,
        "metaclass": "PartDefinition",
        "notationRole": "definition",
        "name": "Child",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "attributes",
            "provenance": "direct",
            "members": [
              1
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "AttributeUsage",
        "notationRole": "usage",
        "name": "mass",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "LiteralInteger",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 6,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 2,
        "source": 3,
        "compartments": [],
        "direction": "out",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 8,
        "source": 0,
        "kind": "specializes",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 1
      }
    ],
    "edges": [
      {
        "reference": 7,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 9,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 10,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      }
    ],
    "metadata": {
      "roots": [
        0
      ]
    },
    "scene": {
      "kind": "general"
    }
  }
}

~~~
