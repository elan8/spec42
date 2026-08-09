# META
~~~ini
description=SysML Example (Simple Tests): FeaturePathTest
type=file
~~~
# SOURCE
~~~sysml
package Q {
  part def F {
  	part a : A;
  }
  
  part f : F;
  
  part def A {
    part g = f.a;
  }
  
  part def B {
  	part f : F;
  	part a : A;
  }
  
  part def C {
	part b : B {
	  connect f.a to a.g;
	  bind f.a = a.g;
	}
  
	part c subsets b.f {
	  	part aa subsets a;
	}
	
	flow b.f.a to c.aa;
  }
  
  part e1 {
  	attribute x : E;
  	// Ensure that "e1" resolves correctly.
  	bind e1.x = E::e2;
  }
  
  enum def E {
  	enum e1;
  	enum e2;
  }
  
  part g = new A().g.g.g;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,Dot,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
LineComment,
KwBind,Ident,Dot,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Q'
    (part_def 'F'
      (part_usage 'a' : 'A'))
    (part_usage 'f' : 'F')
    (part_def 'A'
      (part_usage 'g' value))
    (part_def 'B'
      (part_usage 'f' : 'F')
      (part_usage 'a' : 'A'))
    (part_def 'C'
      (part_usage 'b' : 'B'
        (connection_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end)))
      (part_usage 'c' :> 'b.f'
        (part_usage 'aa' :> 'a'))
      (flow_usage 'b'))
    (part_usage 'e1'
      (attribute_usage 'x' : 'E')
      (line_comment)
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (enum_def 'E'
      (enum_value 'e1')
      (enum_value 'e2'))
    (part_usage 'g' value)))
~~~
# FORMAT
~~~sysml
package Q {
    part def F {
        part a : A;
    }

    part f : F;

    part def A {
        part g = f.a;
    }

    part def B {
        part f : F;
        part a : A;
    }

    part def C {
        part b : B {
            connect f.a to a.g;
            bind f.a = a.g;
        }

        part c subsets b.f {
            part aa subsets a;
        }

        flow b;
    }

    part e1 {
        attribute x : E;
        // Ensure that "e1" resolves correctly.
        bind e1.x = E::e2;
    }

    enum def E {
        enum e1;
        enum e2;
    }

    part g = new A().g.g.g;
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(model
  (namespace
    (package 'Q'
      (part_def 'F'
        (part_usage composite 'a' : 'Q::A'[part_def]))
      (part_usage 'f' : 'Q::F'[part_def])
      (part_def 'A'
        (part_usage composite 'g'
          (feature_value (=))))
      (part_def 'B'
        (part_usage composite 'f' : 'Q::F'[part_def])
        (part_usage composite 'a' : 'Q::A'[part_def]))
      (part_def 'C'
        (part_usage composite 'b' : 'Q::B'[part_def]
          (connection_usage composite
            (connector_end 'f.a')
            (connector_end 'a.g'))
          (binding_connector_def
            (connector_end 'f.a')
            (connector_end 'a.g')))
        (part_usage composite 'c' :> 'Q::B::f'[part_usage]
          (part_usage composite 'aa' :> 'Q::F::a'[part_usage]))
        (flow_usage composite 'b'))
      (part_usage 'e1'
        (attribute_usage composite 'x' : 'Q::E'[enum_def])
        (binding_connector_def
          (connector_end 'e1.x')
          (connector_end 'E::e2')))
      (enum_def 'E'
        (enum_usage composite 'e1')
        (enum_usage composite 'e2'))
      (part_usage 'g'
        (feature_value (=))))))
~~~
