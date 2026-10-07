# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureResultRedefinition requires a function or expression result to redefine inherited results
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureResultRedefinition
type=file
~~~
# SOURCE
~~~kerml
package Results {
    classifier Value;
    function Base { return base : Value; }
    // An owned result redefines the result of the Function its owner specializes.
    function Refined specializes Base { return refined : Value; }
    // A Function that owns no result inherits the result of its general.
    function Inheriting specializes Base;
    // A further result redefines the inherited result.
    function Further specializes Inheriting { return further : Value; }
    // An authored redefinition of the general result keeps its authored provenance.
    function Explicit specializes Base { return explicit : Value :>> Base::base; }
    // An Expression typed by a Function redefines that Function's result.
    expr computed : Base { return computedResult : Value; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureResultRedefinition") (outcome satisfied))
  (relationship (kind redefinition) (source "Results::Refined::refined") (target "Results::Base::base") (provenance implied) (outcome resolved))
  (relationship (kind redefinition) (source "Results::Further::further") (target "Results::Base::base") (provenance implied) (outcome resolved))
  (relationship (kind redefinition) (source "Results::Explicit::explicit") (target "Results::Base::base") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source "Results::computed::computedResult") (target "Results::Base::base") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_result_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bad63a4b6be34af8b324bd587131e3ed11f3df65e5b5b70c09f0fdac161d3a90"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Value")) (redefinition (reference "Base::base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Inheriting")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (kind kerml-function) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (kind kerml-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Value")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind redefinition) (ordinal 0))
      (authored-target "Base::base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (kind specialization) (ordinal 0))
      (authored-target "Inheriting")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit")))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further")))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting")))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined")))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed")))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 2 34) (end 2 39)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 10 34) (end 10 38)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 10 59) (end 10 64)) (probe (position 10 59))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 10 69) (end 10 79)) (probe (position 10 69))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Explicit::explicit"))) (kind redefinition) (ordinal 0) (authored-target "Base::base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 8 33) (end 8 43)) (probe (position 8 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further"))) (kind specialization) (ordinal 0) (authored-target "Inheriting")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 8 63) (end 8 68)) (probe (position 8 63))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Further::further"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 6 36) (end 6 40)) (probe (position 6 36))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Inheriting"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 4 33) (end 4 37)) (probe (position 4 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 4 57) (end 4 62)) (probe (position 4 57))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Refined::refined"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 12 20) (end 12 24)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition.md") (range (start 12 51) (end 12 56)) (probe (position 12 51))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::computed::computedResult"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition.md") (qualified-name "Results::Value")))))
    )
  )
)
~~~
