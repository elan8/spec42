# META
~~~ini
description=Qualified redefinition of a nested feature starts from the enclosing feature's inherited membership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.2.3.5.3:resolveLocalName
libraries=none
~~~
# SOURCE
~~~sysml
package Demo {
    item def Disc {
        item edges;
    }
    item def General {
        item base : Disc {
            ref item edges :>> Disc::edges;
        }
    }
    item def Derived :> General {
        item base :>> General::base : Disc {
            ref item selected :>> base::edges;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind redefinition)
    (source "Demo::Derived::base::selected")
    (target "Demo::General::base::edges")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d94d73ce340cd6e1046fc0cd7f8f5fcdbb2412c0045c575f973368fd57ede2d8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "General")))))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Disc")) (redefinition (reference "General::base")))))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "base::edges")))))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Disc")))))
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Disc::edges")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General")))))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind redefinition) (ordinal 0))
      (authored-target "General::base")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")))))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (kind redefinition) (ordinal 0))
      (authored-target "base::edges")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges")))))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (kind redefinition) (ordinal 0))
      (authored-target "Disc::edges")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived")))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base")))
      (featured-by (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived")))
      (type (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (source direct))
      (effective-type (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (source inherited) (from (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (scopes any))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected")))
      (featured-by (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base")))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges")) (scopes any feature))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base")) (scopes any))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges")))
      (featured-by (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General")))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")))
      (featured-by (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General")))
      (type (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (provenance authored))
      (effective-type (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (source direct))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")) (scopes any))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges")))
      (featured-by (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")))
      (supertype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges")) (scopes any feature))
      (subtype (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 9 24) (end 9 31)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0) (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General")))))
    )
  )
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 10 38) (end 10 42)) (probe (position 10 38))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind featureTyping) (ordinal 0) (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))))
    )
  )
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 10 22) (end 10 35)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base"))) (kind redefinition) (ordinal 0) (authored-target "General::base")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base")))))
    )
  )
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 11 34) (end 11 45)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Derived::base::selected"))) (kind redefinition) (ordinal 0) (authored-target "base::edges")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges")))))
    )
  )
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 5 20) (end 5 24)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base"))) (kind featureTyping) (ordinal 0) (authored-target "Disc")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc")))))
    )
  )
  (query (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (range (start 6 31) (end 6 42)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::General::base::edges"))) (kind redefinition) (ordinal 0) (authored-target "Disc::edges")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_redefinition_enclosing_feature_scope.md") (qualified-name "Demo::Disc::edges")))))
    )
  )
)
~~~
