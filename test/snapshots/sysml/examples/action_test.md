# META
~~~ini
description=SysML Example (Simple Tests): ActionTest
type=file
~~~
# SOURCE
~~~sysml
package ActionTest {
	action def A{ in x; }
	
	action a: A { 
		first start;
		
		action b { in y = x; }
		
		bind x = b.y;
	}
	
	attribute def S;
	
	action a1 {
		first start;		
		then merge m;
		then accept S;
		then accept sig after 10[SI::s]; 
		then accept at new Time::Iso8601DateTime("2022-01-30T01:00:00Z");
		
		then send new S() to b;
		then accept when b.f;
		then decide;
			if true then m;
			else done;
	}
	
	action a2 {
		in s : S;
		action aa {
			out part target;
		}
		flow aa.target to snd.receiver;
		action snd send { 
			in :>> payload = s;
		}
		action snd2 send via this to aa.target;
		bind s = snd2.payload;
	}
	
	action b {
		attribute f : ScalarValues::Boolean;
		ref action a : A;
	}
	
	action def c {
		first start;
		then action c1 {
			terminate c1;
		}
		then terminate;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 15) (end 1 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 2) (end 4 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 13) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 7) (end 8 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 2) (end 14 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 2) (end 15 15))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 16 2) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 16 2) (end 16 19))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 24 3) (end 24 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 20) (end 32 32))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 33 13) (end 33 50))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 33 13) (end 33 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 11) (end 37 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 2) (end 46 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 2) (end 47 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 2) (end 47 39))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package ActionTest {
    action def A{ in x; }

    action a: A {
        first start;

        action b { in y = x; }

        bind x = b.y;
    }

    attribute def S;

    action a1 {
        first start;
        then merge m;
        then accept S;
        then accept sig after 10[SI::s];
        then accept at new Time::Iso8601DateTime("2022-01-30T01:00:00Z");

        then send new S() to b;
        then accept when b.f;
        then decide;
        if true then m;
        else done;
    }

    action a2 {
        in s : S;
        action aa {
            out part target;
        }
        flow aa.target to snd.receiver;
        action snd send {
            in :>> payload = s;
        }
        action snd2 send via this to aa.target;
        bind s = snd2.payload;
    }

    action b {
        attribute f : ScalarValues::Boolean;
        ref action a : A;
    }

    action def c {
        first start;
        then action c1 {
            terminate c1;
        }
        then terminate;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "252f11a5d2075ba44fdae38a0d002660973f4315bc1627810929603ee9c82202") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ActionTest"))) (kind "package") (name "ActionTest") (declared-name "ActionTest"))
    (element (id (node (document "d0") (qualified-name "ActionTest::A"))) (kind "action def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::A::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "ActionTest::A"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::S"))) (kind "attribute def") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a"))) (kind "action") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")) (perform (reference "ActionTest::a::b")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1"))) (kind "action") (name "a1") (declared-name "a1") (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "ActionTest::a1"))) (authored (relationships (flow (reference "ActionTest::a1::start")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1::m"))) (kind "merge") (name "merge") (declared-name "merge") (parent (node (document "d0") (qualified-name "ActionTest::a1"))) (authored (relationships (flow (reference "ActionTest::a1::decide")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2"))) (kind "action") (name "a2") (declared-name "a2") (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Feature)) (relationships (perform (reference "ActionTest::a2::aa")) (perform (reference "ActionTest::a2::snd")) (perform (reference "ActionTest::a2::snd2")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::aa"))) (kind "action") (name "aa") (declared-name "aa") (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::aa::target"))) (kind "part") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "ActionTest::a2::aa"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::s"))) (kind "in out parameter") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "ActionTest::a2"))) (authored (relationships (typing (reference "S")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::snd"))) (kind "action") (name "snd") (declared-name "snd") (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::snd2"))) (kind "action") (name "snd2") (declared-name "snd2") (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "ActionTest::a"))) (authored (relationships (flow (reference "ActionTest::a::start")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::b"))) (kind "action") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "ActionTest::a"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "ActionTest::a::b"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b"))) (kind "action") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b::a"))) (kind "ref") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "ActionTest::b"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b::f : ScalarValues::Boolean"))) (kind "action body decl") (name "f : ScalarValues::Boolean") (declared-name "f : ScalarValues::Boolean") (parent (node (document "d0") (qualified-name "ActionTest::b"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c"))) (kind "action def") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "ActionTest::c::c1")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "ActionTest::c"))) (authored (relationships (flow (reference "ActionTest::c::start")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind "action") (name "c1") (declared-name "c1") (parent (node (document "d0") (qualified-name "ActionTest::c"))) (authored (relationships (typing (reference "")) (flow (reference "ActionTest::c::terminate")))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::c1::_terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (parent (node (document "d0") (qualified-name "ActionTest::c::c1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind bindSource) (ordinal 0)) (authored-target "x") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind bindTarget) (ordinal 0)) (authored-target "b::y") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a::b::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::a::b") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a::b")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a1::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a1::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a1::m"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a1::decide") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind bindSource) (ordinal 1)) (authored-target "s") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::s")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind bindTarget) (ordinal 1)) (authored-target "snd2::payload") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind flowSource) (ordinal 0)) (authored-target "aa::target") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::aa::target")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind flowTarget) (ordinal 0)) (authored-target "snd::receiver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::a2::aa") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::aa")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 1)) (authored-target "ActionTest::a2::snd") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::snd")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 2)) (authored-target "ActionTest::a2::snd2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::snd2")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2::s"))) (kind featureTyping) (ordinal 0)) (authored-target "S") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::S")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::b::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::c::c1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::c::c1")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::c::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::c::terminate") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ActionTest::a"))) (target (node (document "d0") (qualified-name "ActionTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ActionTest::a"))) (target (node (document "d0") (qualified-name "ActionTest::a::b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ActionTest::a2"))) (target (node (document "d0") (qualified-name "ActionTest::a2::aa"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ActionTest::a2"))) (target (node (document "d0") (qualified-name "ActionTest::a2::snd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ActionTest::a2"))) (target (node (document "d0") (qualified-name "ActionTest::a2::snd2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ActionTest::a2::s"))) (target (node (document "d0") (qualified-name "ActionTest::S"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::a2::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ActionTest::b::a"))) (target (node (document "d0") (qualified-name "ActionTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::b::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ActionTest::c"))) (target (node (document "d0") (qualified-name "ActionTest::c::c1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ActionTest::c"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ActionTest::a::b::y")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 7) (end 8 8)) (probe (position 8 7))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a"))
        (kind bindSource) (ordinal 0) (authored-target "x")
        (range (start 8 7) (end 8 8))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 7) (end 37 8)) (probe (position 37 7))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a2"))
        (kind bindSource) (ordinal 1) (authored-target "s")
        (range (start 37 7) (end 37 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ActionTest::a2::s") (range (start 28 2) (end 28 11)))
        )
      )
    )
    (query (range (start 42 17) (end 42 18)) (probe (position 42 17))
      (reference
        (source (document "d0") (qualified-name "ActionTest::b::a"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 42 17) (end 42 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ActionTest::A") (range (start 1 1) (end 1 22)))
        )
      )
    )
    (query (range (start 8 11) (end 8 14)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a"))
        (kind bindTarget) (ordinal 0) (authored-target "b::y")
        (range (start 8 11) (end 8 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ActionTest::a::b::y") (range (start 6 13) (end 6 22)))
        )
      )
    )
    (query (range (start 32 7) (end 32 16)) (probe (position 32 7))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a2"))
        (kind flowSource) (ordinal 0) (authored-target "aa::target")
        (range (start 32 7) (end 32 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ActionTest::a2::aa::target") (range (start 30 3) (end 30 19)))
        )
      )
    )
    (query (range (start 32 20) (end 32 32)) (probe (position 32 20))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a2"))
        (kind flowTarget) (ordinal 0) (authored-target "snd::receiver")
        (range (start 32 20) (end 32 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 11) (end 37 23)) (probe (position 37 11))
      (reference
        (source (document "d0") (qualified-name "ActionTest::a2"))
        (kind bindTarget) (ordinal 1) (authored-target "snd2::payload")
        (range (start 37 11) (end 37 23))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
