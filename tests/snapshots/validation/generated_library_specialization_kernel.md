# META
~~~ini
description=Generated library-specialization checks publish implied canonical anchors for reachable KerML metaclasses
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:checkTypeSpecialization
rule_id=kerml-1.0:8.3.3.1.9:checkMultiplicitySpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureSpecialization
rule_id=kerml-1.0:8.3.4.1.2:checkDataTypeSpecialization
rule_id=kerml-1.0:8.3.4.12.2:checkMetaclassSpecialization
rule_id=kerml-1.0:8.3.4.2.2:checkClassSpecialization
rule_id=kerml-1.0:8.3.4.3.2:checkStructureSpecialization
rule_id=kerml-1.0:8.3.4.4.2:checkAssociationSpecialization
rule_id=kerml-1.0:8.3.4.4.3:checkAssociationStructureSpecialization
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorSpecialization
rule_id=kerml-1.0:8.3.4.6.2:checkBehaviorSpecialization
rule_id=kerml-1.0:8.3.4.6.3:checkStepSpecialization
rule_id=kerml-1.0:8.3.4.7.2:checkBooleanExpressionSpecialization
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionSpecialization
rule_id=kerml-1.0:8.3.4.7.4:checkFunctionSpecialization
rule_id=kerml-1.0:8.3.4.7.6:checkPredicateSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package GeneratedKernel {
    type Type;
    multiplicity Multiplicity [0..1];
    feature Feature;
    datatype DataType;
    metaclass Metaclass;
    class Class;
    struct Structure;
    assoc Association;
    assoc struct AssociationStructure;
    behavior Behavior;
    function Function;
    predicate Predicate;
    expr Expression { 1 }
    bool BooleanExpression { true }
    classifier Holder {
        feature a;
        feature b;
        connector Connector { end feature source :>> a; end feature target :>> b; }
        step Step;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedKernel::Type") (target "Base::Anything") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Multiplicity") (target "Base::naturals") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Feature") (target "Base::things") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::DataType") (target "Base::DataValue") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Metaclass") (target "Metaobjects::Metaobject") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Class") (target "Occurrences::Occurrence") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Structure") (target "Objects::Object") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Association") (target "Links::Link") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::AssociationStructure") (target "Objects::LinkObject") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Holder::Connector") (target "Links::links") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Behavior") (target "Performances::Performance") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Holder::Step") (target "Performances::performances") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::BooleanExpression") (target "Performances::booleanEvaluations") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Expression") (target "Performances::evaluations") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Function") (target "Performances::Evaluation") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedKernel::Predicate") (target "Performances::BooleanEvaluation") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_kernel.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:a75c6b4ca128a63f4085c3b0f623d7cf7106a86a32678bbc2bc8de4e25cbee1d") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Association"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::AssociationStructure"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Behavior"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::BooleanExpression"))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Class"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::DataType"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Expression"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Feature"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Function"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Step"))) (kind kerml-step) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Metaclass"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Multiplicity"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Predicate"))) (kind kerml-predicate) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Structure"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Type"))) (kind kerml-type) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Association"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::AssociationStructure"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Behavior"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::BooleanExpression"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Class"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::DataType"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Expression"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Feature"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Function"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Step"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Step"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b"))) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Metaclass"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Multiplicity"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Predicate"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Structure"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Type"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::BooleanExpression"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Expression"))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Association")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::AssociationStructure")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Behavior")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::BooleanExpression")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Class")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::DataType")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Expression")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Feature")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Function")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector")))
      (supertype (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector")))
      (supertype (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Step")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b")))
      (featured-by (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Metaclass")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Multiplicity")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Predicate")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Structure")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Type")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_library_specialization_kernel.md") (range (start 18 53) (end 18 54)) (probe (position 18 53))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::source"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_kernel.md") (range (start 18 79) (end 18 80)) (probe (position 18 79))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::Connector::target"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kernel.md") (qualified-name "GeneratedKernel::Holder::b")))))
    )
  )
)
~~~
