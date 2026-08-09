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

    part a11 : A1 {
        doc /*
	 * Decomposition 1 - Subsystems b11, b12
	 */

        part b11 : B11 {
            part c1 : C1;
            part c2 : C2;

            connect c1.pa to c2.pc;

            port :>> pe = c1.pb {
                doc /*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
            }
        }

        part b12 : B12 {
            part c3 : C3;
            part c4 : C4;

            port :>> pf = c3.pd;
        }

        connect b11.pe to b12.pf;
    }

    part a12 : A1 {
        doc /*
		 * Decomposition 2 - Assemblies b21, b22
		 */

        part b21 : B21 {
            /*
			 * The c-level entities are already composite parts within
			 * a11, so they cannot also be composite parts within a12.
			 */

            ref c1 : C1 = a11.b11.c1;
            ref c3 : C3 = a11.b12.c3;

            connect c1.pb to c3.pd;

            port :>> pg = c1.pa;
        }

        part b22 : B22 {
            ref c2 : C2 = a11.b11.c2;
            ref c4 : C4 = a11.b12.c4;

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
(model
  (namespace
    (package '2c-Parts Interconnection-Multiple Decompositions'
      (part_def 'A1')
      (part_def 'B11'
        (port_usage composite 'pe'))
      (part_def 'B12'
        (port_usage composite 'pf'))
      (part_def 'B21'
        (port_usage composite 'pg'))
      (part_def 'B22'
        (port_usage composite 'ph'))
      (part_def 'C1'
        (port_usage composite 'pa')
        (port_usage composite 'pb'))
      (part_def 'C2'
        (port_usage composite 'pc'))
      (part_def 'C3'
        (port_usage composite 'pd'))
      (part_def 'C4')
      (part_usage 'a11' : '2c-Parts Interconnection-Multiple Decompositions::A1'[part_def]
        (documentation)
        (part_usage composite 'b11' : '2c-Parts Interconnection-Multiple Decompositions::B11'[part_def]
          (part_usage composite 'c1' : '2c-Parts Interconnection-Multiple Decompositions::C1'[part_def])
          (part_usage composite 'c2' : '2c-Parts Interconnection-Multiple Decompositions::C2'[part_def])
          (connection_usage composite
            (connector_end 'c1.pa')
            (connector_end 'c2.pc'))
          (port_usage composite :>> '2c-Parts Interconnection-Multiple Decompositions::B11::pe'[port_usage]
            (feature_value (=))
            (documentation)))
        (part_usage composite 'b12' : '2c-Parts Interconnection-Multiple Decompositions::B12'[part_def]
          (part_usage composite 'c3' : '2c-Parts Interconnection-Multiple Decompositions::C3'[part_def])
          (part_usage composite 'c4' : '2c-Parts Interconnection-Multiple Decompositions::C4'[part_def])
          (port_usage composite :>> '2c-Parts Interconnection-Multiple Decompositions::B12::pf'[port_usage]
            (feature_value (=))))
        (connection_usage composite
          (connector_end 'b11.pe')
          (connector_end 'b12.pf')))
      (part_usage 'a12' : '2c-Parts Interconnection-Multiple Decompositions::A1'[part_def]
        (documentation)
        (part_usage composite 'b21' : '2c-Parts Interconnection-Multiple Decompositions::B21'[part_def]
          (reference_usage reference 'c1' : '2c-Parts Interconnection-Multiple Decompositions::C1'[part_def]
            (feature_value (=)))
          (reference_usage reference 'c3' : '2c-Parts Interconnection-Multiple Decompositions::C3'[part_def]
            (feature_value (=)))
          (connection_usage composite
            (connector_end 'c1.pb')
            (connector_end 'c3.pd'))
          (port_usage composite :>> '2c-Parts Interconnection-Multiple Decompositions::B21::pg'[port_usage]
            (feature_value (=))))
        (part_usage composite 'b22' : '2c-Parts Interconnection-Multiple Decompositions::B22'[part_def]
          (reference_usage reference 'c2' : '2c-Parts Interconnection-Multiple Decompositions::C2'[part_def]
            (feature_value (=)))
          (reference_usage reference 'c4' : '2c-Parts Interconnection-Multiple Decompositions::C4'[part_def]
            (feature_value (=)))
          (port_usage composite :>> '2c-Parts Interconnection-Multiple Decompositions::B22::ph'[port_usage]
            (feature_value (=))))))))
~~~
