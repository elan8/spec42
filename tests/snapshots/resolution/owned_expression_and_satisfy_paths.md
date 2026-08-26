# META
~~~ini
description=Owned expression parameters and dotted satisfy subjects resolve without changing qualified-name lookup
type=file
~~~
# SOURCE
~~~sysml
package Requirements {
	requirement def Requirement;
	requirement nested : Requirement;
}

package Resolution {
	constraint def WithinLimit {
		in value;
		in limit;
		value <= limit
	}

	requirement def Requirement;
	requirement req : Requirement;
	part def Controller { part processor; }
	part def System { part controller : Controller; }
	part system : System;
	satisfy req by system.controller.processor;

	satisfy Requirements::nested by system.controller;
	satisfy missingRequirement by system.missing;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/owned_expression_and_satisfy_paths.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 14 23) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 9) (end 20 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 31) (end 20 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f91f9fcae45755eac195c69692108de419dbc8e5e1ecc38b39b0dd74b9980dda") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Requirement")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "req")) (memberAccessOperand (reference "system::controller::processor")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Requirements::nested")) (memberAccessOperand (reference "system::controller")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "missingRequirement")) (memberAccessOperand (reference "system::missing")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "value")) (expressionOperand (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Requirement")))))
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (kind featureTyping) (ordinal 0))
      (authored-target "Requirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "req")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0))
      (authored-target "Requirements::nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0))
      (authored-target "missingRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "system::controller::processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "system::controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "system::missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 0))
      (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 1))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (kind featureTyping) (ordinal 0))
      (authored-target "Requirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")))))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value"))) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")))
      (subtype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested")))
      (type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")) (provenance authored))
      (effective-type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")) (source direct))
      (supertype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")))
      (subtype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor")))
      (featured-by (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")))
      (subtype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")))
      (subtype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller")))
      (featured-by (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")))
      (type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")) (source direct))
      (supertype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit")))
      (featured-by (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit")))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value")))
      (featured-by (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit")))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req")))
      (type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")) (provenance authored))
      (effective-type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")) (source direct))
      (supertype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system")))
      (type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")) (provenance authored))
      (effective-type (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")) (source direct))
      (supertype (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 2 22) (end 2 33)) (probe (position 2 22))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested"))) (kind featureTyping) (ordinal 0) (authored-target "Requirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::Requirement")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 17 9) (end 17 12)) (probe (position 17 9))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "req")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 19 9) (end 19 29)) (probe (position 19 9))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0) (authored-target "Requirements::nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Requirements::nested")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 20 9) (end 20 27)) (probe (position 20 9))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0) (authored-target "missingRequirement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 17 16) (end 17 43)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "system::controller::processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller::processor")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 19 33) (end 19 50)) (probe (position 19 33))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "system::controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 20 31) (end 20 45)) (probe (position 20 31))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (path (named (kind package) (name "Resolution")) (anonymous (kind satisfy) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "system::missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 15 37) (end 15 47)) (probe (position 15 37))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System::controller"))) (kind featureTyping) (ordinal 0) (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Controller")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 9 2) (end 9 7)) (probe (position 9 2))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 0) (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::value")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 9 11) (end 9 16)) (probe (position 9 11))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit"))) (kind expressionOperand) (ordinal 1) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::WithinLimit::limit")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 13 19) (end 13 30)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::req"))) (kind featureTyping) (ordinal 0) (authored-target "Requirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::Requirement")))))
    )
  )
  (query (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (range (start 16 15) (end 16 21)) (probe (position 16 15))
    (reference (id (source (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/owned_expression_and_satisfy_paths.md") (qualified-name "Resolution::System")))))
    )
  )
)
~~~
