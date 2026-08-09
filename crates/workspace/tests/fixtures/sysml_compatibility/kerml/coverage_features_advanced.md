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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (name "FeatureAdvancedCoverage") (declared-name "FeatureAdvancedCoverage")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C"))) (name "C") (declared-name "C"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::C#import"))) (name "C") (declared-name "C"))
        (element (kind "package") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::MyPackage"))) (name "MyPackage") (declared-name "MyPackage"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::_Safety"))) (name "Safety") (declared-name "Safety"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fConj"))) (name "fConj") (declared-name "fConj"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fIn"))) (name "fIn") (declared-name "fIn"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::fOut"))) (name "fOut") (declared-name "fOut"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::w"))) (name "w") (declared-name "w"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::x"))) (name "x") (declared-name "x"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::y"))) (name "y") (declared-name "y"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z"))) (name "z") (declared-name "z"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureAdvancedCoverage::z1"))) (name "z1") (declared-name "z1"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "FeatureAdvancedCoverage::_Safety"))) (to (node (document "d0") (qualified-name "FeatureAdvancedCoverage"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "FeatureAdvancedCoverage::_Safety"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/coverage_features_advanced.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 48))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 17 4) (end 17 12))
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
