# META
~~~ini
description=Bare redefines/subsets as shorthand features in KerML bodies
type=kerml
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
    portion feature all portions : Occurrence [1..*] {
        portion redefines portionOfLife = (that as Occurrence).portionOfLife;
    }
    feature all spaceTimeEnclosedPoints : Occurrence [1..*] {
        redefines innerSpaceDimension = 0;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (class_def 'A'
      (feature_def 'innerSpaceDimension' : 'Natural'[unresolved]
        (multiplicity_range [1]))
      (feature_def 'portions' : 'Occurrence'[unresolved]
        (multiplicity_range [1..*])
        (feature_def :>> 'portionOfLife'[unresolved]
          (feature_value (=))))
      (feature_def 'spaceTimeEnclosedPoints' : 'Occurrence'[unresolved]
        (multiplicity_range [1..*])
        (feature_def :>> 'A::innerSpaceDimension'[feature_def]
          (feature_value (=)))))))
~~~
