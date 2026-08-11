# META
~~~ini
description=SysML Example (Simple Tests): StateTest
type=file
~~~
# SOURCE
~~~sysml
package StateTest {
	attribute def Sig {
		x;
	}
	attribute def Exit;
	
	part p;
	
	action act;
	
	state def S {
		do action A;
		entry; then S1;
		
		state S1;
			accept s : Sig
			do action D
			then S2;
				
		state S2 {
			do send new Sig(T.s.x) to p;
			state S3;
		}
		accept Exit then done;
		
		transition
			first S1
			accept s : Sig
			do action D
			then S2.S3;
		
		transition T
			first S2.S3
			accept s : Sig via p
			if true
			do send s to p
			then S1;
			
		exit act;
		
		state S3 {
			state S3a;
		}
		
		transition first S3.S3a then S1; 
	}
	
	state s0 {
  		state s1 {
    		state s2;
  		}
  		state s3 {
  			state s4;
  		}
  		transition t1 first s1.s2 then s3.s4;
	}
	
	state s parallel {
		state s1;
		state s2;
	}
	
	state s4 {
		do action a;
  		action c;
	}
	
	state s5 :> s4 {
  		do action b :>> c;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 1) (end 10 465))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dcbfbdf924b6d4bf1abd746971a808aa13c3cdd91f0536d907241f3984604d69") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StateTest"))) (kind "package") (name "StateTest") (declared-name "StateTest"))
    (element (id (node (document "d0") (qualified-name "StateTest::Exit"))) (kind "attribute def") (name "Exit") (declared-name "Exit") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S"))) (kind "state def") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "StateTest"))) (authored (membership (kind Owning)) (relationships (transition (reference "StateTest::S::S2")) (transition (reference "StateTest::S::done")) (initial-state (reference "StateTest::S::S1")))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S1"))) (kind "state") (name "S1") (declared-name "S1") (parent (node (document "d0") (qualified-name "StateTest::S"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S2::S3")))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S2"))) (kind "state") (name "S2") (declared-name "S2") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (kind "state") (name "S3") (declared-name "S3") (parent (node (document "d0") (qualified-name "StateTest::S::S2"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S1")))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S3"))) (kind "state") (name "S3") (declared-name "S3") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (kind "state") (name "S3a") (declared-name "S3a") (parent (node (document "d0") (qualified-name "StateTest::S::S3"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S1")))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T"))) (kind "transition") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_exit"))) (kind "action") (name "exit") (declared-name "exit") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))) (kind "transition") (name "transition_S1_to_S3") (declared-name "transition_S1_to_S3") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S3a_to_S1"))) (kind "transition") (name "transition_S3a_to_S1") (declared-name "transition_S3a_to_S1") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))) (kind "transition") (name "transition_S_to_S2") (declared-name "transition_S_to_S2") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))) (kind "transition") (name "transition_S_to_done") (declared-name "transition_S_to_done") (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))))
    (element (id (node (document "d0") (qualified-name "StateTest::Sig"))) (kind "attribute def") (name "Sig") (declared-name "Sig") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::Sig::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "StateTest::Sig"))))
    (element (id (node (document "d0") (qualified-name "StateTest::act"))) (kind "action") (name "act") (declared-name "act") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s"))) (kind "state") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0"))) (kind "state") (name "s0") (declared-name "s0") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s1"))) (kind "state") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (kind "state") (name "s2") (declared-name "s2") (parent (node (document "d0") (qualified-name "StateTest::s0::s1"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::s0::s3::s4")))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s3"))) (kind "state") (name "s3") (declared-name "s3") (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (kind "state") (name "s4") (declared-name "s4") (parent (node (document "d0") (qualified-name "StateTest::s0::s3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::t1"))) (kind "transition") (name "t1") (declared-name "t1") (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s4"))) (kind "state") (name "s4") (declared-name "s4") (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s4::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "StateTest::s4"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s5"))) (kind "state") (name "s5") (declared-name "s5") (parent (node (document "d0") (qualified-name "StateTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "s4")))))
    (element (id (node (document "d0") (qualified-name "StateTest::s::s1"))) (kind "state") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "StateTest::s"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s::s2"))) (kind "state") (name "s2") (declared-name "s2") (parent (node (document "d0") (qualified-name "StateTest::s"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S2") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S2")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind transitionSource) (ordinal 1)) (authored-target "StateTest::S::done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind initialStateSource) (ordinal 0)) (authored-target "StateTest::S::S1") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S1"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S2::S3") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S2::S3")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S1") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S1") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::s0::s3::s4") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::s0::s3::s4")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0)) (authored-target "s4") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::s4")))))
  )
  (relationships
    (relationship (kind transition) (source (node (document "d0") (qualified-name "StateTest::S"))) (target (node (document "d0") (qualified-name "StateTest::S::S2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::S"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "StateTest::S"))) (target (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::S"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "StateTest::S::S1"))) (target (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::S::S1"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (target (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (target (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (target (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "StateTest::s5"))) (target (node (document "d0") (qualified-name "StateTest::s4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "StateTest::S::T::guard")) (expression (status "ok") (value (boolean true))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 67 13) (end 67 15)) (probe (position 67 13))
      (reference
        (source (document "d0") (qualified-name "StateTest::s5"))
        (kind subsetting) (ordinal 0) (authored-target "s4")
        (range (start 67 13) (end 67 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "StateTest::s4") (range (start 62 1) (end 62 43)))
        )
      )
    )
  )
)
~~~
