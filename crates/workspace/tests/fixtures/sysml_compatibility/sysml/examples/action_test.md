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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ActionTest"))) (name "ActionTest") (declared-name "ActionTest")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "ActionTest::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ActionTest::A::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::A")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ActionTest::S"))) (name "S") (declared-name "S") (declared (properties (ordered false) (unique true))))
        (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a"))) (name "a") (declared-name "a") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "ActionTest::a::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::A")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a::b"))) (name "b") (declared-name "b") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::A"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::A")))))
              )
            )
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a1"))) (name "a1") (declared-name "a1") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "ActionTest::a1::_initial"))) (name "_initial"))
            (element (kind "merge") (id (node (document "d0") (qualified-name "ActionTest::a1::m"))) (name "merge") (declared-name "merge"))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a2"))) (name "a2") (declared-name "a2") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a2::aa"))) (name "aa") (declared-name "aa") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "ActionTest::a2::aa::target"))) (name "target") (declared-name "target") (declared (properties (direction "out") (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ActionTest::a2::s"))) (name "s") (declared-name "s"))
            (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a2::snd"))) (name "snd") (declared-name "snd") (declared (properties (composite true) (reference false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::a2::snd2"))) (name "snd2") (declared-name "snd2") (declared (properties (composite true) (reference false))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::b"))) (name "b") (declared-name "b") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "ActionTest::b::a"))) (name "a") (declared-name "a") (declared (properties (composite false) (reference true))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "ActionTest::b::f : ScalarValues::Boolean"))) (name "f : ScalarValues::Boolean") (declared-name "f : ScalarValues::Boolean"))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "ActionTest::c"))) (name "c") (declared-name "c")
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "ActionTest::c::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::c")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "ActionTest::c::c1"))) (name "c1") (declared-name "c1") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::c"))))
              (contains
                (element (kind "terminate") (id (node (document "d0") (qualified-name "ActionTest::c::c1::_terminate"))) (name "terminate") (declared-name "terminate") (effective (featuring-type (node (document "d0") (qualified-name "ActionTest::c")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "ActionTest::A::x"))) (to (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (connect (source-expression "x") (target-expression "b::y") (container-prefix "ActionTest::a")))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a"))) (to (node (document "d0") (qualified-name "ActionTest::a::b"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a2"))) (to (node (document "d0") (qualified-name "ActionTest::a2::aa"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a2"))) (to (node (document "d0") (qualified-name "ActionTest::a2::snd"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a2"))) (to (node (document "d0") (qualified-name "ActionTest::a2::snd2"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ActionTest::c"))) (to (node (document "d0") (qualified-name "ActionTest::c::c1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a"))) (to (node (document "d0") (qualified-name "ActionTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ActionTest::a2::s"))) (to (node (document "d0") (qualified-name "ActionTest::S"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ActionTest::b::a"))) (to (node (document "d0") (qualified-name "ActionTest::A"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "ActionTest::a1::_initial") (target-qualified "ActionTest::a1::start"))
    (flow (status pending) (document "d0") (source-qualified "ActionTest::a1::m") (target-qualified "ActionTest::a1::decide"))
    (flow (status pending) (document "d0") (source-qualified "ActionTest::a::_initial") (target-qualified "ActionTest::a::start"))
    (flow (status pending) (document "d0") (source-qualified "ActionTest::c::_initial") (target-qualified "ActionTest::c::start"))
    (flow (status pending) (document "d0") (source-qualified "ActionTest::c::c1") (target-qualified "ActionTest::c::terminate"))
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "s") (target-expression "snd2::payload") (container-prefix "ActionTest::a2"))
  )
)
~~~
