# META
~~~ini
description=SysML Example (Simple Tests): ControlNodeTest
type=file
~~~
# SOURCE
~~~sysml
action def ControlNodeTest {
	action A1;
	then J;
	
	action A2 {
	    out a;
	}
	then J;
	
	flow A2.a to F.a;
	
	join J;
	then fork F {
	    in a;
	    out b1;
	    out b2;
	}
	then B1;
	then B2;
	
	flow F.b1 to B1.b;
	flow F.b2 to B2.b;
		
	action B1 {
	    in b;
	}
	then M;
	
	action B2 {
	    in b;
	}
	then M; 
	
	merge M;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/control_node_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:44ae771608e2f329257fcd7ea6e7219b7f4a598f5a1809d9ede0b9a98b1eff35") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "J")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "J")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A2::a")) (flowTarget (reference "F::a")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "B1")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "B2")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "F::b1")) (flowTarget (reference "B1::b")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "F::b2")) (flowTarget (reference "B2::b")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "M")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "M")))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J"))) (kind join) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M"))) (kind merge) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "J")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "J")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "B1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0))
      (authored-target "B2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (kind thenTarget) (ordinal 0))
      (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (kind thenTarget) (ordinal 0))
      (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "A2::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0))
      (authored-target "F::b1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowSource) (ordinal 0))
      (authored-target "F::b2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "F::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0))
      (authored-target "B1::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowTarget) (ordinal 0))
      (authored-target "B2::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b")))))
  )
  (relationships
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A1"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M"))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5)))))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A1")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M")))
      (featured-by (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/control_node_test.md") (range (start 2 6) (end 2 7)) (probe (position 2 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "J")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 7 6) (end 7 7)) (probe (position 7 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "J")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::J")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 17 6) (end 17 8)) (probe (position 17 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "B1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 18 6) (end 18 8)) (probe (position 18 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0) (authored-target "B2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 26 6) (end 26 7)) (probe (position 26 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 4))))) (kind thenTarget) (ordinal 0) (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 31 6) (end 31 7)) (probe (position 31 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind then-continuation) (ordinal 5))))) (kind thenTarget) (ordinal 0) (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::M")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 9 6) (end 9 10)) (probe (position 9 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "A2::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 20 6) (end 20 10)) (probe (position 20 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0) (authored-target "F::b1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b1")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 21 6) (end 21 10)) (probe (position 21 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowSource) (ordinal 0) (authored-target "F::b2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::b2")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 9 14) (end 9 17)) (probe (position 9 14))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "F::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::F::a")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 20 14) (end 20 18)) (probe (position 20 14))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0) (authored-target "B1::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b")))))
    )
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 21 14) (end 21 18)) (probe (position 21 14))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (path (named (kind action-def) (name "ControlNodeTest")) (anonymous (kind flow) (ordinal 2))))) (kind flowTarget) (ordinal 0) (authored-target "B2::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b")))))
    )
  )
)
~~~
