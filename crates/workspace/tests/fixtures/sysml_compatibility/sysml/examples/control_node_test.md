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

    flow A2;

    join J;
    then fork F {
	    in a;
	    out b1;
	    out b2;
	}
    then B1;
    then B2;

    flow F;
    flow F;

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
(model
  (namespace
    (action_def 'ControlNodeTest'
      (action_usage composite 'A1')
      (source_succession
        (reference_usage reference 'J'))
      (action_usage composite 'A2'
        (reference_usage out reference 'a'))
      (source_succession
        (reference_usage reference 'J'))
      (flow_usage composite 'A2')
      (join_node 'J')
      (source_succession
        (fork_node 'F'
          (reference_usage in reference 'a')
          (reference_usage out reference 'b1')
          (reference_usage out reference 'b2')))
      (source_succession
        (reference_usage reference 'B1'))
      (source_succession
        (reference_usage reference 'B2'))
      (flow_usage composite 'F')
      (flow_usage composite 'F')
      (action_usage composite 'B1'
        (reference_usage in reference 'b'))
      (source_succession
        (reference_usage reference 'M'))
      (action_usage composite 'B2'
        (reference_usage in reference 'b'))
      (source_succession
        (reference_usage reference 'M'))
      (merge_node 'M'))))
~~~
