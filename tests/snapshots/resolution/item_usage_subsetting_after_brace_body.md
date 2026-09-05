# META
~~~ini
description=spec42#128 (Apollo 11 Purpose/StakeholderPackage.sysml, 40 occurrences): an item usage with a brace body subsets another item usage via a `:>` clause trailing the body. Both are item-family features, so the specialization is well-formed; a false `incompatible_subset_redefine_kind` regressed here when the trailing `:>` parsed as a new, keyword-less (attribute-family) member instead of concern1's own clause -- fixed in the parser (elan8/sysml-v2-parser#136/#137), not here. Negative control: subsetting an actually incompatible family still reports the kind mismatch.
type=file
~~~
# SOURCE
~~~sysml
package ApolloItemSubsettingRepro {
    item def Concern;
    action def Objective;

    part def Stakeholder {
        item concerns[*] : Concern;
        action objectives[*] : Objective;
    }

    part NASA : Stakeholder {
        // Accepted: concern1 and concerns are both item-family, comparable in the occurrence
        // hierarchy -- the trailing `:>` after concern1's own brace body is its own subsetting
        // clause, not a separate member.
        item concern1 : Concern {
            doc /* Mission success. */
        } :> concerns;

        // Negative control: an item usage subsetting an actually incompatible family (action) is
        // still reported.
        item rejected : Concern {
            doc /* Not an objective. */
        } :> objectives;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/item_usage_subsetting_after_brace_body.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_subset_redefine_kind")
        (source "semantic")
        (range (start 21 13) (end 21 23))
        (related-information
          (related
            (uri "memory://snapshot/item_usage_subsetting_after_brace_body.md")
            (range (start 6 8) (end 6 41))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/item_usage_subsetting_after_brace_body.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_subset_redefine_kind")
        (source "semantic")
        (range (start 21 13) (end 21 23))
        (related-information
          (related
            (uri "memory://snapshot/item_usage_subsetting_after_brace_body.md")
            (range (start 6 8) (end 6 41))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9011da7ac6dc66ab073c279c04e16f79078c0c10f00eb98628e952158e93fbae"))
  (declarations
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Stakeholder")))))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind item) (membership (kind feature) (visibility default)) (documentation (doc (text " Mission success. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Concern")) (subsetting (reference "concerns")))))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind item) (membership (kind feature) (visibility default)) (documentation (doc (text " Not an objective. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Concern")) (subsetting (reference "objectives")))))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Concern")))))
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Objective")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Stakeholder")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind subsetting) (ordinal 0))
      (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind featureTyping) (ordinal 0))
      (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind subsetting) (ordinal 0))
      (authored-target "objectives")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (kind featureTyping) (ordinal 0))
      (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (kind featureTyping) (ordinal 0))
      (authored-target "Objective")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1")) (scopes any))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected")) (scopes any))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA")))
      (type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")) (source direct))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1")))
      (featured-by (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA")))
      (type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (provenance authored))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (source direct))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (source inherited) (from (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (scopes any))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected")))
      (featured-by (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA")))
      (type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (provenance authored))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (source direct))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")) (source inherited) (from (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (scopes any))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")) (scopes any))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns")))
      (featured-by (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")))
      (type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (provenance authored))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (source direct))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")) (scopes any))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives")))
      (featured-by (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")))
      (type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")) (provenance authored))
      (effective-type (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")) (source direct))
      (supertype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")) (scopes any))
      (subtype (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 9 16) (end 9 27)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA"))) (kind featureTyping) (ordinal 0) (authored-target "Stakeholder")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 13 24) (end 13 31)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind featureTyping) (ordinal 0) (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 15 13) (end 15 21)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::concern1"))) (kind subsetting) (ordinal 0) (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 19 24) (end 19 31)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind featureTyping) (ordinal 0) (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 21 13) (end 21 23)) (probe (position 21 13))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::NASA::rejected"))) (kind subsetting) (ordinal 0) (authored-target "objectives")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 5 27) (end 5 34)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::concerns"))) (kind featureTyping) (ordinal 0) (authored-target "Concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Concern")))))
    )
  )
  (query (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (range (start 6 31) (end 6 40)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Stakeholder::objectives"))) (kind featureTyping) (ordinal 0) (authored-target "Objective")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_usage_subsetting_after_brace_body.md") (qualified-name "ApolloItemSubsettingRepro::Objective")))))
    )
  )
)
~~~
