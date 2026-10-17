# META
~~~ini
description=Anonymous redefinitions retain inherited targets as sibling effective names settle for one, two, and three features (KerML 1.0 8.2.3.5.1; issue 255)
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.4.5:validateNamespaceDistinguishibility
coverage_role=secondary
require_complete_publication=true
libraries=none
~~~
# SOURCE
~~~sysml
package Names {
    part def Base { attribute target; }
    part def One :> Base { attribute :>> target; }
    part def Two :> Base {
        attribute :>> target;
        attribute :>> target;
    }
    part def Three :> Base {
        attribute :>> target;
        attribute :>> target;
        attribute :>> target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/anonymous_redefinition_effective_names.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 5 8) (end 5 29))
        (related-information
          (related
            (uri "memory://snapshot/anonymous_redefinition_effective_names.md")
            (range (start 4 8) (end 4 29))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 9 8) (end 9 29))
        (related-information
          (related
            (uri "memory://snapshot/anonymous_redefinition_effective_names.md")
            (range (start 8 8) (end 8 29))
          )
        )
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind redefinition) (source (anonymous (owner "Names::One") (kind AttributeUsage) (ordinal 0))) (target "Names::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Names::Two") (kind AttributeUsage) (ordinal 0))) (target "Names::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Names::Two") (kind AttributeUsage) (ordinal 1))) (target "Names::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Names::Three") (kind AttributeUsage) (ordinal 0))) (target "Names::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Names::Three") (kind AttributeUsage) (ordinal 1))) (target "Names::Base::target") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source (anonymous (owner "Names::Three") (kind AttributeUsage) (ordinal 2))) (target "Names::Base::target") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/anonymous_redefinition_effective_names.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 5 8) (end 5 29))
        (related-information
          (related
            (uri "memory://snapshot/anonymous_redefinition_effective_names.md")
            (range (start 4 8) (end 4 29))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 9 8) (end 9 29))
        (related-information
          (related
            (uri "memory://snapshot/anonymous_redefinition_effective_names.md")
            (range (start 8 8) (end 8 29))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b882ad9d3669a413712653bfc6cc58f1601cb34c47aca0f4cb5aa0cf31ecd068"))
  (declarations
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "target") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target"))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two")))
      (supertype (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 2 20) (end 2 24)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::One"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 2 41) (end 2 47)) (probe (position 2 41))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "One")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 7 22) (end 7 26)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Three"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 8 22) (end 8 28)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 9 22) (end 9 28)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Three")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 3 20) (end 3 24)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Two"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 4 22) (end 4 28)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
  (query (document "memory://snapshot/anonymous_redefinition_effective_names.md") (range (start 5 22) (end 5 28)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (path (named (kind package) (name "Names")) (named (kind part-def) (name "Two")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/anonymous_redefinition_effective_names.md") (qualified-name "Names::Base::target")))))
    )
  )
)
~~~
