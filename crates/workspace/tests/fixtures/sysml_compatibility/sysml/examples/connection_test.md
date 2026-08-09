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
        end [1] a : A {
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
(model
  (namespace
    (package 'ConnectionTest'
      (part_usage 'p'
        (part_usage composite 'x'
          (part_usage composite 'x1')))
      (part_def 'P'
        (part_usage composite 'y')
        (connection_usage composite
          (connector_end 'p')
          (connector_end 'y'))
        (part_usage composite 'p1' :> 'ConnectionTest::p'[part_usage])
        (connection_usage composite
          (connector_end 'p1.x')
          (connector_end 'y'))
        (connection_usage composite
          (connector_end 'p1.x.x1')
          (connector_end 'y')))
      (connection_def abstract 'C'
        (part_usage composite 'p')
        (port_usage end 'end1')
        (port_usage end 'end2')
        (port_usage end 'end3'))
      (part_usage 'd1')
      (part_usage 'd2')
      (part_usage 'd3')
      (part_usage 'd4')
      (connection_usage 'bus' : 'ConnectionTest::C'[connection_def])
      (connection_usage : 'ConnectionTest::C'[connection_def]
        (port_usage end :>> 'ConnectionTest::C::end1'[port_usage] :> 'ConnectionTest::d1'[part_usage])
        (port_usage end 'end2' :> 'ConnectionTest::d2'[part_usage])
        (port_usage end 'end3' :> 'ConnectionTest::d3'[part_usage]))
      (not_implemented 'malformed')
      (flow_def abstract 'F')
      (flow_usage : 'ConnectionTest::F'[flow_def]
        (connector_end 'p')
        (connector_end 'p'))
      (part_def 'A'
        (reference_usage reference 'b' : 'ConnectionTest::B'[part_def]))
      (part_def 'B')
      (connection_def 'AB'
        (port_usage end 'a' : 'ConnectionTest::A'[part_def]
          (multiplicity_range [1])
          (metadata_usage :> 'ConnectionTest::M'[metadata_def]))
        (port_usage end 'b' : 'ConnectionTest::B'[part_def]))
      (metadata_def 'M'))))
~~~
