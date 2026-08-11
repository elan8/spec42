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
  (document "control_node_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 5) (end 5 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 14) (end 9 17))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 12 1) (end 12 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 6) (end 20 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 6) (end 21 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 5) (end 24 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 5) (end 29 10))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "4a9efb6eb190d426579177146445c53c54861d7d8d35760914ed4b691ef444cd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ControlNodeTest"))) (kind "action def") (name "ControlNodeTest") (declared-name "ControlNodeTest") (range (start (line 0) (character 0)) (end (line 0) (character 329))) (authored (membership (kind Owning)) (relationships (flow (reference "ControlNodeTest::J") (range none)) (perform (reference "ControlNodeTest::A1") (range none)) (perform (reference "ControlNodeTest::A2") (range none)) (perform (reference "ControlNodeTest::B1") (range none)) (perform (reference "ControlNodeTest::B2") (range none)))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::A1"))) (kind "action") (name "A1") (declared-name "A1") (range (start (line 1) (character 1)) (end (line 1) (character 11))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::A2"))) (kind "action") (name "A2") (declared-name "A2") (range (start (line 4) (character 1)) (end (line 4) (character 27))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::A2::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 5) (character 5)) (end (line 5) (character 11))) (parent (node (document "d0") (qualified-name "ControlNodeTest::A2"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (kind "action") (name "B1") (declared-name "B1") (range (start (line 23) (character 1)) (end (line 23) (character 26))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::B1::b"))) (kind "in out parameter") (name "b") (declared-name "b") (range (start (line 24) (character 5)) (end (line 24) (character 10))) (parent (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::B2"))) (kind "action") (name "B2") (declared-name "B2") (range (start (line 28) (character 1)) (end (line 28) (character 26))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::B2::b"))) (kind "in out parameter") (name "b") (declared-name "b") (range (start (line 29) (character 5)) (end (line 29) (character 10))) (parent (node (document "d0") (qualified-name "ControlNodeTest::B2"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::J"))) (kind "join") (name "join") (declared-name "join") (range (start (line 11) (character 1)) (end (line 11) (character 8))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))) (authored (relationships (flow (reference "ControlNodeTest::B1") (range none)))))
    (element (id (node (document "d0") (qualified-name "ControlNodeTest::M"))) (kind "merge") (name "merge") (declared-name "merge") (range (start (line 33) (character 1)) (end (line 33) (character 9))) (parent (node (document "d0") (qualified-name "ControlNodeTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowSource) (ordinal 0)) (authored-target "ControlNodeTest::J") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::J")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowSource) (ordinal 0)) (authored-target "A2::a") (range (start (line 9) (character 6)) (end (line 9) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::A2::a")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowSource) (ordinal 1)) (authored-target "F::b1") (range (start (line 20) (character 6)) (end (line 20) (character 10))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowSource) (ordinal 2)) (authored-target "F::b2") (range (start (line 21) (character 6)) (end (line 21) (character 10))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowTarget) (ordinal 0)) (authored-target "F::a") (range (start (line 9) (character 14)) (end (line 9) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowTarget) (ordinal 1)) (authored-target "B1::b") (range (start (line 20) (character 14)) (end (line 20) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::B1::b")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowTarget) (ordinal 2)) (authored-target "B2::b") (range (start (line 21) (character 14)) (end (line 21) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::B2::b")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 0)) (authored-target "ControlNodeTest::A1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 1)) (authored-target "ControlNodeTest::A2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::A2")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 2)) (authored-target "ControlNodeTest::B1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::B1")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 3)) (authored-target "ControlNodeTest::B2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::B2")))))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest::A2::a"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest::B1::b"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest::B2::b"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlNodeTest::J"))) (kind flowSource) (ordinal 0)) (authored-target "ControlNodeTest::B1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ControlNodeTest::B1")))))
  )
  (relationships
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ControlNodeTest"))) (target (node (document "d0") (qualified-name "ControlNodeTest::J"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind flowSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ControlNodeTest"))) (target (node (document "d0") (qualified-name "ControlNodeTest::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ControlNodeTest"))) (target (node (document "d0") (qualified-name "ControlNodeTest::A2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ControlNodeTest"))) (target (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ControlNodeTest"))) (target (node (document "d0") (qualified-name "ControlNodeTest::B2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest"))) (kind performSource) (ordinal 3)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ControlNodeTest::J"))) (target (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ControlNodeTest::J"))) (kind flowSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
