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
# FORMAT
~~~sysml
package InvalidMult {
    classifier Bad[3..1];
    classifier AlsoBad[*..5];
    classifier Valid[1..3];
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'InvalidMult'
      (classifier_def 'Bad'
        (multiplicity_range [3..1]))
      (classifier_def 'AlsoBad'
        (multiplicity_range [65534..5]))
      (classifier_def 'Valid'
        (multiplicity_range [1..3])))))
~~~
