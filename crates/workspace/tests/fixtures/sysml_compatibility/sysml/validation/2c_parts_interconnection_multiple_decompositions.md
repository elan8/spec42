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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (name "2c-Parts Interconnection-Multiple Decompositions") (declared-name "2c-Parts Interconnection-Multiple Decompositions")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (name "A1") (declared-name "A1") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (name "B11") (declared-name "B11") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))) (name "pe") (declared-name "pe") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (name "B12") (declared-name "B12") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))) (name "pf") (declared-name "pf") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (name "B21") (declared-name "B21") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))) (name "pg") (declared-name "pg") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (name "B22") (declared-name "B22") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))) (name "ph") (declared-name "ph") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (name "C1") (declared-name "C1") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pa"))) (name "pa") (declared-name "pa") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pb"))) (name "pb") (declared-name "pb") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (name "C2") (declared-name "C2") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2::pc"))) (name "pc") (declared-name "pc") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (name "C3") (declared-name "C3") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3::pd"))) (name "pd") (declared-name "pd") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (name "C4") (declared-name "C4") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (name "a11") (declared-name "a11") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (name "b11") (declared-name "b11") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (name "c1") (declared-name "c1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (name "c2") (declared-name "c2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (name "pe") (declared-name "pe") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (name "b12") (declared-name "b12") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (name "c3") (declared-name "c3") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (name "c4") (declared-name "c4") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (name "pf") (declared-name "pf") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (name "a12") (declared-name "a12") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (name "b21") (declared-name "b21") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (name "c1") (declared-name "c1") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "c1") (children (expression (kind "memberAccess") (reference "b11") (children (expression (kind "featureReference") (reference "a11")))))))) (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (role feature-value))))
                (element (kind "ref") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (name "c3") (declared-name "c3") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "c3") (children (expression (kind "memberAccess") (reference "b12") (children (expression (kind "featureReference") (reference "a11")))))))) (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (role feature-value))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (name "pg") (declared-name "pg") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (name "b22") (declared-name "b22") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (name "c2") (declared-name "c2") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "c2") (children (expression (kind "memberAccess") (reference "b11") (children (expression (kind "featureReference") (reference "a11")))))))) (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (role feature-value))))
                (element (kind "ref") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (name "c4") (declared-name "c4") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "c4") (children (expression (kind "memberAccess") (reference "b12") (children (expression (kind "featureReference") (reference "a11")))))))) (effective (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (role feature-value))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (name "ph") (declared-name "ph") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::_documentation"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe::_documentation"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::_documentation"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pa"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2::pc"))) (connect (source-expression "c1::pa") (target-expression "c2::pc") (container-prefix "2c-Parts Interconnection-Multiple Decompositions::a11::b11")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pb"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3::pd"))) (connect (source-expression "c1::pb") (target-expression "c3::pd") (container-prefix "2c-Parts Interconnection-Multiple Decompositions::a12::b21")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (connect (source-expression "b11::pe") (target-expression "b12::pf") (container-prefix "2c-Parts Interconnection-Multiple Decompositions::a11")))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))))
    (reference (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))))
    (reference (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))))
    (reference (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))))
    (reference (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (to (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/2c_parts_interconnection_multiple_decompositions.md"
    (diagnostics
    )
  )
)
~~~
