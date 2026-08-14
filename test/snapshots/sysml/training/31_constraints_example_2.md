# META
~~~ini
description=SysML Training 31 (Constraints): Constraints Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraints Example-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		attribute partMasses : MassValue[0..*];
		attribute massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		constraint massConstraint : MassConstraint {
			redefines partMasses = (chassisMass, engine.mass, transmission.mass);
			redefines massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/31_constraints_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 2) (end 9 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 12) (end 9 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 9 23) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 2) (end 10 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 12) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 10 22) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 2) (end 12 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 6) (end 12 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 21) (end 12 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 3) (end 17 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 13) (end 17 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 17 24) (end 18 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 3) (end 18 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 13) (end 18 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 18 23) (end 19 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 26) (end 21 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 20) (end 24 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 20) (end 28 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:1401b94c195cf681c773b33c51257cac88eec771fd607c5e2b716d4656834f0e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "attribute")) (expressionOperand (reference "partMasses")) (expressionOperand (reference "attribute")) (expressionOperand (reference "massLimit")) (expressionOperand (reference "partMasses")) (expressionOperand (reference "massLimit")) (invocationCallee (reference "sum"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassConstraint")) (expressionOperand (reference "redefines")) (expressionOperand (reference "partMasses")) (expressionOperand (reference "redefines")) (expressionOperand (reference "massLimit"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "partMasses")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 2))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 3))
      (authored-target "massLimit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 4))
      (authored-target "partMasses")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 5))
      (authored-target "massLimit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine")))))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint")))))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "redefines")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "partMasses")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 2))
      (authored-target "redefines")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 3))
      (authored-target "massLimit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine")))))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine"))) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (path (named (kind package) (name "Constraints Example-2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 9 2) (end 9 11)) (probe (position 9 2))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 0) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 9 12) (end 9 22)) (probe (position 9 12))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 1) (authored-target "partMasses")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 10 2) (end 10 11)) (probe (position 10 2))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 2) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 10 12) (end 10 21)) (probe (position 10 12))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 3) (authored-target "massLimit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 12 6) (end 12 16)) (probe (position 12 6))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 4) (authored-target "partMasses")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 12 21) (end 12 30)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind expressionOperand) (ordinal 5) (authored-target "massLimit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 12 2) (end 12 5)) (probe (position 12 2))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint"))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 21 26) (end 21 35)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 23 16) (end 23 22)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine")))))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 24 20) (end 24 29)) (probe (position 24 20))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 16 30) (end 16 44)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind featureTyping) (ordinal 0) (authored-target "MassConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::MassConstraint")))))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 17 3) (end 17 12)) (probe (position 17 3))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 0) (authored-target "redefines")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 17 13) (end 17 23)) (probe (position 17 13))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 1) (authored-target "partMasses")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 18 3) (end 18 12)) (probe (position 18 3))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 2) (authored-target "redefines")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 18 13) (end 18 22)) (probe (position 18 13))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::massConstraint"))) (kind expressionOperand) (ordinal 3) (authored-target "massLimit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 27 22) (end 27 28)) (probe (position 27 22))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Engine")))))
  )
  (query (document "memory://snapshot/31_constraints_example_2.md") (range (start 28 20) (end 28 29)) (probe (position 28 20))
    (reference (id (source (node (document "memory://snapshot/31_constraints_example_2.md") (qualified-name "Constraints Example-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
)
~~~
