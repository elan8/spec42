# META
~~~ini
description=KerML KerML Spec Annex A: A-3-2-WithoutConnectors
type=file
~~~
# SOURCE
~~~kerml

package WithoutConnectorsModelToBeExecuted {
	doc
	/* 
	 */

	classifier Bicycle {
		feature rollsOn : Wheel [2];
		feature holdsWheel : BikeFork [*];
	}
	classifier Wheel;
	classifier BikeFork;
}

package WithoutConnectorsExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import WithoutConnectorsModelToBeExecuted::*;

	#atom
	classifier MyWheel1 specializes Wheel;
	#atom
	classifier MyWheel2 specializes Wheel;

	classifier MyWheel unions MyWheel1, MyWheel2;

	#atom
	classifier MyBike specializes Bicycle {
		feature redefines rollsOn : MyWheel;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a_3_2_without_connectors.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 19 16) (end 19 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 22 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 24 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 6))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:cd7b2c5d63c5eaa2d805ae74d34045a0450b3e9cf91b1708811174433315549d") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " \n\t "))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "WithoutConnectorsModelToBeExecuted") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bicycle")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MyWheel")) (redefinition (reference "rollsOn")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "MyWheel1")) (unioning (reference "MyWheel2")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " \n\t "))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BikeFork")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "WithoutConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind specialization) (ordinal 0))
      (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MyWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rollsOn")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 0))
      (authored-target "MyWheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 1))
      (authored-target "MyWheel2")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "BikeFork")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")))))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike")))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike")))
      (type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")) (source direct))
      (effective-type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (source inherited) (from (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")) (scopes any))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn")) (scopes any feature))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2")))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1")))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2")))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel")))
      (featured-by (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")))
      (type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")) (provenance authored))
      (effective-type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")) (source direct))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn")))
      (featured-by (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")))
      (type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 19 16) (end 19 24)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 20 16) (end 20 53)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "WithoutConnectorsModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 30 31) (end 30 38)) (probe (position 30 31))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyBike"))) (kind specialization) (ordinal 0) (authored-target "Bicycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 31 30) (end 31 37)) (probe (position 31 30))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MyWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 31 20) (end 31 27)) (probe (position 31 20))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (path (named (kind package) (name "WithoutConnectorsExecution")) (named (kind kerml-classifier) (name "MyBike")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rollsOn")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 27 27) (end 27 35)) (probe (position 27 27))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 0) (authored-target "MyWheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 27 37) (end 27 45)) (probe (position 27 37))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel"))) (kind unioning) (ordinal 1) (authored-target "MyWheel2")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 23 33) (end 23 38)) (probe (position 23 33))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel1"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 25 33) (end 25 38)) (probe (position 25 33))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsExecution::MyWheel2"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 8 23) (end 8 31)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::holdsWheel"))) (kind featureTyping) (ordinal 0) (authored-target "BikeFork")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::BikeFork")))))
    )
  )
  (query (document "memory://snapshot/a_3_2_without_connectors.md") (range (start 7 20) (end 7 25)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Bicycle::rollsOn"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_2_without_connectors.md") (qualified-name "WithoutConnectorsModelToBeExecuted::Wheel")))))
    )
  )
)
~~~
