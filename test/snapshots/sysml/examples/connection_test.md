# META
~~~ini
description=SysML Example (Simple Tests): ConnectionTest
type=file
~~~
# SOURCE
~~~sysml
package ConnectionTest {
	
	part p {
		part x {
			part x1;
		}
	}
	
	part def P {
		part y;

		connect p to y;
		
		part p1 :> p;
	
		connect p1.x to y;
		connect p1.x.x1 to y;
	}

	abstract connection def C {
		part p;
		end end1;
		end end2;
		end end3;
	}
	
	part d1;
	part d2;
	part d3;
	part d4;
	
	connection bus : C connect (d1, d2, d3, d4);
	
	connection : C {
	    end :>> end1 ::> d1;
	    end end2 ::> d2;
	    end end3 ::> d3;
	}
	
	connection {
		part q;
		end ref end1 ::> d1 :> q;
		end end2 ::> d2;
	}
	
	abstract flow def F;
	
	message : F from p to p;
	
	part def A {
	    ref b : B;
	}
	
	part def B;
	
	connection def AB {
	    end [1] item a : A {
	    	@M;
	    }
	    end b : B;
	}
	
	metadata def M;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwConnect,Ident,KwTo,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConnection,KwDef,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwEnd,Ident,Semicolon,
KwEnd,Ident,Semicolon,
KwEnd,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwConnection,Ident,Colon,Ident,KwConnect,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwConnection,Colon,Ident,OpenCurly,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
KwConnection,OpenCurly,
KwPart,Ident,Semicolon,
KwEnd,KwRef,Ident,ColonColonGt,Ident,ColonGt,Ident,Semicolon,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFlow,KwDef,Ident,Semicolon,
KwMessage,Colon,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenCurly,
At,Ident,Semicolon,
CloseCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ConnectionTest'
    (part_usage 'p'
      (part_usage 'x'
        (part_usage 'x1')))
    (part_def 'P'
      (part_usage 'y')
      (connection_usage
        (connector_end)
        (connector_end))
      (part_usage 'p1' :> 'p')
      (connection_usage
        (connector_end)
        (connector_end))
      (connection_usage
        (connector_end)
        (connector_end)))
    (connection_def abstract 'C'
      (part_usage 'p')
      (interface_end end 'end1')
      (interface_end end 'end2')
      (interface_end end 'end3'))
    (part_usage 'd1')
    (part_usage 'd2')
    (part_usage 'd3')
    (part_usage 'd4')
    (connection_usage 'C' 'bus')
    (connection_usage 'C'
      (interface_end end :>> 'end1' references 'd1')
      (interface_end end 'end2' references 'd2')
      (interface_end end 'end3' references 'd3'))
    (malformed)
    (flow_def abstract 'F')
    (message_usage 'F'
      (connector_end)
      (connector_end))
    (part_def 'A'
      (ref_usage ref 'b' : 'B'))
    (part_def 'B')
    (connection_def 'AB'
      (interface_end end 'a' : 'A' multiplicity
        (metadata_feature typed 'M'))
      (interface_end end 'b' : 'B'))
    (metadata_def 'M')))
~~~
# FORMAT
~~~sysml
package ConnectionTest {

    part p {
        part x {
            part x1;
        }
    }

    part def P {
        part y;

        connect p to y;

        part p1 :> p;

        connect p1.x to y;
        connect p1.x.x1 to y;
    }

    abstract connection def C {
        part p;
        end end1;
        end end2;
        end end3;
    }

    part d1;
    part d2;
    part d3;
    part d4;

    connection bus : C connect (d1, d2, d3, d4);

    connection : C {
        end :>> end1 ::> d1;
        end end2 ::> d2;
        end end3 ::> d3;
    }

    connection {
        part q;
        end ref end1 ::> d1 :> q;
        end end2 ::> d2;
    }

    abstract flow def F;

    message : F from p to p;

    part def A {
        ref b : B;
    }

    part def B;

    connection def AB {
        end [1] item a : A {
            @M;
        }
        end b : B;
    }

    metadata def M;
}

