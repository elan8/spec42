# META
~~~ini
description=SysML Example (Simple Tests): AllocationTest
type=file
~~~
# SOURCE
~~~sysml
package AllocationTest {
	part def Logical {
		part component;
	}
	
	part def Physical {
		part assembly {
			part element;
		}
	}
	
	part l : Logical {
		part :>> component;
	}
	part p : Physical {
		part :>> assembly {
			part :>> element;
		}
        allocate l.component to assembly.element;
	}
	
	allocation def A;
	
	allocation def Logical_to_Physical :> A {
		end logical : Logical;
		end physical : Physical;
	}
	
	allocation allocation1 : Logical_to_Physical allocate l to p;	
	allocation allocation2 : Logical_to_Physical allocate (
		logical ::> l,
		physical ::> p
	);

	allocate l.component to p.assembly.element;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/allocation_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 2) (end 2 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 2) (end 8 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 3) (end 7 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 12) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 32) (end 18 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 29 1) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 25) (end 34 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:c26d93f48afb7e16dbdf626d3f719811fd741dd30cf67f6906c45bb44046f79a") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "l::component")) (allocateTarget (reference "p::assembly::element")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind allocation-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Logical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Physical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly::element"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Logical_to_Physical")) (allocateSource (reference "l")) (allocateTarget (reference "p")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Logical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "component")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Physical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "assembly")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "l::component")) (memberAccessOperand (reference "assembly::element")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "element")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "l::component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "p::assembly::element")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0))
      (authored-target "Logical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0))
      (authored-target "Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Logical_to_Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateSource) (ordinal 0))
      (authored-target "l")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateTarget) (ordinal 0))
      (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0))
      (authored-target "Logical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "l::component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "assembly::element")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "element")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly::element"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical")) (scopes any))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))
      (subtype (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical")))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))
      (type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical")))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))
      (type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical")) (scopes any))
      (subtype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))
      (subtype (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly::element")))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1")))
      (positional-ends (authored 0) (effective 2))
      (type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")) (scopes any))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")))
      (type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))
      (type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/allocation_test.md") (range (start 34 10) (end 34 21)) (probe (position 34 10))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "l::component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 34 25) (end 34 43)) (probe (position 34 25))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "p::assembly::element")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 23 39) (end 23 40)) (probe (position 23 39))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 24 16) (end 24 23)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0) (authored-target "Logical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 25 17) (end 25 25)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0) (authored-target "Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 28 26) (end 28 45)) (probe (position 28 26))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0) (authored-target "Logical_to_Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 28 55) (end 28 56)) (probe (position 28 55))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateSource) (ordinal 0) (authored-target "l")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 28 60) (end 28 61)) (probe (position 28 60))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::allocation1"))) (kind allocateTarget) (ordinal 0) (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 11 10) (end 11 17)) (probe (position 11 10))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0) (authored-target "Logical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 12 11) (end 12 20)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 14 10) (end 14 18)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0) (authored-target "Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 15 11) (end 15 19)) (probe (position 15 11))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 18 17) (end 18 28)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "l::component")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")))))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 18 32) (end 18 48)) (probe (position 18 32))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "assembly::element")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/allocation_test.md") (range (start 16 12) (end 16 19)) (probe (position 16 12))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "element")
      (outcome (status unresolved)))
    )
  )
)
~~~
