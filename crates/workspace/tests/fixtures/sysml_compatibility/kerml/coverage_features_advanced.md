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
        const end feature constPort : T;
        member feature m : T;
    }

    namespace NS;

    all
    import C::*;
    import C::**;
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'FeatureAdvancedCoverage'
      (not_implemented 'malformed')
      (feature_def 'myFeature' : 'T'[unresolved])
      (package 'MyPackage')
      (feature_def 'fIn'
        (feature_def in 'input' : 'T'[unresolved]))
      (feature_def 'fOut' ~ 'FeatureAdvancedCoverage::fIn'[feature_def])
      (feature_def 'fConj' ~ 'FeatureAdvancedCoverage::fIn'[feature_def])
      (feature_def 'x' :> 'FeatureAdvancedCoverage::myFeature'[feature_def])
      (feature_def 'y' :> 'FeatureAdvancedCoverage::myFeature'[feature_def])
      (feature_def 'z' :>> 'myFeature'[unresolved])
      (feature_def 'w' :>> 'myFeature'[unresolved])
      (feature_def 'z1' : 'T'[unresolved])
      (class_def 'C'
        (feature_def end 'port1' : 'T'[unresolved])
        (feature_def end 'constPort' : 'T'[unresolved])
        (feature_def 'm' : 'T'[unresolved]))
      (namespace 'NS')
      (not_implemented 'malformed')
      (namespace_import -> 'FeatureAdvancedCoverage::C'[class_def])
      (membership_import recursive -> 'FeatureAdvancedCoverage::C'[class_def]))))
~~~
