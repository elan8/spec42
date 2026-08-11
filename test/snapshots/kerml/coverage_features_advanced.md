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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwType,Ident,Semicolon,
KwFeature,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwPackage,OpenAngle,Ident,CloseAngle,Ident,Semicolon,
KwFeature,Ident,OpenCurly,
KwIn,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Tilde,Ident,Semicolon,
KwFeature,Ident,KwConjugates,Ident,Semicolon,
KwFeature,Ident,KwSubsets,Ident,Semicolon,
KwFeature,Ident,KwReferences,Ident,Semicolon,
KwFeature,Ident,ColonGtGt,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
Hash,Ident,KwFeature,Ident,Colon,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,KwConst,KwFeature,Ident,Colon,Ident,Semicolon,
KwMember,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwNamespace,Ident,Semicolon,
KwAll,KwImport,Ident,ColonColon,Star,Semicolon,
KwImport,Ident,ColonColon,StarStar,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FeatureAdvancedCoverage'
    (malformed)
    (feature_def 'myFeature' : 'T')
    (package_def 'MyPackage')
    (feature_def 'fIn'
      (feature_def in 'input' : 'T'))
    (feature_def 'fOut' ~ fIn)
    (feature_def 'fConj' conjugates fIn)
    (feature_def 'x' :> 'myFeature')
    (feature_def 'y' references 'myFeature')
    (feature_def 'z' :>> 'myFeature')
    (feature_def 'w' :>> 'myFeature')
    (feature_def #'Safety' 'z1' : 'T')
    (class_def 'C'
      (feature_def end 'port1' : 'T')
      (feature_def const end 'constPort' : 'T')
      (feature_def member 'm' : 'T'))
    (namespace_def 'NS')
    (malformed)
    (import_decl 'C::*')
    (import_decl 'C::**')))
~~~
# EXPECTED
~~~
parse.expected_specialization_or_body
parse.unexpected_token
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'myFeature'
semantic.unresolved_name 'myFeature'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
~~~
# PROBLEMS
~~~
parse.expected_specialization_or_body
parse.unexpected_token
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'myFeature'
semantic.unresolved_name 'myFeature'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ae02cf1e5293606f98a8b70268bc44312106b6e80ab94846e5af31b68ffc0455") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (kind "package") (name "FeatureAdvancedCoverage") (declared-name "FeatureAdvancedCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 566))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 19) (character 4)) (end (line 19) (character 121))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))) (kind "import") (name "C") (declared-name "C") (range (start (line 28) (character 4)) (end (line 28) (character 17))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (authored (membership (kind Import) (import (reference "C") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 28) (character 11)) (end (line 28) (character 12))))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::MyPackage"))) (kind "package") (name "MyPackage") (declared-name "MyPackage") (range (start (line 4) (character 4)) (end (line 4) (character 28))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::_Safety"))) (kind "metadata keyword") (name "Safety") (declared-name "Safety") (range (start (line 17) (character 4)) (end (line 17) (character 12))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fConj"))) (kind "feature decl") (name "fConj") (declared-name "fConj") (range (start (line 10) (character 4)) (end (line 10) (character 33))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fIn"))) (kind "feature decl") (name "fIn") (declared-name "fIn") (range (start (line 6) (character 4)) (end (line 6) (character 53))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fOut"))) (kind "feature decl") (name "fOut") (declared-name "fOut") (range (start (line 9) (character 4)) (end (line 9) (character 23))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::w"))) (kind "feature decl") (name "w") (declared-name "w") (range (start (line 15) (character 4)) (end (line 15) (character 34))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 12) (character 4)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 13) (character 4)) (end (line 13) (character 35))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z"))) (kind "feature decl") (name "z") (declared-name "z") (range (start (line 14) (character 4)) (end (line 14) (character 28))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind "feature decl") (name "z1") (declared-name "z1") (range (start (line 17) (character 12)) (end (line 17) (character 27))) (parent (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))) (kind membershipImport) (ordinal 0)) (authored-target "C") (range (start (line 28) (character 11)) (end (line 28) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
