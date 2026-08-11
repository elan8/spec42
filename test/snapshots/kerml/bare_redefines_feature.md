# META
~~~ini
description=Bare redefines/subsets as shorthand features in KerML bodies
type=kerml
semantic_graph=skip
semantic_graph_skip_reason=KerML class bodies with shorthand redefinitions are opaque parser fallback nodes; feature values and redefinition targets are unavailable as structured semantic inputs
~~~
# SOURCE
~~~kerml
class A {
	feature innerSpaceDimension : Natural [1];
	portion feature all portions: Occurrence[1..*] {
		portion redefines portionOfLife = (that as Occurrence).portionOfLife;
	}
	feature all spaceTimeEnclosedPoints : Occurrence[1..*] {
		redefines innerSpaceDimension = 0;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'portionOfLife'
semantic.unresolved_name 'Occurrence'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'portionOfLife'
semantic.unresolved_name 'Occurrence'
~~~
# TOKENS
~~~zig
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPortion,KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwPortion,KwRedefines,Ident,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'A'
    (feature_def 'innerSpaceDimension' : 'Natural' multiplicity)
    (feature_def portion all 'portions' : 'Occurrence' multiplicity
      (feature_def portion :>> 'portionOfLife' value))
    (feature_def all 'spaceTimeEnclosedPoints' : 'Occurrence' multiplicity
      (feature_def :>> 'innerSpaceDimension' value))))
~~~
# FORMAT
~~~sysml
class A {
	feature innerSpaceDimension : Natural [1];
	portion feature all portions: Occurrence[1..*] {
		portion redefines portionOfLife = (that as Occurrence).portionOfLife;
	}
	feature all spaceTimeEnclosedPoints : Occurrence[1..*] {
		redefines innerSpaceDimension = 0;
	}
}
~~~
# SMG
~~~
(semantic-graph
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/bare_redefines_feature.md"
    (diagnostics
    )
  )
)
~~~
