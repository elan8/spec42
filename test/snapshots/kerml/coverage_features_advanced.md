# META
~~~ini
description=Coverage: Feature declarations with short names, conjugation, end const, member keyword, prefix metadata
type=file
~~~
# SOURCE
~~~kerml
package FeatureAdvancedCoverage {
    type T;

    feature <f> myFeature : T;
    package <pkg> MyPackage;

    feature fIn {
        in feature input : T;
    }
    feature fOut ~ fIn;
    feature fConj conjugates fIn;

    feature x subsets myFeature;
    feature y references myFeature;
    feature z :>> myFeature;
    feature w redefines myFeature;

    #Safety feature z1 : T;

    class C {
        end feature port1 : T;
        end const feature constPort : T;
        member feature m : T;
    }

    namespace NS;

    all import C::*;
    import C::**;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_features_advanced.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 48))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 25 4) (end 25 44))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package FeatureAdvancedCoverage {
    type T;

    feature <f> myFeature : T;
    package <pkg> MyPackage;

    feature fIn {
        in feature input : T;
    }
    feature fOut ~ fIn;
    feature fConj conjugates fIn;

    feature x subsets myFeature;
    feature y references myFeature;
    feature z :>> myFeature;
    feature w redefines myFeature;

    #Safety feature z1 : T;

    class C {
        end feature port1 : T;
        end const feature constPort : T;
        member feature m : T;
    }

    namespace NS;

    all import C::*;
    import C::**;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a1b13c4d4734ee8cf1e59c0ada92967b11e43f55714fe6566514cb1091c4ac2c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (kind "package") (name "FeatureAdvancedCoverage") (declared-name "FeatureAdvancedCoverage"))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))) (kind "import") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (authored (membership (kind Import) (import (reference "C") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::MyPackage"))) (kind "package") (name "MyPackage") (declared-name "MyPackage") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::_Safety"))) (kind "metadata keyword") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fConj"))) (kind "feature decl") (name "fConj") (declared-name "fConj") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fIn"))) (kind "feature decl") (name "fIn") (declared-name "fIn") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fOut"))) (kind "feature decl") (name "fOut") (declared-name "fOut") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::w"))) (kind "feature decl") (name "w") (declared-name "w") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z"))) (kind "feature decl") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind "feature decl") (name "z1") (declared-name "z1") (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))) (kind membershipImport) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C")))) (import (origin import) (shape membership) (recursive true) (conformance valid)))
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
  (document "d0"
    (query (range (start 28 11) (end 28 12)) (probe (position 28 11))
      (reference
        (source (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))
        (kind membershipImport) (ordinal 0) (authored-target "C")
        (range (start 28 11) (end 28 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "FeatureAdvancedCoverage::C") (range (start 19 4) (end 19 121)))
        )
      )
    )
  )
)
~~~
