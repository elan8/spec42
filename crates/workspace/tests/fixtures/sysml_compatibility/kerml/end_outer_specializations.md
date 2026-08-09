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
# EXPECTED
~~~
semantic.unresolved_name 'HappensLink'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'sourceOccurrence'
semantic.unresolved_name 'longerOccurrence::timeEnclosedOccurrences'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'Within'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'largerOccurrence'
semantic.unresolved_name 'Within'
semantic.unresolved_name 'spaceTimeCoincidentOccurrences'
semantic.unresolved_name 'largerOccurrence'
semantic.unresolved_name 'OutsideOf'
semantic.unresolved_name 'separateSpaceToo'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'separateSpace::justOutsideOfOccurrences'
semantic.unresolved_name 'separateSpace'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'separateSpaceToo::justOutsideOfOccurrences'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'HappensLink'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'sourceOccurrence'
semantic.unresolved_name 'longerOccurrence::timeEnclosedOccurrences'
semantic.unresolved_name 'timeCoincidentOccurrences'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'longerOccurrence'
semantic.unresolved_name 'Within'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'largerOccurrence'
semantic.unresolved_name 'Within'
semantic.unresolved_name 'spaceTimeCoincidentOccurrences'
semantic.unresolved_name 'largerOccurrence'
semantic.unresolved_name 'OutsideOf'
semantic.unresolved_name 'separateSpaceToo'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'separateSpace::justOutsideOfOccurrences'
semantic.unresolved_name 'separateSpace'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'separateSpaceToo::justOutsideOfOccurrences'
~~~
# TOKENS
~~~zig
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (association_def 'HappensDuring' :> 'HappensLink'
    (feature_def end 'shorterOccurrence' : 'Occurrence' :>> 'sourceOccurrence' crosses 'longerOccurrence.timeEnclosedOccurrences')
    (feature_def end 'thatOccurrence' multiplicity :> 'timeCoincidentOccurrences' : 'Occurrence' :>> 'longerOccurrence'))
  (association_def 'PortionOf' :> 'Within'
    (feature_def end 'portionedOccurrence' :> 'portionOf' : 'Occurrence' :>> 'largerOccurrence'))
  (association_def 'WithinBoth' :> 'Within'
    (feature_def end 'thatOccurrence' :> 'spaceTimeCoincidentOccurrences' :>> 'largerOccurrence'))
  (association_def 'JustOutsideOf' :> 'OutsideOf'
    (feature_def end :>> 'separateSpaceToo' : 'Occurrence' crosses 'separateSpace.justOutsideOfOccurrences')
    (feature_def end :>> 'separateSpace' : 'Occurrence' crosses 'separateSpaceToo.justOutsideOfOccurrences')))
~~~
# FORMAT
~~~sysml
assoc HappensDuring specializes HappensLink {
    end feature shorterOccurrence : Occurrence redefines sourceOccurrence crosses longerOccurrence.timeEnclosedOccurrences;
    end happensDuring [1..*] subsets timeCoincidentOccurrences feature thatOccurrence : Occurrence redefines longerOccurrence;
}

assoc PortionOf specializes Within {
    end portionWithin subsets portionOf feature portionedOccurrence : Occurrence redefines largerOccurrence;
}

assoc WithinBoth specializes Within {
    end withinBoth subsets spaceTimeCoincidentOccurrences feature thatOccurrence redefines largerOccurrence;
}

assoc JustOutsideOf specializes OutsideOf {
    end feature redefines separateSpaceToo : Occurrence crosses separateSpace.justOutsideOfOccurrences;
    end feature redefines separateSpace : Occurrence crosses separateSpaceToo.justOutsideOfOccurrences;
}
~~~
# SMG
~~~
(semantic-graph
  (status (skip (code "SMG-EMPTY-STRICT") (reason "strictly parsed non-empty source produced no typed semantic graph facts")))
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