~~~
# EXPECTED
~~~
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ConnectionTest"))) (name "ConnectionTest") (declared-name "ConnectionTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConnectionTest::A"))) (name "A") (declared-name "A") (declared)
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (name "b") (declared-name "b") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "ConnectionTest::A")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ConnectionTest::AB"))) (name "AB") (declared-name "AB")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (name "b") (declared-name "b") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConnectionTest::AB")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConnectionTest::B"))) (name "B") (declared-name "B") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ConnectionTest::C"))) (name "C") (declared-name "C"))
        (element (kind "flow def") (id (node (document "d0") (qualified-name "ConnectionTest::F"))) (name "F") (declared-name "F"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ConnectionTest::M"))) (name "M") (declared-name "M"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConnectionTest::P"))) (name "P") (declared-name "P") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (name "p1") (declared-name "p1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConnectionTest::P")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (name "y") (declared-name "y") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConnectionTest::P")))))
          )
        )
        (element (kind "connection") (id (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (name "_connection") (declared-name "_connection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (name "end2") (declared-name "end2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConnectionTest::C")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (name "end3") (declared-name "end3") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConnectionTest::C")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (name "_connectionDef")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (name "end2") (declared-name "end2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConnectionTest::_connectionDef")))))
          )
        )
        (element (kind "connection") (id (node (document "d0") (qualified-name "ConnectionTest::bus"))) (name "bus") (declared-name "bus"))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::d1"))) (name "d1") (declared-name "d1") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::d2"))) (name "d2") (declared-name "d2") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::d3"))) (name "d3") (declared-name "d3") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::d4"))) (name "d4") (declared-name "d4") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::p"))) (name "p") (declared-name "p") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::p::x"))) (name "x") (declared-name "x") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "ConnectionTest::p::x::x1"))) (name "x1") (declared-name "x1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::d1"))) (to (node (document "d0") (qualified-name "ConnectionTest::d2"))) (connect (source-expression "d1") (target-expression "d2") (container-prefix "ConnectionTest")) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::d1"))) (to (node (document "d0") (qualified-name "ConnectionTest::d3"))) (connect (source-expression "d1") (target-expression "d3") (container-prefix "ConnectionTest")) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::d1"))) (to (node (document "d0") (qualified-name "ConnectionTest::d4"))) (connect (source-expression "d1") (target-expression "d4") (container-prefix "ConnectionTest")) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::d2"))) (to (node (document "d0") (qualified-name "ConnectionTest::d3"))) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::p"))) (to (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (connect (source-expression "p") (target-expression "y") (container-prefix "ConnectionTest::P")) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (to (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (to (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (to (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConnectionTest::bus"))) (to (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "p1::x") (target-expression "y") (container-prefix "ConnectionTest::P"))
    (connection (status pending-expression) (document "d0") (source-expression "p1::x::x1") (target-expression "y") (container-prefix "ConnectionTest::P"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::A"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::AB"))) (status missing-prerequisite) (target "Connections::Connection"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::B"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::C"))) (status missing-prerequisite) (target "Connections::Connection"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::F"))) (status missing-prerequisite) (target "Flows::MessageAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::M"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::P"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (status missing-prerequisite) (target "Connections::connections"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (status missing-prerequisite) (target "Connections::Connection"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::bus"))) (status missing-prerequisite) (target "Connections::connections"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::d1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::d2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::d3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::d4"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::p"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::p::x"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConnectionTest::p::x::x1"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/connection_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 3) (end 4 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 9 2) (end 9 9))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 15 10) (end 15 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 15 10) (end 15 14))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 16 10) (end 16 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 16 10) (end 16 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 20 2) (end 20 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 12))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 14))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 21 2) (end 21 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 1) (end 26 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 27 1) (end 27 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 28 1) (end 28 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 29 1) (end 29 9))
      )
      (diagnostic
        (severity warning)
        (code "incomplete_connection_like_end_pair")
        (source "semantic")
        (range (start 39 1) (end 39 73))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 40 2) (end 40 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 40 2) (end 40 12))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 41 2) (end 41 30))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 47 1) (end 47 29))
      )
      (diagnostic
        (severity warning)
        (code "incomplete_connection_like_end_pair")
        (source "semantic")
        (range (start 55 1) (end 55 82))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 56 5) (end 56 48))
      )
    )
  )
)
~~~
