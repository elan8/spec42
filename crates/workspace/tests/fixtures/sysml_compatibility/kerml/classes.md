# META
~~~ini
description=KerML Simple Tests: Classes
type=file
~~~
# SOURCE
~~~kerml
package Classes {
	
	feature f: A;

	public class <'1'> A { 
		feature b: B;
		protected in c: C;
		portion feature p : A;
	}
	
	abstract class <'2'> B {
		public abstract feature a: A {
			composite feature aa: A;
		}
		public composite feature a1: A;
		feature x {
			composite feature a: A {
			    portion feature q : A;
			}
			portion feature q : A;
		}
		package P { }
	}
	
	private struct C specializes Classes::'2' {
		private y: A, '2'[0..*];
		alias z for y;
		composite feature c : C {
			composite feature cc : C;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwPublic,KwClass,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwProtected,KwIn,Ident,Colon,Ident,Semicolon,
KwPortion,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwClass,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwPublic,KwAbstract,KwFeature,Ident,Colon,Ident,OpenCurly,
KwComposite,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPublic,KwComposite,KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,OpenCurly,
KwComposite,KwFeature,Ident,Colon,Ident,OpenCurly,
KwPortion,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,CloseCurly,
CloseCurly,
KwPrivate,KwStruct,Ident,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwPrivate,Ident,Colon,Ident,Comma,UnrestrictedName,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwComposite,KwFeature,Ident,Colon,Ident,OpenCurly,
KwComposite,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Classes'
    (feature_def 'f' : 'A')
    (class_def public 'A'
      (feature_def 'b' : 'B')
      (feature_def protected in 'c' : 'C')
      (feature_def portion 'p' : 'A'))
    (class_def abstract 'B'
      (feature_def public abstract 'a' : 'A'
        (feature_def composite 'aa' : 'A'))
      (feature_def public composite 'a1' : 'A')
      (feature_def 'x'
        (feature_def composite 'a' : 'A'
          (feature_def portion 'q' : 'A'))
        (feature_def portion 'q' : 'A'))
      (package_def 'P'))
    (structure_def private 'C' :> 'Classes::'2''
      (feature_def private 'y' : 'A', ''2'' multiplicity)
      (alias_member 'z' for 'y')
      (feature_def composite 'c' : 'C'
        (feature_def composite 'cc' : 'C')))))
~~~
# FORMAT
~~~sysml
package Classes {
	
	feature f: A;

	public class <'1'> A { 
		feature b: B;
		protected in c: C;
		portion feature p : A;
	}
	
	abstract class <'2'> B {
		public abstract feature a: A {
			composite feature aa: A;
		}
		public composite feature a1: A;
		feature x {
			composite feature a: A {
			    portion feature q : A;
			}
			portion feature q : A;
		}
		package P { }
	}
	
	private struct C specializes Classes::'2' {
		private y: A, '2'[0..*];
		alias z for y;
		composite feature c : C {
			composite feature cc : C;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Classes"))) (name "Classes") (declared-name "Classes")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Classes::1"))) (name "1") (declared-name "1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Classes::2"))) (name "2") (declared-name "2"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Classes::C"))) (name "C") (declared-name "C"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Classes::f"))) (name "f") (declared-name "f"))
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
