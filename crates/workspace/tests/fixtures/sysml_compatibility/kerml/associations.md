# META
~~~ini
description=KerML Simple Tests: Associations
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    datatype X;
    class Y;
    
	assoc A {
		end x_cross [1..1] feature x : X; 
		end y_cross [1..*] feature y : Y;
	}
	
	assoc B specializes A {
		end x1;
		end [0..*] feature y1 redefines y;
	}
	
	assoc struct C {
		const end [1] feature a;
		const end feature b;
	}
	
	metaclass M;	
	assoc XY {
		end [0..1] feature x : X {
			@M;
		}
		end feature y : Y;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDatatype,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwAssoc,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwStruct,Ident,OpenCurly,
KwConst,KwEnd,OpenSquare,DecimalValue,CloseSquare,KwFeature,Ident,Semicolon,
KwConst,KwEnd,KwFeature,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,Semicolon,
KwAssoc,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenCurly,
At,Ident,Semicolon,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Associations'
    (datatype_def 'X')
    (class_def 'Y')
    (association_def 'A'
      (feature_def end 'x' multiplicity : 'X')
      (feature_def end 'y' multiplicity : 'Y'))
    (association_def 'B' :> 'A'
      (feature_def end 'x1')
      (feature_def end 'y1' multiplicity :>> 'y'))
    (assoc_struct_def 'C'
      (feature_def const end 'a' multiplicity)
      (feature_def const end 'b'))
    (metaclass_def 'M')
    (association_def 'XY'
      (feature_def end 'x' multiplicity : 'X'
        (metadata_feature typed 'M'))
      (feature_def end 'y' : 'Y'))))
~~~
# FORMAT
~~~sysml
package Associations {
    datatype X;
    class Y;

    assoc A {
        end x_cross [1..1] feature x : X;
        end y_cross [1..*] feature y : Y;
    }

    assoc B specializes A {
        end x1;
        end feature y1[0..*] redefines y;
    }

    assoc struct C {
        const end feature a[1];
        const end feature b;
    }

    metaclass M;
    assoc XY {
        end feature x[0..1] : X {
            @M;
        }
        end feature y : Y;
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
    (package 'Associations'
      (datatype_def 'X')
      (class_def 'Y')
      (association_def 'A'
        (feature_def end 'x' : 'Associations::X'[datatype_def]
          (multiplicity_range [1..1]))
        (feature_def end 'y' : 'Associations::Y'[class_def]
          (multiplicity_range [1..*])))
      (association_def 'B' :> 'Associations::A'[association_def]
        (feature_def end 'x1' :>> 'Associations::A::x'[feature_def][implied])
        (feature_def end 'y1' :>> 'Associations::A::y'[feature_def]
          (multiplicity_range [0..*])))
      (assoc_struct_def 'C'
        (feature_def end 'a'
          (multiplicity_range [1]))
        (feature_def end 'b'))
      (metaclass_def 'M')
      (association_def 'XY'
        (feature_def end 'x' : 'Associations::X'[datatype_def]
          (multiplicity_range [0..1])
          (metadata_usage :> 'Associations::M'[metaclass_def]))
        (feature_def end 'y' : 'Associations::Y'[class_def])))))
~~~
