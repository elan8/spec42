# META
~~~ini
description=SysML Training 29 (Expressions): MassRollup1
type=file
~~~
# SOURCE
~~~sysml
package MassRollup1 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute :>> totalMass = simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass); 
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/29_mass_rollup1.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 26) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 25) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 16) (end 15 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2aabd2b4b6764ffef81178db1f001b5b209afb433fc32e220d809d26c21988d8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "totalMass") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "totalMass")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "simpleMass")) (memberAccessOperand (reference "subcomponents::totalMass")) (invocationCallee (reference "sum")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "totalMass") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "totalMass")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "simpleMass")))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing")) (scopes any))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents")) (scopes any))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing")))
      (type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing")))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents")))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing")))
      (type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing")))
      (type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing")))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "simpleMass" (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 4 26) (end 4 35)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 5 25) (end 5 34)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 12 23) (end 12 34)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 14 16) (end 14 25)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 15 3) (end 15 13)) (probe (position 15 3))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "simpleMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 15 20) (end 15 43)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 15 16) (end 15 19)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "compositeThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 13 22) (end 13 33)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 8 20) (end 8 31)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 9 16) (end 9 25)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    )
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 9 28) (end 9 38)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (path (named (kind package) (name "MassRollup1")) (named (kind part) (name "simpleThing")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "simpleMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass")))))
    )
  )
)
~~~
