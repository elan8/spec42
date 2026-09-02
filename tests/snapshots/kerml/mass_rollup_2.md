# META
~~~ini
description=KerML Mass Roll-up: MassRollup_2
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/mass_rollup_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 17) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 22) (end 6 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 10) (end 7 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b83d6990feb616a01e6085d8f17bc56ce84bd86e75e3f0081eba8ad6690c9b75"))
  (declarations
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massedThings")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "mass")) (memberAccessOperand (reference "subcomponents::totalMass")) (invocationCallee (reference "sum")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (kind redefinition) (ordinal 0))
      (authored-target "massedThings")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))
      (subtype (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass")))
      (featured-by (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents")))
      (featured-by (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))
      (effective-type (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")) (source inherited) (from (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))))
      (supertype (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")) (scopes any))
      (supertype (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass")))
      (featured-by (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings")))
      (type (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")) (scopes any))
      (subtype (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 5 17) (end 5 35)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 9 34) (end 9 46)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::subcomponents"))) (kind redefinition) (ordinal 0) (authored-target "massedThings")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 6 22) (end 6 40)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 7 3) (end 7 7)) (probe (position 7 3))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::mass")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 7 14) (end 7 37)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing::totalMass")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 7 10) (end 7 13)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (named (kind class-def) (name "MassedThing")) (named (kind kerml-feature) (name "totalMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 12 23) (end 12 34)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))))
    )
  )
)
~~~
