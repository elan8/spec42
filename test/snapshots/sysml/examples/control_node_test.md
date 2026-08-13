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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 2 1) (end 2 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 7 1) (end 7 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 9 1) (end 9 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 11 1) (end 11 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 12 1) (end 16 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 17 1) (end 17 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 18 1) (end 18 9))
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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 26 1) (end 26 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 31 1) (end 31 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 33 1) (end 33 9))
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
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::A2::a"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B1::b"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_node_test.md") (qualified-name "ControlNodeTest::B2::b"))) (kind parameter) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
