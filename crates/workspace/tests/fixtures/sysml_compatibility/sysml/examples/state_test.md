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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAttribute,KwDef,Ident,OpenCurly,
Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwState,KwDef,Ident,OpenCurly,
KwDo,KwAction,Ident,Semicolon,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,
KwDo,KwAction,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwDo,KwSend,Ident,Ident,OpenParen,Ident,Dot,Ident,Dot,Ident,CloseParen,KwTo,Ident,Semicolon,
KwState,Ident,Semicolon,
CloseCurly,
KwAccept,Ident,KwThen,Ident,Semicolon,
KwTransition,
KwFirst,Ident,
KwAccept,Ident,Colon,Ident,
KwDo,KwAction,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,Dot,Ident,
KwAccept,Ident,Colon,Ident,KwVia,Ident,
KwIf,KwTrue,
KwDo,KwSend,Ident,KwTo,Ident,
KwThen,Ident,Semicolon,
KwExit,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwState,Ident,Semicolon,
CloseCurly,
KwTransition,KwFirst,Ident,Dot,Ident,KwThen,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwState,Ident,OpenCurly,
KwState,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwState,Ident,Semicolon,
CloseCurly,
KwTransition,Ident,KwFirst,Ident,Dot,Ident,KwThen,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwState,Ident,KwParallel,OpenCurly,
KwState,Ident,Semicolon,
KwState,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwDo,KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
CloseCurly,
KwState,Ident,ColonGt,Ident,OpenCurly,
KwDo,KwAction,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'StateTest'
    (attribute_def 'Sig'
      (default_ref_usage 'x'))
    (attribute_def 'Exit')
    (part_usage 'p')
    (action_usage 'act')
    (state_def 'S'
      (do_action 'A')
      (entry_action)
      (source_succession
        (default_ref_usage 'S1'))
      (state_usage 'S1')
      (target_transition)
      (state_usage 'S2'
        (malformed))
      (target_transition)
      (transition_usage)
      (transition_usage 'T')
      (exit_action 'act')
      (state_usage 'S3'
        (state_usage 'S3a'))
      (transition_usage))
    (state_usage 's0'
      (state_usage 's1'
        (state_usage 's2'))
      (state_usage 's3'
        (state_usage 's4'))
      (transition_usage 't1'))
    (state_usage parallel 's'
      (state_usage 's1')
      (state_usage 's2'))
    (state_usage 's4'
      (do_action 'a')
      (action_usage 'c'))
    (state_usage 's5' :> 's4'
      (do_action 'b' :>> 'c'))))
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
semantic.duplicate_name 'S1'
semantic.redefinition_featuring_type_overlap
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
semantic.duplicate_name 'S1'
semantic.redefinition_featuring_type_overlap
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "StateTest"))) (name "StateTest") (declared-name "StateTest")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateTest::Exit"))) (name "Exit") (declared-name "Exit") (declared (properties (ordered false) (unique true))))
        (element (kind "state def") (id (node (document "d0") (qualified-name "StateTest::S"))) (name "S") (declared-name "S")
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::S::S1"))) (name "S1") (declared-name "S1") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::S::S2"))) (name "S2") (declared-name "S2") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (name "S3") (declared-name "S3") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::S::S3"))) (name "S3") (declared-name "S3") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (name "S3a") (declared-name "S3a") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::S::T"))) (name "T") (declared-name "T") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "transition effect") (id (node (document "d0") (qualified-name "StateTest::S::T::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
                (element (kind "transition guard") (id (node (document "d0") (qualified-name "StateTest::S::T::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "booleanLiteral") (literal (boolean true))))) (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))) (evaluation (expression (status "ok") (value (boolean true)))))
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "StateTest::S::T::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "StateTest::S::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "StateTest::S::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "StateTest::S::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))) (name "transition_S1_to_S3") (declared-name "transition_S1_to_S3") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "transition effect") (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::S::transition_S3a_to_S1"))) (name "transition_S3a_to_S1") (declared-name "transition_S3a_to_S1") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))) (name "transition_S_to_S2") (declared-name "transition_S_to_S2") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "transition effect") (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))) (name "transition_S_to_done") (declared-name "transition_S_to_done") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "StateTest::S")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateTest::Sig"))) (name "Sig") (declared-name "Sig") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "StateTest::Sig::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "StateTest::Sig")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "StateTest::act"))) (name "act") (declared-name "act") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "StateTest::p"))) (name "p") (declared-name "p") (declared (properties (ordered false))))
        (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s"))) (name "s") (declared-name "s") (declared)
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s::s1"))) (name "s1") (declared-name "s1"))
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s::s2"))) (name "s2") (declared-name "s2"))
          )
        )
        (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s0"))) (name "s0") (declared-name "s0") (declared)
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s0::s1"))) (name "s1") (declared-name "s1")
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (name "s2") (declared-name "s2"))
              )
            )
            (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s0::s3"))) (name "s3") (declared-name "s3")
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (name "s4") (declared-name "s4"))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "StateTest::s0::t1"))) (name "t1") (declared-name "t1"))
          )
        )
        (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s4"))) (name "s4") (declared-name "s4") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "StateTest::s4::_do"))) (name "do") (declared-name "do"))
          )
        )
        (element (kind "state") (id (node (document "d0") (qualified-name "StateTest::s5"))) (name "s5") (declared-name "s5") (declared))
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "StateTest::S"))) (to (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "StateTest::s5"))) (to (node (document "d0") (qualified-name "StateTest::s4"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "StateTest::S"))) (to (node (document "d0") (qualified-name "StateTest::S::S2"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "StateTest::S::S1"))) (to (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (to (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (to (node (document "d0") (qualified-name "StateTest::S::S1"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (to (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (provenance authored))
  )
  (pending-relationships
    (transition (status pending) (document "d0") (source-qualified "StateTest::S") (target-qualified "StateTest::S::done"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::Exit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::S1"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::S2"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::S2::S3"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::S3"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::S3::S3a"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::T"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::T::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::T::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::_do"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::_exit"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S1_to_S3::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S3a_to_S1"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S_to_S2::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::S::transition_S_to_done::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::Sig"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::Sig::x"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::act"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::p"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0::s1"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0::s1::s2"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0::s3"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0::s3::s4"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s0::t1"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s4"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s4::_do"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s5"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s::s1"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateTest::s::s2"))) (status missing-prerequisite) (target "States::stateActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/state_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 8))
      )
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 15 3) (end 15 44))
      )
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 25 2) (end 25 72))
      )
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 31 2) (end 31 94))
      )
    )
  )
)
~~~
