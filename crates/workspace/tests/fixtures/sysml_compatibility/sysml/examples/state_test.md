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
        do A;
        entry;
        then S1;

        state S1;
        accept s : Sig do action D then S2;

        state S2 {
            state S3;
        }
        accept Exit then done;

        transition first S1 accept s : Sig do action D then S2 . S3;

        transition T first S2 . S3 accept s : Sig via p if true do send s to p then S1;

        exit act;

        state S3 {
            state S3a;
        }

        transition first S3 . S3a then S1;
    }

    state s0 {
        state s1 {
            state s2;
        }
        state s3 {
            state s4;
        }
        transition t1 first s1 . s2 then s3 . s4;
    }

    state s parallel {
        state s1;
        state s2;
    }

    state s4 {
        do a;
        action c;
    }

    state s5 :> s4 {
        do b :>> c;
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
(model
  (namespace
    (package 'StateTest'
      (attribute_def 'Sig'
        (reference_usage reference 'x'))
      (attribute_def 'Exit')
      (part_usage 'p')
      (action_usage 'act')
      (state_def 'S'
        (state_subaction_membership 'do'
          (action_usage 'A'))
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'S1'))
        (state_usage composite 'S1')
        (transition_usage)
        (state_usage composite 'S2'
          (not_implemented 'malformed'))
        (transition_usage)
        (transition_usage)
        (transition_usage 'T')
        (state_subaction_membership 'exit'
          (action_usage 'act'))
        (state_usage composite 'S3'
          (state_usage composite 'S3a'))
        (transition_usage))
      (state_usage 's0'
        (state_usage composite 's1'
          (state_usage composite 's2'))
        (state_usage composite 's3'
          (state_usage composite 's4'))
        (transition_usage 't1'))
      (state_usage parallel 's'
        (state_usage composite 's1')
        (state_usage composite 's2'))
      (state_usage 's4'
        (state_subaction_membership 'do'
          (action_usage 'a'))
        (action_usage composite 'c'))
      (state_usage 's5' :> 'StateTest::s4'[state_usage]
        (state_subaction_membership 'do' :>> 'StateTest::s4::do'[state_subaction_membership][implied]
          (action_usage 'b' :>> 'StateTest::s4::c'[action_usage]))))))
~~~
