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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dcbfbdf924b6d4bf1abd746971a808aa13c3cdd91f0536d907241f3984604d69") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StateTest"))) (kind "package") (name "StateTest") (declared-name "StateTest") (range (start (line 0) (character 0)) (end (line 0) (character 840))))
    (element (id (node (document "d0") (qualified-name "StateTest::Exit"))) (kind "attribute def") (name "Exit") (declared-name "Exit") (range (start (line 4) (character 1)) (end (line 4) (character 20))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S"))) (kind "state def") (name "S") (declared-name "S") (range (start (line 10) (character 1)) (end (line 10) (character 465))) (parent (node (document "d0") (qualified-name "StateTest"))) (authored (membership (kind Owning)) (relationships (transition (reference "StateTest::S::S2") (range none)) (transition (reference "StateTest::S::done") (range none)) (initial-state (reference "StateTest::S::S1") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S1"))) (kind "state") (name "S1") (declared-name "S1") (range (start (line 14) (character 2)) (end (line 14) (character 11))) (parent (node (document "d0") (qualified-name "StateTest::S"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S2::S3") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S2"))) (kind "state") (name "S2") (declared-name "S2") (range (start (line 19) (character 2)) (end (line 19) (character 61))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (kind "state") (name "S3") (declared-name "S3") (range (start (line 21) (character 3)) (end (line 21) (character 12))) (parent (node (document "d0") (qualified-name "StateTest::S::S2"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S1") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S3"))) (kind "state") (name "S3") (declared-name "S3") (range (start (line 40) (character 2)) (end (line 40) (character 30))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (kind "state") (name "S3a") (declared-name "S3a") (range (start (line 41) (character 3)) (end (line 41) (character 13))) (parent (node (document "d0") (qualified-name "StateTest::S::S3"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::S::S1") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T"))) (kind "transition") (name "T") (declared-name "T") (range (start (line 31) (character 2)) (end (line 31) (character 94))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 31) (character 2)) (end (line 31) (character 94))) (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 34) (character 6)) (end (line 34) (character 10))) (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::T::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 31) (character 2)) (end (line 31) (character 94))) (parent (node (document "d0") (qualified-name "StateTest::S::T"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 11) (character 2)) (end (line 11) (character 14))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 12) (character 2)) (end (line 12) (character 8))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::_exit"))) (kind "action") (name "exit") (declared-name "exit") (range (start (line 38) (character 2)) (end (line 38) (character 11))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))) (kind "transition") (name "transition_S1_to_S3") (declared-name "transition_S1_to_S3") (range (start (line 25) (character 2)) (end (line 25) (character 72))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 25) (character 2)) (end (line 25) (character 72))) (parent (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 25) (character 2)) (end (line 25) (character 72))) (parent (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S3a_to_S1"))) (kind "transition") (name "transition_S3a_to_S1") (declared-name "transition_S3a_to_S1") (range (start (line 44) (character 2)) (end (line 44) (character 34))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))) (kind "transition") (name "transition_S_to_S2") (declared-name "transition_S_to_S2") (range (start (line 15) (character 3)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 15) (character 3)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 15) (character 3)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))) (kind "transition") (name "transition_S_to_done") (declared-name "transition_S_to_done") (range (start (line 23) (character 2)) (end (line 23) (character 24))) (parent (node (document "d0") (qualified-name "StateTest::S"))))
    (element (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 23) (character 2)) (end (line 23) (character 24))) (parent (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))))
    (element (id (node (document "d0") (qualified-name "StateTest::Sig"))) (kind "attribute def") (name "Sig") (declared-name "Sig") (range (start (line 1) (character 1)) (end (line 1) (character 28))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::Sig::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 2) (character 2)) (end (line 2) (character 4))) (parent (node (document "d0") (qualified-name "StateTest::Sig"))))
    (element (id (node (document "d0") (qualified-name "StateTest::act"))) (kind "action") (name "act") (declared-name "act") (range (start (line 8) (character 1)) (end (line 8) (character 12))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 6) (character 1)) (end (line 6) (character 8))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s"))) (kind "state") (name "s") (declared-name "s") (range (start (line 57) (character 1)) (end (line 57) (character 46))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0"))) (kind "state") (name "s0") (declared-name "s0") (range (start (line 47) (character 1)) (end (line 47) (character 129))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s1"))) (kind "state") (name "s1") (declared-name "s1") (range (start (line 48) (character 4)) (end (line 48) (character 36))) (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (kind "state") (name "s2") (declared-name "s2") (range (start (line 49) (character 6)) (end (line 49) (character 15))) (parent (node (document "d0") (qualified-name "StateTest::s0::s1"))) (authored (membership (kind Feature)) (relationships (transition (reference "StateTest::s0::s3::s4") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s3"))) (kind "state") (name "s3") (declared-name "s3") (range (start (line 51) (character 4)) (end (line 51) (character 35))) (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (kind "state") (name "s4") (declared-name "s4") (range (start (line 52) (character 5)) (end (line 52) (character 14))) (parent (node (document "d0") (qualified-name "StateTest::s0::s3"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s0::t1"))) (kind "transition") (name "t1") (declared-name "t1") (range (start (line 54) (character 4)) (end (line 54) (character 41))) (parent (node (document "d0") (qualified-name "StateTest::s0"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s4"))) (kind "state") (name "s4") (declared-name "s4") (range (start (line 62) (character 1)) (end (line 62) (character 43))) (parent (node (document "d0") (qualified-name "StateTest"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s4::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 63) (character 2)) (end (line 63) (character 14))) (parent (node (document "d0") (qualified-name "StateTest::s4"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s5"))) (kind "state") (name "s5") (declared-name "s5") (range (start (line 67) (character 1)) (end (line 67) (character 43))) (parent (node (document "d0") (qualified-name "StateTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "s4") (range (start (line 67) (character 13)) (end (line 67) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "StateTest::s::s1"))) (kind "state") (name "s1") (declared-name "s1") (range (start (line 58) (character 2)) (end (line 58) (character 11))) (parent (node (document "d0") (qualified-name "StateTest::s"))))
    (element (id (node (document "d0") (qualified-name "StateTest::s::s2"))) (kind "state") (name "s2") (declared-name "s2") (range (start (line 59) (character 2)) (end (line 59) (character 11))) (parent (node (document "d0") (qualified-name "StateTest::s"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S2")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind transitionSource) (ordinal 1)) (authored-target "StateTest::S::done") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S"))) (kind initialStateSource) (ordinal 0)) (authored-target "StateTest::S::S1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S1"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S2::S3") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S2::S3")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::S::S1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (kind transitionSource) (ordinal 0)) (authored-target "StateTest::s0::s3::s4") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::s0::s3::s4")))))
    (reference (id (source (node (document "d0") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0)) (authored-target "s4") (range (start (line 67) (character 13)) (end (line 67) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateTest::s4")))))
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
