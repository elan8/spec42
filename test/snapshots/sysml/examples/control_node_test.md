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
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 6) (end 2 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 6) (end 7 7))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 9 1) (end 9 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 6) (end 11 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 11) (end 12 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 20 1) (end 20 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 21 1) (end 21 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 6) (end 26 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 6) (end 31 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 7) (end 33 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:44ae771608e2f329257fcd7ea6e7219b7f4a598f5a1809d9ede0b9a98b1eff35") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "J"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "J"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind join) (ordinal 0))))) (kind join) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (joinInput (reference "J"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind fork) (ordinal 0))))) (kind fork) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (forkInput (reference "F"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "B1"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 3))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "B2"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 4))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "M"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 5))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "M"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (anonymous (kind merge) (ordinal 0))))) (kind merge) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (mergeInput (reference "M"))))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::::a"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::::b1"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::::b2"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b"))) (kind parameter) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0))
      (authored-target "M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind fork) (ordinal 0))))) (kind forkInput) (ordinal 0))
      (authored-target "F")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind join) (ordinal 0))))) (kind joinInput) (ordinal 0))
      (authored-target "J")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "J")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "J")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "B1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0))
      (authored-target "B2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 4))))) (kind thenTarget) (ordinal 0))
      (authored-target "M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 5))))) (kind thenTarget) (ordinal 0))
      (authored-target "M")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/control_node_test.md") (range (start 33 7) (end 33 8)) (probe (position 33 7))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0) (authored-target "M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 12 11) (end 12 12)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind fork) (ordinal 0))))) (kind forkInput) (ordinal 0) (authored-target "F")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 11 6) (end 11 7)) (probe (position 11 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind join) (ordinal 0))))) (kind joinInput) (ordinal 0) (authored-target "J")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 2 6) (end 2 7)) (probe (position 2 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "J")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 7 6) (end 7 7)) (probe (position 7 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "J")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 17 6) (end 17 8)) (probe (position 17 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "B1")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1")))))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 18 6) (end 18 8)) (probe (position 18 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0) (authored-target "B2")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2")))))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 26 6) (end 26 7)) (probe (position 26 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 4))))) (kind thenTarget) (ordinal 0) (authored-target "M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_node_test.md") (range (start 31 6) (end 31 7)) (probe (position 31 6))
    (reference (id (source (node (document "memory://snapshot/control_node_test.md") (anonymous (kind then-continuation) (ordinal 5))))) (kind thenTarget) (ordinal 0) (authored-target "M")
      (outcome (status unresolved)))
  )
)
~~~
