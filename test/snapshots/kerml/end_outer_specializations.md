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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 0 0) (end 3 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 0) (end 3 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 5 0) (end 7 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 0) (end 7 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 0) (end 11 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 0) (end 11 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 0) (end 16 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 0) (end 16 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:634c1786fddee30481cb57e559058cc765b2b1be390bafa18eb8fae1980af62f") (contract-version "parser-owned-resolution-v1"))
  (declarations
  )
  (references
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
)
~~~
