# META
~~~ini
description=Derivation endpoint resolution coverage
type=file
observed_gap=Both derivation endpoint reference-subsetting facts resolve and are published; the snapshot pins endpoint coverage without assuming an additional derived relationship.
~~~
# SOURCE
~~~sysml
package DerivationCoverage {
    requirement def ParentRequirement;
    requirement def ChildRequirement;
    #derivation connection {
        end #original ::> ParentRequirement;
        end #derive ::> ChildRequirement;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_endpoints.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e4b5bc0c3f49e79a1df2ac0ec41f8a55a44d29ff58ec570ecd966163e7debb44") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DerivationCoverage"))) (kind "package") (name "DerivationCoverage") (declared-name "DerivationCoverage"))
    (element (id (node (document "d0") (qualified-name "DerivationCoverage::ChildRequirement"))) (kind "requirement def") (name "ChildRequirement") (declared-name "ChildRequirement") (parent (node (document "d0") (qualified-name "DerivationCoverage"))))
    (element (id (node (document "d0") (qualified-name "DerivationCoverage::ParentRequirement"))) (kind "requirement def") (name "ParentRequirement") (declared-name "ParentRequirement") (parent (node (document "d0") (qualified-name "DerivationCoverage"))))
    (element (id (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection"))) (kind "derivation connection") (name "_derivationConnection") (parent (node (document "d0") (qualified-name "DerivationCoverage"))))
    (element (id (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#derive"))) (kind "interface end") (name "#derive") (declared-name "#derive") (parent (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "ChildRequirement")))))
    (element (id (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#original"))) (kind "interface end") (name "#original") (declared-name "#original") (parent (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "ParentRequirement")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#derive"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "ChildRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationCoverage::ChildRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#original"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "ParentRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "DerivationCoverage::ParentRequirement")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#derive"))) (target (node (document "d0") (qualified-name "DerivationCoverage::ChildRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#derive"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#original"))) (target (node (document "d0") (qualified-name "DerivationCoverage::ParentRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#original"))) (kind referenceSubsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 24) (end 5 40)) (probe (position 5 24))
      (reference
        (source (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#derive"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "ChildRequirement")
        (range (start 5 24) (end 5 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DerivationCoverage::ChildRequirement") (range (start 2 4) (end 2 37)))
        )
      )
    )
    (query (range (start 4 26) (end 4 43)) (probe (position 4 26))
      (reference
        (source (document "d0") (qualified-name "DerivationCoverage::_derivationConnection::#original"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "ParentRequirement")
        (range (start 4 26) (end 4 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DerivationCoverage::ParentRequirement") (range (start 1 4) (end 1 38)))
        )
      )
    )
  )
)
~~~
