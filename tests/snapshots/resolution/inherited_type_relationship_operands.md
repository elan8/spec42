# META
~~~ini
description=A type relationship may name an inherited feature operand
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=Local name resolution includes inherited memberships
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.2.3.5.3:resolveLocalName
type=file
libraries=none
~~~
# SOURCE
~~~kerml
package P {
    classifier Base {
        feature inherited;
    }
    classifier Derived specializes Base {
        feature direct;
        feature intersection intersects direct, inherited;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inherited_type_relationship_operands.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind intersecting)
    (source "P::Derived::intersection")
    (target "P::Base::inherited")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inherited_type_relationship_operands.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8eee46b803eed3cd3429cd5fa8a2782054db388cca72681ea12250771cff9869"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (intersecting (reference "direct")) (intersecting (reference "inherited")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base")))))
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 0))
      (authored-target "direct")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct")))))
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 1))
      (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base")))
      (subtype (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited")))
      (featured-by (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base")))
    )
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived")))
      (supertype (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct")))
      (featured-by (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived")))
    )
    (declaration (id (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection")))
      (featured-by (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inherited_type_relationship_operands.md") (range (start 4 35) (end 4 39)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base")))))
    )
  )
  (query (document "memory://snapshot/inherited_type_relationship_operands.md") (range (start 6 40) (end 6 46)) (probe (position 6 40))
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 0) (authored-target "direct")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::direct")))))
    )
  )
  (query (document "memory://snapshot/inherited_type_relationship_operands.md") (range (start 6 48) (end 6 57)) (probe (position 6 48))
    (reference (id (source (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Derived::intersection"))) (kind intersecting) (ordinal 1) (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_type_relationship_operands.md") (qualified-name "P::Base::inherited")))))
    )
  )
)
~~~
