# META
~~~ini
description=KerML Invalid Multiplicity Ranges
type=file
~~~
# SOURCE
~~~kerml
package InvalidMult {
    classifier Bad [3..1];
    classifier AlsoBad [*..5];
    classifier Valid [1..3];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "invalid_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwClassifier,Ident,OpenSquare,Star,DotDot,DecimalValue,CloseSquare,Semicolon,
KwClassifier,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'InvalidMult'
    (classifier_def 'Bad' multiplicity     (multiplicity_range))
    (classifier_def 'AlsoBad' multiplicity     (multiplicity_range))
    (classifier_def 'Valid' multiplicity     (multiplicity_range))))
~~~
# EXPECTED
~~~
semantic.computed_multiplicity_invalid
semantic.computed_multiplicity_invalid
~~~
# PROBLEMS
~~~
semantic.computed_multiplicity_invalid
semantic.computed_multiplicity_invalid
~~~
# FORMAT
~~~sysml
package InvalidMult {
    classifier Bad [3..1];
    classifier AlsoBad [*..5];
    classifier Valid [1..3];
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fd8e2c79f01ffd4412c56c5ba8416632a9bd909d1b2aaf49856362872d0aec00") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "InvalidMult"))) (kind "package") (name "InvalidMult") (declared-name "InvalidMult") (range (start (line 0) (character 0)) (end (line 0) (character 110))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::AlsoBad"))) (kind "classifier decl") (name "AlsoBad") (declared-name "AlsoBad") (range (start (line 2) (character 4)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Bad"))) (kind "classifier decl") (name "Bad") (declared-name "Bad") (range (start (line 1) (character 4)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Valid"))) (kind "classifier decl") (name "Valid") (declared-name "Valid") (range (start (line 3) (character 4)) (end (line 3) (character 28))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
