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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Associations"))) (name "Associations") (declared-name "Associations")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::A"))) (name "A") (declared-name "A"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::B"))) (name "B") (declared-name "B"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::M"))) (name "M") (declared-name "M"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::X"))) (name "X") (declared-name "X"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::XY"))) (name "XY") (declared-name "XY"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Associations::Y"))) (name "Y") (declared-name "Y"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Associations::struct"))) (name "struct") (declared-name "struct"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
