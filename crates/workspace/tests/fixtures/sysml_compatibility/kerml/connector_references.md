# META
~~~ini
description=Connector definitions with references in ends
type=kerml
~~~
# SOURCE
~~~kerml
class A {
	feature self : A;
	feature this : A;
	connector :HappensDuring
		from [1] self references self
		to [1] this references this;
	connector :InsideOf
		from [0..*] smallerOccurrence references elements
		to [1] largerOccurrence references union;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'InsideOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'union'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'InsideOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'union'
~~~
# TOKENS
~~~zig
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'A'
    (feature_def 'self' : 'A')
    (feature_def 'this' : 'A')
    (connector_def : 'HappensDuring'
      (connector_end)
      (connector_end))
    (connector_def : 'InsideOf'
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
class A {
    feature self : A;
    feature this : A;
    connector : HappensDuring from [1] self references self to [1] this references this;
    connector : InsideOf from [0..*] smallerOccurrence references elements to [1] largerOccurrence references union;
}
~~~
# SMG
~~~
(model
  (namespace
    (class_def 'A'
      (feature_def 'self' : 'A'[class_def])
      (feature_def 'this' : 'A'[class_def])
      (connector_def : 'HappensDuring'[unresolved]
        (connector_end 'self' :> 'A::self'[feature_def])
        (connector_end 'this' :> 'A::this'[feature_def]))
      (connector_def : 'InsideOf'[unresolved]
        (connector_end 'smallerOccurrence' :> 'elements'[unresolved])
        (connector_end 'largerOccurrence' :> 'union'[unresolved])))))
~~~
