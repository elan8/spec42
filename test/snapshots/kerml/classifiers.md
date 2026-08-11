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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifiers.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 4 1) (end 4 319))
      )
    )
  )
)
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
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "2373c1638d17a8133b46ae178f8ffcb0e034a289fa6de195e32d83adcced3016") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Classifiers"))) (kind "package") (name "Classifiers") (declared-name "Classifiers") (range (start (line 0) (character 0)) (end (line 0) (character 374))))
    (element (id (node (document "d0") (qualified-name "Classifiers::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 14))) (parent (node (document "d0") (qualified-name "Classifiers"))))
    (element (id (node (document "d0") (qualified-name "Classifiers::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 2) (character 1)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "Classifiers"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
