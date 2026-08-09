# META
~~~ini
description=KerML Simple Tests: Classifiers
type=file
~~~
# SOURCE
~~~kerml
package Classifiers {
	classifier A;
	classifier B;
	
	specialization Super subclassifier A specializes B;
	specialization subclassifier B :> A;
	
	subclassifier C specializes A;
	subclassifier C specializes B;
	
	classifier C specializes A, B;
	
	classifier D disjoint from C differences A, B;
	classifier E specializes C intersects A, B;
	classifier F unions A unions B;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwSpecialization,Ident,KwSubclassifier,Ident,KwSpecializes,Ident,Semicolon,
KwSpecialization,KwSubclassifier,Ident,ColonGt,Ident,Semicolon,
KwSubclassifier,Ident,KwSpecializes,Ident,Semicolon,
KwSubclassifier,Ident,KwSpecializes,Ident,Semicolon,
KwClassifier,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwClassifier,Ident,KwDisjoint,KwFrom,Ident,KwDifferences,Ident,Comma,Ident,Semicolon,
KwClassifier,Ident,KwSpecializes,Ident,KwIntersects,Ident,Comma,Ident,Semicolon,
KwClassifier,Ident,KwUnions,Ident,KwUnions,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Classifiers'
    (classifier_def 'A')
    (classifier_def 'B')
    (subclassification specialization 'Super' specific 'A' general 'B')
    (malformed)
    (subclassification specific 'B' general 'A')
    (subclassification specific 'C' general 'A')
    (subclassification specific 'C' general 'B')
    (classifier_def 'C' :> 'A', 'B')
    (classifier_def 'D' disjoint from 'C' differences 'A', 'B')
    (classifier_def 'E' :> 'C' intersects 'A', 'B')
    (classifier_def 'F' unions 'A' unions 'B')))
~~~
# FORMAT
~~~sysml
package Classifiers {
    classifier A;
    classifier B;

    specialization Super subclassifier A specializes B;
    specialization
    subclassifier B :> A;

    subclassifier C specializes A;
    subclassifier C specializes B;

    classifier C specializes A, B;

    classifier D disjoint from C differences A, B;
    classifier E specializes C intersects A, B;
    classifier F unions A unions B;
}
~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# SMG
~~~
(model
  (namespace
    (package 'Classifiers'
      (classifier_def 'A')
      (classifier_def 'B')
      (subclassification 'Super')
      (not_implemented 'malformed')
      (subclassification)
      (subclassification)
      (subclassification)
      (classifier_def 'C' :> 'Classifiers::A'[classifier_def] :> 'Classifiers::B'[classifier_def])
      (classifier_def 'D'
        (disjoining_decl)
        (differencing)
        (differencing))
      (classifier_def 'E' :> 'Classifiers::C'[classifier_def]
        (intersecting)
        (intersecting))
      (classifier_def 'F'
        (unioning)
        (unioning)))))
~~~
