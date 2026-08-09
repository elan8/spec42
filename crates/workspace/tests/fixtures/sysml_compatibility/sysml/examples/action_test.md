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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Semicolon,CloseCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAccept,Ident,Semicolon,
KwThen,KwAccept,Ident,KwAfter,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
KwThen,KwAccept,Ident,Ident,Ident,ColonColon,Ident,OpenParen,StringValue,CloseParen,Semicolon,
KwThen,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,Semicolon,
KwThen,KwAccept,KwWhen,Ident,Dot,Ident,Semicolon,
KwThen,KwDecide,Semicolon,
KwIf,KwTrue,KwThen,Ident,Semicolon,
KwElse,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwOut,KwPart,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,KwSend,OpenCurly,
KwIn,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAction,Ident,KwSend,KwVia,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwRef,KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwTerminate,Ident,Semicolon,
CloseCurly,
KwThen,KwTerminate,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ActionTest'
    (action_def 'A'
      (default_ref_usage in 'x'))
    (action_usage 'a' : 'A'
      (initial_node start)
      (action_usage 'b'
        (default_ref_usage in 'y' value))
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (attribute_def 'S')
    (action_usage 'a1'
      (initial_node start)
      (source_succession
        (sysml_decl 'm'))
      (source_succession
        (accept_node))
      (source_succession
        (accept_node))
      (source_succession
        (accept_node))
      (source_succession
        (send_node))
      (source_succession
        (accept_node))
      (source_succession
        (sysml_decl))
      (if_node)
      (source_succession
        (default_ref_usage 'm'))
      (source_succession
        (default_ref_usage 'done')))
    (action_usage 'a2'
      (default_ref_usage in 's' : 'S')
      (action_usage 'aa'
        (part_usage out 'target'))
      (flow_usage 'aa')
      (action_usage 'snd')
      (send_node)
      (action_usage 'snd2')
      (send_node)
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (action_usage 'b'
      (attribute_usage 'f' : 'ScalarValues::Boolean')
      (action_usage ref 'a' : 'A'))
    (action_def 'c'
      (initial_node start)
      (source_succession
        (action_usage 'c1'
          (terminate_node c1)))
      (source_succession
        (terminate_node)))))
~~~
# FORMAT
~~~sysml
package ActionTest {
    action def A {
        in x;
    }

    action a : A {
        first start;

        action b {
            in y = x;
        }

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
        if true;
        then m;
        else done;
    }

    action a2 {
        in s : S;
        action aa {
            out part target;
        }
        flow aa;
        action snd;
        send {
            in :>> payload = s;
        }
        action snd2;
        send via this to aa.target;
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
# EXPECTED
~~~
semantic.duplicate_name 'm'
semantic.duplicate_name 'aa'
semantic.invalid_connection_end_count
semantic.unresolved_name 'payload'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'm'
semantic.duplicate_name 'aa'
semantic.invalid_connection_end_count
semantic.unresolved_name 'payload'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ActionTest'
      (action_def 'A'
        (reference_usage in reference 'x'))
      (action_usage 'a' : 'ActionTest::A'[action_def]
        (initial_node)
        (action_usage composite 'b'
          (reference_usage in reference 'y'
            (feature_value (=))))
        (binding_connector_def
          (connector_end 'x')
          (connector_end 'b.y')))
      (attribute_def 'S')
      (action_usage 'a1'
        (initial_node)
        (source_succession
          (merge_node 'm'))
        (source_succession
          (accept_action_usage))
        (source_succession
          (accept_action_usage))
        (source_succession
          (accept_action_usage))
        (source_succession
          (send_action_usage))
        (source_succession
          (accept_action_usage))
        (source_succession
          (decide_node))
        (if_action_usage)
        (source_succession
          (reference_usage reference 'm'))
        (source_succession
          (reference_usage reference 'done')))
      (action_usage 'a2'
        (reference_usage in reference 's' : 'ActionTest::S'[attribute_def])
        (action_usage composite 'aa'
          (part_usage out 'target'))
        (flow_usage composite 'aa')
        (action_usage composite 'snd')
        (send_action_usage
          (reference_usage in reference :>> 'payload'[unresolved]
            (feature_value (=))))
        (action_usage composite 'snd2')
        (send_action_usage)
        (binding_connector_def
          (connector_end 's')
          (connector_end 'snd2.payload')))
      (action_usage 'b'
        (attribute_usage composite 'f' : 'ScalarValues::Boolean'[unresolved])
        (action_usage reference 'a' : 'ActionTest::A'[action_def]))
      (action_def 'c'
        (initial_node)
        (source_succession
          (action_usage 'c1'
            (terminate_action_usage)))
        (source_succession
          (terminate_action_usage))))))
~~~
