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
  (document "memory://snapshot/coverage_features_advanced.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 3 4) (end 3 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 4) (end 3 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 4) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 4) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 4) (end 10 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 4) (end 10 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 22) (end 12 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 25) (end 13 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 18) (end 14 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 24) (end 15 33))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 21 8) (end 22 8))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 25 4) (end 28 4))
      )
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 28 11) (end 28 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 28 11) (end 28 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b316894a6bb12af4449e04c2ccc74a6daadd36ede6183f995e60a59ff9a7730b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (path (named (kind package) (name "FeatureAdvancedCoverage")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (membershipImport (reference "C") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::MyPackage"))) (kind package) (membership (kind owning) (visibility default)) (facts (short-name "pkg")))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T") (direction in)))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::w"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "myFeature")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "myFeature")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "myFeature")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "myFeature")))))
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (path (named (kind package) (name "FeatureAdvancedCoverage")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "C")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::w"))) (kind redefinition) (ordinal 0))
      (authored-target "myFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::x"))) (kind subsetting) (ordinal 0))
      (authored-target "myFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::y"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "myFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z"))) (kind redefinition) (ordinal 0))
      (authored-target "myFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m")))
      (featured-by (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C")))
      (type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1")))
      (featured-by (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C")))
      (type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))
      (subtype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input")))
      (featured-by (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn")))
      (type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1")))
      (type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 28 11) (end 28 16)) (probe (position 28 11))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (path (named (kind package) (name "FeatureAdvancedCoverage")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "C")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 22 27) (end 22 28)) (probe (position 22 27))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::m"))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 20 28) (end 20 29)) (probe (position 20 28))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::C::port1"))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 7 27) (end 7 28)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::fIn::input"))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 15 24) (end 15 33)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::w"))) (kind redefinition) (ordinal 0) (authored-target "myFeature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 12 22) (end 12 31)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::x"))) (kind subsetting) (ordinal 0) (authored-target "myFeature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 13 25) (end 13 34)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::y"))) (kind referenceSubsetting) (ordinal 0) (authored-target "myFeature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 14 18) (end 14 27)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z"))) (kind redefinition) (ordinal 0) (authored-target "myFeature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_features_advanced.md") (range (start 17 25) (end 17 26)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::z1"))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_features_advanced.md") (qualified-name "FeatureAdvancedCoverage::T")))))
    )
  )
)
~~~
