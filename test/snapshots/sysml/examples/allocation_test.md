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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 1) (end 28 62))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 34 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c26d93f48afb7e16dbdf626d3f719811fd741dd30cf67f6906c45bb44046f79a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind allocation-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Logical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Physical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly::element"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Logical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "component")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Physical")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "assembly")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "l::component")) (memberAccessOperand (reference "assembly::element")))))
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "element")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0))
      (authored-target "Logical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0))
      (authored-target "Physical")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")))))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::logical")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical_to_Physical::physical")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::l")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "l")) (anonymous (kind part) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Logical::component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::p")))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_test.md") (path (named (kind package) (name "AllocationTest")) (named (kind part) (name "p")) (anonymous (kind part) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/allocation_test.md") (qualified-name "AllocationTest::Physical::assembly")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
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
