# META
~~~ini
description=SysML Validation (02-Parts Interconnection): 2c-Parts Interconnection-Multiple Decompositions
type=file
~~~
# SOURCE
~~~sysml
package '2c-Parts Interconnection-Multiple Decompositions' {
	
	part def A1;
	
	part def B11 {
		port pe;
	}
	part def B12 {
		port pf;
	}
	part def B21 {
		port pg;
	}
	part def B22 {
		port ph;
	}
	
	part def C1 {
		port pa;
		port pb;
	}	
	part def C2 {
		port pc;
	}
	part def C3 {
		port pd;
	}
	part def C4;
	
	part a11: A1 {
	doc
	/*
	 * Decomposition 1 - Subsystems b11, b12
	 */
	
		part b11: B11 {
			part c1: C1;			
			part c2: C2;
			
			connect c1.pa to c2.pc;
			
			port :>> pe = c1.pb {
				doc
				/*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
			}
		}
		
		part b12: B12 {
			part c3: C3;			
			part c4: C4;
			
			port :>> pf = c3.pd;
		}
		
		connect b11.pe to b12.pf;
	}
	
	part a12: A1 {
		doc
		/*
		 * Decomposition 2 - Assemblies b21, b22
		 */
	
		part b21: B21 {
			/*
			 * The c-level entities are already composite parts within
			 * a11, so they cannot also be composite parts within a12.
			 */
			 
			ref c1: C1 = a11.b11.c1;			
			ref c3: C3 = a11.b12.c3;
			
			connect c1.pb to c3.pd;
			
			port :>> pg = c1.pa;
		}
		
		part b22: B22 {
			ref c2: C2 = a11.b11.c2;			
			ref c4: C4 = a11.b12.c4;
			
			port :>> ph = c2.pc;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "2c_parts_interconnection_multiple_decompositions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 11) (end 39 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 20) (end 39 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 11) (end 77 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 20) (end 77 25))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPort,ColonGtGt,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPort,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwRef,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPort,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPort,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''2c-Parts Interconnection-Multiple Decompositions''
    (part_def 'A1')
    (part_def 'B11'
      (port_usage 'pe'))
    (part_def 'B12'
      (port_usage 'pf'))
    (part_def 'B21'
      (port_usage 'pg'))
    (part_def 'B22'
      (port_usage 'ph'))
    (part_def 'C1'
      (port_usage 'pa')
      (port_usage 'pb'))
    (part_def 'C2'
      (port_usage 'pc'))
    (part_def 'C3'
      (port_usage 'pd'))
    (part_def 'C4')
    (part_usage 'a11' : 'A1'
      (documentation)
      (part_usage 'b11' : 'B11'
        (part_usage 'c1' : 'C1')
        (part_usage 'c2' : 'C2')
        (connection_usage
          (connector_end)
          (connector_end))
        (port_usage :>> 'pe' value
          (documentation)))
      (part_usage 'b12' : 'B12'
        (part_usage 'c3' : 'C3')
        (part_usage 'c4' : 'C4')
        (port_usage :>> 'pf' value))
      (connection_usage
        (connector_end)
        (connector_end)))
    (part_usage 'a12' : 'A1'
      (documentation)
      (part_usage 'b21' : 'B21'
        (comment)
        (ref_usage ref 'c1' : 'C1' value)
        (ref_usage ref 'c3' : 'C3' value)
        (connection_usage
          (connector_end)
          (connector_end))
        (port_usage :>> 'pg' value))
      (part_usage 'b22' : 'B22'
        (ref_usage ref 'c2' : 'C2' value)
        (ref_usage ref 'c4' : 'C4' value)
        (port_usage :>> 'ph' value)))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package '2c-Parts Interconnection-Multiple Decompositions' {

    part def A1;

    part def B11 {
        port pe;
    }
    part def B12 {
        port pf;
    }
    part def B21 {
        port pg;
    }
    part def B22 {
        port ph;
    }

    part def C1 {
        port pa;
        port pb;
    }
    part def C2 {
        port pc;
    }
    part def C3 {
        port pd;
    }
    part def C4;

    part a11: A1 {
        doc
        /*
	 * Decomposition 1 - Subsystems b11, b12
	 */

        part b11: B11 {
            part c1: C1;
            part c2: C2;

            connect c1.pa to c2.pc;

            port :>> pe = c1.pb {
                doc
                /*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
            }
        }

        part b12: B12 {
            part c3: C3;
            part c4: C4;

            port :>> pf = c3.pd;
        }

        connect b11.pe to b12.pf;
    }

    part a12: A1 {
        doc
        /*
		 * Decomposition 2 - Assemblies b21, b22
		 */

        part b21: B21 {
            /*
			 * The c-level entities are already composite parts within
			 * a11, so they cannot also be composite parts within a12.
			 */

            ref c1: C1 = a11.b11.c1;
            ref c3: C3 = a11.b12.c3;

            connect c1.pb to c3.pd;

            port :>> pg = c1.pa;
        }

        part b22: B22 {
            ref c2: C2 = a11.b11.c2;
            ref c4: C4 = a11.b12.c4;

            port :>> ph = c2.pc;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "32cd7305b43476190ddad77caaed6e4d2030c82b1e803d8d23a62ad9b005e0fd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (kind "package") (name "2c-Parts Interconnection-Multiple Decompositions") (declared-name "2c-Parts Interconnection-Multiple Decompositions") (range (start (line 0) (character 0)) (end (line 0) (character 1343))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (kind "part def") (name "A1") (declared-name "A1") (range (start (line 2) (character 1)) (end (line 2) (character 13))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (kind "part def") (name "B11") (declared-name "B11") (range (start (line 4) (character 1)) (end (line 4) (character 29))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))) (kind "port") (name "pe") (declared-name "pe") (range (start (line 5) (character 2)) (end (line 5) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (kind "part def") (name "B12") (declared-name "B12") (range (start (line 7) (character 1)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))) (kind "port") (name "pf") (declared-name "pf") (range (start (line 8) (character 2)) (end (line 8) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (kind "part def") (name "B21") (declared-name "B21") (range (start (line 10) (character 1)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))) (kind "port") (name "pg") (declared-name "pg") (range (start (line 11) (character 2)) (end (line 11) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (kind "part def") (name "B22") (declared-name "B22") (range (start (line 13) (character 1)) (end (line 13) (character 29))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))) (kind "port") (name "ph") (declared-name "ph") (range (start (line 14) (character 2)) (end (line 14) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (kind "part def") (name "C1") (declared-name "C1") (range (start (line 17) (character 1)) (end (line 17) (character 39))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pa"))) (kind "port") (name "pa") (declared-name "pa") (range (start (line 18) (character 2)) (end (line 18) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pb"))) (kind "port") (name "pb") (declared-name "pb") (range (start (line 19) (character 2)) (end (line 19) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (kind "part def") (name "C2") (declared-name "C2") (range (start (line 21) (character 1)) (end (line 21) (character 28))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2::pc"))) (kind "port") (name "pc") (declared-name "pc") (range (start (line 22) (character 2)) (end (line 22) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (kind "part def") (name "C3") (declared-name "C3") (range (start (line 24) (character 1)) (end (line 24) (character 28))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3::pd"))) (kind "port") (name "pd") (declared-name "pd") (range (start (line 25) (character 2)) (end (line 25) (character 10))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (kind "part def") (name "C4") (declared-name "C4") (range (start (line 27) (character 1)) (end (line 27) (character 13))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind "part") (name "a11") (declared-name "a11") (range (start (line 29) (character 1)) (end (line 29) (character 548))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (authored (membership (kind Feature)) (relationships (typing (reference "A1") (range (start (line 29) (character 11)) (end (line 29) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::_documentation"))) (kind "documentation") (name "") (range (start (line 29) (character 1)) (end (line 29) (character 548))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind "part") (name "b11") (declared-name "b11") (range (start (line 35) (character 2)) (end (line 35) (character 352))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (authored (membership (kind Feature)) (relationships (typing (reference "B11") (range (start (line 35) (character 12)) (end (line 35) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind "part") (name "c1") (declared-name "c1") (range (start (line 36) (character 3)) (end (line 36) (character 15))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (typing (reference "C1") (range (start (line 36) (character 12)) (end (line 36) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind "part") (name "c2") (declared-name "c2") (range (start (line 37) (character 3)) (end (line 37) (character 15))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (typing (reference "C2") (range (start (line 37) (character 12)) (end (line 37) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind "port") (name "pe") (declared-name "pe") (range (start (line 41) (character 3)) (end (line 41) (character 260))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pe") (range (start (line 41) (character 12)) (end (line 41) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe::_documentation"))) (kind "documentation") (name "") (range (start (line 41) (character 3)) (end (line 41) (character 260))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind "part") (name "b12") (declared-name "b12") (range (start (line 52) (character 2)) (end (line 52) (character 84))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (authored (membership (kind Feature)) (relationships (typing (reference "B12") (range (start (line 52) (character 12)) (end (line 52) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind "part") (name "c3") (declared-name "c3") (range (start (line 53) (character 3)) (end (line 53) (character 15))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (typing (reference "C3") (range (start (line 53) (character 12)) (end (line 53) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind "part") (name "c4") (declared-name "c4") (range (start (line 54) (character 3)) (end (line 54) (character 15))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (typing (reference "C4") (range (start (line 54) (character 12)) (end (line 54) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind "port") (name "pf") (declared-name "pf") (range (start (line 56) (character 3)) (end (line 56) (character 23))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pf") (range (start (line 56) (character 12)) (end (line 56) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind "part") (name "a12") (declared-name "a12") (range (start (line 62) (character 1)) (end (line 62) (character 474))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (authored (membership (kind Feature)) (relationships (typing (reference "A1") (range (start (line 62) (character 11)) (end (line 62) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::_documentation"))) (kind "documentation") (name "") (range (start (line 62) (character 1)) (end (line 62) (character 474))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind "part") (name "b21") (declared-name "b21") (range (start (line 68) (character 2)) (end (line 68) (character 281))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (authored (membership (kind Feature)) (relationships (typing (reference "B21") (range (start (line 68) (character 12)) (end (line 68) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind "ref") (name "c1") (declared-name "c1") (range (start (line 74) (character 3)) (end (line 74) (character 27))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (typing (reference "C1") (range (start (line 74) (character 11)) (end (line 74) (character 13)))) (reference (reference "a11.b11.c1") (range none)))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind "ref") (name "c3") (declared-name "c3") (range (start (line 75) (character 3)) (end (line 75) (character 27))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (typing (reference "C3") (range (start (line 75) (character 11)) (end (line 75) (character 13)))) (reference (reference "a11.b12.c3") (range none)))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind "port") (name "pg") (declared-name "pg") (range (start (line 79) (character 3)) (end (line 79) (character 23))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pg") (range (start (line 79) (character 12)) (end (line 79) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind "part") (name "b22") (declared-name "b22") (range (start (line 82) (character 2)) (end (line 82) (character 108))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (authored (membership (kind Feature)) (relationships (typing (reference "B22") (range (start (line 82) (character 12)) (end (line 82) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind "ref") (name "c2") (declared-name "c2") (range (start (line 83) (character 3)) (end (line 83) (character 27))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (typing (reference "C2") (range (start (line 83) (character 11)) (end (line 83) (character 13)))) (reference (reference "a11.b11.c2") (range none)))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind "ref") (name "c4") (declared-name "c4") (range (start (line 84) (character 3)) (end (line 84) (character 27))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (typing (reference "C4") (range (start (line 84) (character 11)) (end (line 84) (character 13)))) (reference (reference "a11.b12.c4") (range none)))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind "port") (name "ph") (declared-name "ph") (range (start (line 86) (character 3)) (end (line 86) (character 23))) (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ph") (range (start (line 86) (character 12)) (end (line 86) (character 14)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0)) (authored-target "A1") (range (start (line 29) (character 11)) (end (line 29) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionSource) (ordinal 0)) (authored-target "b11::pe") (range (start (line 59) (character 10)) (end (line 59) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionTarget) (ordinal 0)) (authored-target "b12::pf") (range (start (line 59) (character 20)) (end (line 59) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0)) (authored-target "B11") (range (start (line 35) (character 12)) (end (line 35) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind connectionSource) (ordinal 0)) (authored-target "c1::pa") (range (start (line 39) (character 11)) (end (line 39) (character 16))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind connectionTarget) (ordinal 0)) (authored-target "c2::pc") (range (start (line 39) (character 20)) (end (line 39) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C1") (range (start (line 36) (character 12)) (end (line 36) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0)) (authored-target "C2") (range (start (line 37) (character 12)) (end (line 37) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind redefinition) (ordinal 0)) (authored-target "pe") (range (start (line 41) (character 12)) (end (line 41) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0)) (authored-target "B12") (range (start (line 52) (character 12)) (end (line 52) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0)) (authored-target "C3") (range (start (line 53) (character 12)) (end (line 53) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0)) (authored-target "C4") (range (start (line 54) (character 12)) (end (line 54) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind redefinition) (ordinal 0)) (authored-target "pf") (range (start (line 56) (character 12)) (end (line 56) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0)) (authored-target "A1") (range (start (line 62) (character 11)) (end (line 62) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0)) (authored-target "B21") (range (start (line 68) (character 12)) (end (line 68) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind connectionSource) (ordinal 0)) (authored-target "c1::pb") (range (start (line 77) (character 11)) (end (line 77) (character 16))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind connectionTarget) (ordinal 0)) (authored-target "c3::pd") (range (start (line 77) (character 20)) (end (line 77) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C1") (range (start (line 74) (character 11)) (end (line 74) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b11.c1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0)) (authored-target "C3") (range (start (line 75) (character 11)) (end (line 75) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b12.c3") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind redefinition) (ordinal 0)) (authored-target "pg") (range (start (line 79) (character 12)) (end (line 79) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0)) (authored-target "B22") (range (start (line 82) (character 12)) (end (line 82) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0)) (authored-target "C2") (range (start (line 83) (character 11)) (end (line 83) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b11.c2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0)) (authored-target "C4") (range (start (line 84) (character 11)) (end (line 84) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b12.c4") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind redefinition) (ordinal 0)) (authored-target "ph") (range (start (line 86) (character 12)) (end (line 86) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "b11::pe") (target "b12::pf") (source-range (start (line 59) (character 10)) (end (line 59) (character 16))) (target-range (start (line 59) (character 20)) (end (line 59) (character 26)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
