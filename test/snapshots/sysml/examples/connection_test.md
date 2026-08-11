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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_test.md"
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 10) (end 15 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 56 5) (end 56 48))
      )
    )
  )
)
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
# EXPECTED
~~~
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c814e30487edcc33beb3320d55723f1c18b2293b2626c1b7c727577cda40acf5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConnectionTest"))) (kind "package") (name "ConnectionTest") (declared-name "ConnectionTest") (range (start (line 0) (character 0)) (end (line 0) (character 726))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 49) (character 1)) (end (line 49) (character 32))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind "ref") (name "b") (declared-name "b") (range (start (line 50) (character 5)) (end (line 50) (character 15))) (parent (node (document "d0") (qualified-name "ConnectionTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 50) (character 13)) (end (line 50) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::AB"))) (kind "connection def") (name "AB") (declared-name "AB") (range (start (line 55) (character 1)) (end (line 55) (character 82))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind "interface end") (name "b") (declared-name "b") (range (start (line 59) (character 5)) (end (line 59) (character 15))) (parent (node (document "d0") (qualified-name "ConnectionTest::AB"))) (authored (relationships (typing (reference "B") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 53) (character 1)) (end (line 53) (character 12))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::C"))) (kind "connection def") (name "C") (declared-name "C") (range (start (line 19) (character 1)) (end (line 19) (character 77))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::F"))) (kind "flow def") (name "F") (declared-name "F") (range (start (line 45) (character 1)) (end (line 45) (character 21))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::M"))) (kind "metadata def") (name "M") (declared-name "M") (range (start (line 62) (character 1)) (end (line 62) (character 16))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind "part def") (name "P") (declared-name "P") (range (start (line 8) (character 1)) (end (line 8) (character 111))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind "part") (name "p1") (declared-name "p1") (range (start (line 13) (character 2)) (end (line 13) (character 15))) (parent (node (document "d0") (qualified-name "ConnectionTest::P"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p") (range (start (line 13) (character 13)) (end (line 13) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 9) (character 2)) (end (line 9) (character 9))) (parent (node (document "d0") (qualified-name "ConnectionTest::P"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind "connection") (name "_connection") (declared-name "_connection") (range (start (line 33) (character 1)) (end (line 33) (character 90))) (parent (node (document "d0") (qualified-name "ConnectionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind "interface end") (name "end2") (declared-name "end2") (range (start (line 35) (character 5)) (end (line 35) (character 21))) (parent (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (authored (relationships (reference-subsetting (reference "d2") (range (start (line 35) (character 18)) (end (line 35) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind "interface end") (name "end3") (declared-name "end3") (range (start (line 36) (character 5)) (end (line 36) (character 21))) (parent (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (authored (relationships (reference-subsetting (reference "d3") (range (start (line 36) (character 18)) (end (line 36) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (kind "connection def") (name "_connectionDef") (range (start (line 39) (character 1)) (end (line 39) (character 73))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind "interface end") (name "end2") (declared-name "end2") (range (start (line 42) (character 2)) (end (line 42) (character 18))) (parent (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (authored (relationships (reference-subsetting (reference "d2") (range (start (line 42) (character 15)) (end (line 42) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind "connection") (name "bus") (declared-name "bus") (range (start (line 31) (character 1)) (end (line 31) (character 45))) (parent (node (document "d0") (qualified-name "ConnectionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d1"))) (kind "part") (name "d1") (declared-name "d1") (range (start (line 26) (character 1)) (end (line 26) (character 9))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d2"))) (kind "part") (name "d2") (declared-name "d2") (range (start (line 27) (character 1)) (end (line 27) (character 9))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d3"))) (kind "part") (name "d3") (declared-name "d3") (range (start (line 28) (character 1)) (end (line 28) (character 9))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d4"))) (kind "part") (name "d4") (declared-name "d4") (range (start (line 29) (character 1)) (end (line 29) (character 9))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 2) (character 1)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 3) (character 2)) (end (line 3) (character 26))) (parent (node (document "d0") (qualified-name "ConnectionTest::p"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p::x::x1"))) (kind "part") (name "x1") (declared-name "x1") (range (start (line 4) (character 3)) (end (line 4) (character 11))) (parent (node (document "d0") (qualified-name "ConnectionTest::p::x"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 0)) (authored-target "d1") (range (start (line 31) (character 29)) (end (line 31) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 1)) (authored-target "d1") (range (start (line 31) (character 29)) (end (line 31) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 2)) (authored-target "d1") (range (start (line 31) (character 29)) (end (line 31) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 0)) (authored-target "d2") (range (start (line 31) (character 33)) (end (line 31) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 1)) (authored-target "d3") (range (start (line 31) (character 37)) (end (line 31) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 2)) (authored-target "d4") (range (start (line 31) (character 41)) (end (line 31) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d4")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 50) (character 13)) (end (line 50) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 0)) (authored-target "p") (range (start (line 11) (character 10)) (end (line 11) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 1)) (authored-target "p1::x") (range (start (line 15) (character 10)) (end (line 15) (character 14))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 2)) (authored-target "p1::x::x1") (range (start (line 16) (character 10)) (end (line 16) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 0)) (authored-target "y") (range (start (line 11) (character 15)) (end (line 11) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 1)) (authored-target "y") (range (start (line 15) (character 18)) (end (line 15) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 2)) (authored-target "y") (range (start (line 16) (character 21)) (end (line 16) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0)) (authored-target "p") (range (start (line 13) (character 13)) (end (line 13) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d2") (range (start (line 35) (character 18)) (end (line 35) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d3") (range (start (line 36) (character 18)) (end (line 36) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d2") (range (start (line 42) (character 15)) (end (line 42) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::C")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (target (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (target (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (target (node (document "d0") (qualified-name "ConnectionTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (target (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (target (node (document "d0") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (target (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "d1") (target "d2") (source-range (start (line 31) (character 29)) (end (line 31) (character 31))) (target-range (start (line 31) (character 33)) (end (line 31) (character 35)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "d1") (target "d3") (source-range (start (line 31) (character 29)) (end (line 31) (character 31))) (target-range (start (line 31) (character 37)) (end (line 31) (character 39)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 2)) (expression (kind connection) (source "d1") (target "d4") (source-range (start (line 31) (character 29)) (end (line 31) (character 31))) (target-range (start (line 31) (character 41)) (end (line 31) (character 43)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::p"))) (target (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "p") (target "y") (source-range (start (line 11) (character 10)) (end (line 11) (character 11))) (target-range (start (line 11) (character 15)) (end (line 11) (character 16)))))
  )
  (evaluation
  )
)
~~~
