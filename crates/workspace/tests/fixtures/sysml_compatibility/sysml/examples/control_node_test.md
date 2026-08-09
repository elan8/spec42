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
# TOKENS
~~~zig
KwAction,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwJoin,Ident,Semicolon,
KwThen,KwFork,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwMerge,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (action_def 'ControlNodeTest'
    (action_usage 'A1')
    (source_succession
      (default_ref_usage 'J'))
    (action_usage 'A2'
      (default_ref_usage out 'a'))
    (source_succession
      (default_ref_usage 'J'))
    (flow_usage 'A2')
    (sysml_decl 'J')
    (source_succession
      (sysml_decl 'F'
        (default_ref_usage in 'a')
        (default_ref_usage out 'b1')
        (default_ref_usage out 'b2')))
    (source_succession
      (default_ref_usage 'B1'))
    (source_succession
      (default_ref_usage 'B2'))
    (flow_usage 'F')
    (flow_usage 'F')
    (action_usage 'B1'
      (default_ref_usage in 'b'))
    (source_succession
      (default_ref_usage 'M'))
    (action_usage 'B2'
      (default_ref_usage in 'b'))
    (source_succession
      (default_ref_usage 'M'))
    (sysml_decl 'M')))
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
# EXPECTED
~~~
semantic.duplicate_name 'J'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'J'
semantic.duplicate_name 'F'
semantic.duplicate_name 'F'
semantic.duplicate_name 'B1'
semantic.duplicate_name 'B2'
semantic.duplicate_name 'M'
semantic.duplicate_name 'M'
semantic.ambiguous_member 'A2'
semantic.ambiguous_member 'F'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'J'
semantic.duplicate_name 'A2'
semantic.duplicate_name 'J'
semantic.duplicate_name 'F'
semantic.duplicate_name 'F'
semantic.duplicate_name 'B1'
semantic.duplicate_name 'B2'
semantic.duplicate_name 'M'
semantic.duplicate_name 'M'
semantic.ambiguous_member 'A2'
semantic.ambiguous_member 'F'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "action def") (id (node (document "d0") (qualified-name "ControlNodeTest"))) (name "ControlNodeTest") (declared-name "ControlNodeTest")
      (contains
        (element (kind "action") (id (node (document "d0") (qualified-name "ControlNodeTest::A1"))) (name "A1") (declared-name "A1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
        (element (kind "action") (id (node (document "d0") (qualified-name "ControlNodeTest::A2"))) (name "A2") (declared-name "A2") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest"))))
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ControlNodeTest::A2::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (name "B1") (declared-name "B1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest"))))
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ControlNodeTest::B1::b"))) (name "b") (declared-name "b") (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "ControlNodeTest::B2"))) (name "B2") (declared-name "B2") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest"))))
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ControlNodeTest::B2::b"))) (name "b") (declared-name "b") (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
          )
        )
        (element (kind "join") (id (node (document "d0") (qualified-name "ControlNodeTest::J"))) (name "join") (declared-name "join") (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
        (element (kind "merge") (id (node (document "d0") (qualified-name "ControlNodeTest::M"))) (name "merge") (declared-name "merge") (effective (featuring-type (node (document "d0") (qualified-name "ControlNodeTest")))))
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest"))) (to (node (document "d0") (qualified-name "ControlNodeTest::J"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest::B1"))) (to (node (document "d0") (qualified-name "ControlNodeTest::B2"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest::B2"))) (to (node (document "d0") (qualified-name "ControlNodeTest::M"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest::J"))) (to (node (document "d0") (qualified-name "ControlNodeTest::B1"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest::J"))) (to (node (document "d0") (qualified-name "ControlNodeTest::J"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest::M"))) (to (node (document "d0") (qualified-name "ControlNodeTest::M"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest"))) (to (node (document "d0") (qualified-name "ControlNodeTest::A1"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest"))) (to (node (document "d0") (qualified-name "ControlNodeTest::A2"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest"))) (to (node (document "d0") (qualified-name "ControlNodeTest::B1"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ControlNodeTest"))) (to (node (document "d0") (qualified-name "ControlNodeTest::B2"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
