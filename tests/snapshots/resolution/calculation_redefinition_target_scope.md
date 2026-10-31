# META
~~~ini
description=Calculation usage and anonymous parameter redefinitions resolve inherited features, never siblings (KerML 1.0 8.2.3.5.1; issue 255)
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition
coverage_role=secondary
require_complete_publication=true
libraries=none
~~~
# SOURCE
~~~sysml
package Demo {
    calc def Base { in target; }
    calc def Sum :> Base { in :>> target = 1; }
    calc def SiblingParameter {
        in target;
        in :>> target = 1;
    }
    calc def Calc;
    part def P { calc calcA : Calc; }
    part def Derived :> P { calc calcB : Calc :>> calcA; }
    part def SiblingCalc {
        calc calcA : Calc;
        calc calcB : Calc :>> calcA;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/calculation_redefinition_target_scope.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 15) (end 5 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 30) (end 12 35))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind redefinition) (source (anonymous (owner "Demo::Sum") (kind ReferenceUsage) (ordinal 0))) (target "Demo::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Demo::SiblingParameter") (kind ReferenceUsage) (ordinal 0))) (provenance authored) (outcome unresolved))
  (relationship (kind redefinition) (source "Demo::Derived::calcB") (target "Demo::P::calcA") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Demo::Derived::calcB") (target "Demo::Calc") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source "Demo::SiblingCalc::calcB") (provenance authored) (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/calculation_redefinition_target_scope.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 15) (end 5 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 30) (end 12 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:32fb1f792f13398fe9256f80fe09a2e36ceec4bac55f98c2fac0fb5b90b3cff0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calc")) (redefinition (reference "calcA")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calc")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calc")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Calc")) (redefinition (reference "calcA")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter::target"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind redefinition) (ordinal 0))
      (authored-target "calcA")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind featureTyping) (ordinal 0))
      (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind redefinition) (ordinal 0))
      (authored-target "calcA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base")))))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter::target"))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base")))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base")))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB")) (scopes any))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA")) (scopes any))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA")) (scopes any))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived")))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived")))
      (type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (source direct))
      (effective-type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (source inherited) (from (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (scopes any))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P")))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P")))
      (type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (source direct))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (scopes any))
      (subtype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc")))
      (type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (source direct))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc")))
      (type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (source direct))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter")))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter")))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter::target")))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingParameter")))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum")))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum")))
      (supertype (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum")))
    )
    (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 9 24) (end 9 25)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 9 41) (end 9 45)) (probe (position 9 41))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind featureTyping) (ordinal 0) (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 9 50) (end 9 55)) (probe (position 9 50))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Derived::calcB"))) (kind redefinition) (ordinal 0) (authored-target "calcA")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 8 30) (end 8 34)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::P::calcA"))) (kind featureTyping) (ordinal 0) (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 11 21) (end 11 25)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcA"))) (kind featureTyping) (ordinal 0) (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 12 21) (end 12 25)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind featureTyping) (ordinal 0) (authored-target "Calc")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Calc")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 12 30) (end 12 35)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::SiblingCalc::calcB"))) (kind redefinition) (ordinal 0) (authored-target "calcA")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 5 15) (end 5 21)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "SiblingParameter")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 2 20) (end 2 24)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Sum"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base")))))
    )
  )
  (query (document "memory://snapshot/calculation_redefinition_target_scope.md") (range (start 2 34) (end 2 40)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Sum")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_redefinition_target_scope.md") (qualified-name "Demo::Base::target")))))
    )
  )
)
~~~
