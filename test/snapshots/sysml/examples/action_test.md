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
    (element (id (node (document "d0") (qualified-name "ActionTest"))) (kind "package") (name "ActionTest") (declared-name "ActionTest") (range (start (line 0) (character 0)) (end (line 0) (character 809))))
    (element (id (node (document "d0") (qualified-name "ActionTest::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 22))) (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::A::x"))) (kind "in out parameter") (name "x") (declared-name "x") (range (start (line 1) (character 15)) (end (line 1) (character 20))) (parent (node (document "d0") (qualified-name "ActionTest::A"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::S"))) (kind "attribute def") (name "S") (declared-name "S") (range (start (line 11) (character 1)) (end (line 11) (character 17))) (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a"))) (kind "action") (name "a") (declared-name "a") (range (start (line 3) (character 1)) (end (line 3) (character 80))) (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)) (perform (reference "ActionTest::a::b") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1"))) (kind "action") (name "a1") (declared-name "a1") (range (start (line 13) (character 1)) (end (line 13) (character 270))) (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1::_initial"))) (kind "initial") (name "_initial") (range (start (line 14) (character 2)) (end (line 14) (character 14))) (parent (node (document "d0") (qualified-name "ActionTest::a1"))) (authored (relationships (flow (reference "ActionTest::a1::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a1::m"))) (kind "merge") (name "merge") (declared-name "merge") (range (start (line 15) (character 2)) (end (line 15) (character 15))) (parent (node (document "d0") (qualified-name "ActionTest::a1"))) (authored (relationships (flow (reference "ActionTest::a1::decide") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2"))) (kind "action") (name "a2") (declared-name "a2") (range (start (line 27) (character 1)) (end (line 27) (character 214))) (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Feature)) (relationships (perform (reference "ActionTest::a2::aa") (range none)) (perform (reference "ActionTest::a2::snd") (range none)) (perform (reference "ActionTest::a2::snd2") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::aa"))) (kind "action") (name "aa") (declared-name "aa") (range (start (line 29) (character 2)) (end (line 29) (character 37))) (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::aa::target"))) (kind "part") (name "target") (declared-name "target") (range (start (line 30) (character 3)) (end (line 30) (character 19))) (parent (node (document "d0") (qualified-name "ActionTest::a2::aa"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::s"))) (kind "in out parameter") (name "s") (declared-name "s") (range (start (line 28) (character 2)) (end (line 28) (character 11))) (parent (node (document "d0") (qualified-name "ActionTest::a2"))) (authored (relationships (typing (reference "S") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::snd"))) (kind "action") (name "snd") (declared-name "snd") (range (start (line 33) (character 2)) (end (line 33) (character 13))) (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a2::snd2"))) (kind "action") (name "snd2") (declared-name "snd2") (range (start (line 36) (character 2)) (end (line 36) (character 14))) (parent (node (document "d0") (qualified-name "ActionTest::a2"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::_initial"))) (kind "initial") (name "_initial") (range (start (line 4) (character 2)) (end (line 4) (character 14))) (parent (node (document "d0") (qualified-name "ActionTest::a"))) (authored (relationships (flow (reference "ActionTest::a::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::b"))) (kind "action") (name "b") (declared-name "b") (range (start (line 6) (character 2)) (end (line 6) (character 24))) (parent (node (document "d0") (qualified-name "ActionTest::a"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (kind "in out parameter") (name "y") (declared-name "y") (range (start (line 6) (character 13)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "ActionTest::a::b"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b"))) (kind "action") (name "b") (declared-name "b") (range (start (line 40) (character 1)) (end (line 40) (character 73))) (parent (node (document "d0") (qualified-name "ActionTest"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b::a"))) (kind "ref") (name "a") (declared-name "a") (range (start (line 42) (character 2)) (end (line 42) (character 19))) (parent (node (document "d0") (qualified-name "ActionTest::b"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 42) (character 17)) (end (line 42) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "ActionTest::b::f : ScalarValues::Boolean"))) (kind "action body decl") (name "f : ScalarValues::Boolean") (declared-name "f : ScalarValues::Boolean") (range (start (line 41) (character 2)) (end (line 41) (character 38))) (parent (node (document "d0") (qualified-name "ActionTest::b"))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c"))) (kind "action def") (name "c") (declared-name "c") (range (start (line 45) (character 1)) (end (line 45) (character 91))) (parent (node (document "d0") (qualified-name "ActionTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "ActionTest::c::c1") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::_initial"))) (kind "initial") (name "_initial") (range (start (line 46) (character 2)) (end (line 46) (character 14))) (parent (node (document "d0") (qualified-name "ActionTest::c"))) (authored (relationships (flow (reference "ActionTest::c::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind "action") (name "c1") (declared-name "c1") (range (start (line 47) (character 2)) (end (line 47) (character 39))) (parent (node (document "d0") (qualified-name "ActionTest::c"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "ActionTest::c::terminate") (range none)))))
    (element (id (node (document "d0") (qualified-name "ActionTest::c::c1::_terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (range (start (line 48) (character 3)) (end (line 48) (character 16))) (parent (node (document "d0") (qualified-name "ActionTest::c::c1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind bindSource) (ordinal 0)) (authored-target "x") (range (start (line 8) (character 7)) (end (line 8) (character 8))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind bindTarget) (ordinal 0)) (authored-target "b::y") (range (start (line 8) (character 11)) (end (line 8) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a::b::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::a::b") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a::b")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a1::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a1::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a1::m"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a1::decide") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind bindSource) (ordinal 1)) (authored-target "s") (range (start (line 37) (character 7)) (end (line 37) (character 8))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::s")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind bindTarget) (ordinal 1)) (authored-target "snd2::payload") (range (start (line 37) (character 11)) (end (line 37) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind flowSource) (ordinal 0)) (authored-target "aa::target") (range (start (line 32) (character 7)) (end (line 32) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::aa::target")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind flowTarget) (ordinal 0)) (authored-target "snd::receiver") (range (start (line 32) (character 20)) (end (line 32) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::a2::aa") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::aa")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 1)) (authored-target "ActionTest::a2::snd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::snd")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2"))) (kind performSource) (ordinal 2)) (authored-target "ActionTest::a2::snd2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::a2::snd2")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a2::s"))) (kind featureTyping) (ordinal 0)) (authored-target "S") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::S")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::a::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::a::b::y"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::b::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 42) (character 17)) (end (line 42) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c"))) (kind performSource) (ordinal 0)) (authored-target "ActionTest::c::c1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ActionTest::c::c1")))))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::c::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ActionTest::c::c1"))) (kind flowSource) (ordinal 0)) (authored-target "ActionTest::c::terminate") (range none) (outcome (status unresolved)))
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
