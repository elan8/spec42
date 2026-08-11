# META
~~~ini
description=Complex end members with outer specializations before feature keyword
type=kerml
semantic_graph=skip
semantic_graph_skip_reason=KerML association and outer-specialization declarations are opaque parser fallback nodes; end relationships are unavailable as structured semantic inputs
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
  (document "end_outer_specializations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1657944386d0528cfec52b234d5b9f762edd8a31e57a20193066c8c1529c1c88") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
