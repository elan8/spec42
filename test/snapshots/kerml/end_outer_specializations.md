# META
~~~ini
description=Complex end members with outer specializations before feature keyword
type=kerml
~~~
# SOURCE
~~~kerml
assoc HappensDuring specializes HappensLink {
	end feature shorterOccurrence: Occurrence redefines sourceOccurrence crosses longerOccurrence.timeEnclosedOccurrences;
	end happensDuring [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines longerOccurrence;
}

assoc PortionOf specializes Within {
	end portionWithin subsets portionOf feature portionedOccurrence: Occurrence redefines largerOccurrence;
}

assoc WithinBoth specializes Within {
	end withinBoth subsets spaceTimeCoincidentOccurrences feature thatOccurrence redefines largerOccurrence;
}

assoc JustOutsideOf specializes OutsideOf {
	end feature redefines separateSpaceToo: Occurrence crosses separateSpace.justOutsideOfOccurrences;
	end feature redefines separateSpace: Occurrence crosses separateSpaceToo.justOutsideOfOccurrences;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/end_outer_specializations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 0 32) (end 0 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 32) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 53) (end 1 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 2 1) (end 2 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 28) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 6 1) (end 6 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 29) (end 9 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 10 1) (end 10 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 32) (end 13 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 23) (end 14 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 41) (end 14 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 23) (end 15 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 38) (end 15 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:634c1786fddee30481cb57e559058cc765b2b1be390bafa18eb8fae1980af62f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensLink"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring::shorterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "sourceOccurrence"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "JustOutsideOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OutsideOf"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpaceToo"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpace"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "PortionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Within"))))
    (declaration (id (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "WithinBoth"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Within"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring"))) (kind specialization) (ordinal 0))
      (authored-target "HappensLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring::shorterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring::shorterOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "sourceOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "JustOutsideOf"))) (kind specialization) (ordinal 0))
      (authored-target "OutsideOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpaceToo")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "PortionOf"))) (kind specialization) (ordinal 0))
      (authored-target "Within")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "WithinBoth"))) (kind specialization) (ordinal 0))
      (authored-target "Within")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 0 32) (end 0 43)) (probe (position 0 32))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring"))) (kind specialization) (ordinal 0) (authored-target "HappensLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 1 32) (end 1 42)) (probe (position 1 32))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring::shorterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 1 53) (end 1 69)) (probe (position 1 53))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "HappensDuring::shorterOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "sourceOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 13 32) (end 13 41)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "JustOutsideOf"))) (kind specialization) (ordinal 0) (authored-target "OutsideOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 14 41) (end 14 51)) (probe (position 14 41))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 15 38) (end 15 48)) (probe (position 15 38))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 14 23) (end 14 39)) (probe (position 14 23))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "separateSpaceToo")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 15 23) (end 15 36)) (probe (position 15 23))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "separateSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 5 28) (end 5 34)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "PortionOf"))) (kind specialization) (ordinal 0) (authored-target "Within")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/end_outer_specializations.md") (range (start 9 29) (end 9 35)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/end_outer_specializations.md") (qualified-name "WithinBoth"))) (kind specialization) (ordinal 0) (authored-target "Within")
      (outcome (status unresolved)))
  )
)
~~~
