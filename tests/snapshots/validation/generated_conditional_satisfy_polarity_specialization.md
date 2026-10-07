# META
~~~ini
description=Generated satisfy specialization chooses the exact negated requirement-check anchor
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package SatisfyPolaritySpecialization {
    requirement def Safety;
    part def Vehicle;
    requirement safety : Safety;
    part vehicle : Vehicle;
    not satisfy safety by vehicle;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source (anonymous (owner "SatisfyPolaritySpecialization") (kind SatisfyRequirementUsage) (ordinal 0))) (target "Requirements::notSatisfiedRequirementChecks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:0e39e0a9f2c577ade7398203478011f7814b8d7502544a7c143effbb04d2b1d8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "safety")) (satisfyTarget (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (kind featureTyping) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::negatedConstraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::falseEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety")))
      (type (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (supertype (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle")))
      (type (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")) (scopes any))
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
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 5 16) (end 5 22)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 5 26) (end 5 33)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 3 25) (end 3 31)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::safety"))) (kind featureTyping) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 4 19) (end 4 26)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))))
    )
  )
)
~~~
