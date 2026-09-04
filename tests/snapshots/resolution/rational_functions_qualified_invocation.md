# META
~~~ini
description=A qualified invocation of a Kernel Function Library function resolves against the admitted standard library with no import, including its callee reachability into the library closure (spec42#129); negative controls for a missing package and a missing member stay unresolved
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package RationalFunctionsInvocation {
    attribute conversionFactor = RationalFunctions::rat(1, 100);

    // Negative control: the package exists, the member does not.
    attribute missingMember = RationalFunctions::notAFunction(1, 100);

    // Negative control: neither the package nor the member exists.
    attribute missingPackage = NotAPackage::notAFunction(1, 100);
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/rational_functions_qualified_invocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 30) (end 4 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 31) (end 7 56))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/rational_functions_qualified_invocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 30) (end 4 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 31) (end 7 56))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a51c8e9b31f973a0059345d6e02077c6bf737c32ad0f7b2bf778bd6a53290cc2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::conversionFactor"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "RationalFunctions::rat")))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingMember"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "RationalFunctions::notAFunction")))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingPackage"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "NotAPackage::notAFunction")))))
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "RationalFunctions::rat")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat")))))
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "RationalFunctions::notAFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "NotAPackage::notAFunction")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::conversionFactor"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::conversionFactor"))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingMember"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingMember"))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingPackage"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingPackage"))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat"))) (supplied 2) (required 0) (start 1 33) (end 1 63))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::conversionFactor")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::conversionFactor")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingMember")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingMember")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingPackage")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (qualified-name "RationalFunctionsInvocation::missingPackage")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 100)))))
  (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 100)))))
  (declaration (id (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 100)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/rational_functions_qualified_invocation.md") (range (start 1 33) (end 1 55)) (probe (position 1 33))
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "conversionFactor")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "RationalFunctions::rat")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/rational_functions.md") (qualified-name "RationalFunctions::rat")))))
    )
  )
  (query (document "memory://snapshot/rational_functions_qualified_invocation.md") (range (start 4 30) (end 4 61)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingMember")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "RationalFunctions::notAFunction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rational_functions_qualified_invocation.md") (range (start 7 31) (end 7 56)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/rational_functions_qualified_invocation.md") (path (named (kind package) (name "RationalFunctionsInvocation")) (named (kind attribute) (name "missingPackage")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "NotAPackage::notAFunction")
      (outcome (status unresolved)))
    )
  )
)
~~~
