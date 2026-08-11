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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classes.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8a2d88a449dd53a1097534fa3636ffeef7488ed1f20a745e3741de74999bc582") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Classes"))) (kind "package") (name "Classes") (declared-name "Classes"))
    (element (id (node (document "d0") (qualified-name "Classes::1"))) (kind "classifier decl") (name "1") (declared-name "1") (parent (node (document "d0") (qualified-name "Classes"))))
    (element (id (node (document "d0") (qualified-name "Classes::2"))) (kind "classifier decl") (name "2") (declared-name "2") (parent (node (document "d0") (qualified-name "Classes"))))
    (element (id (node (document "d0") (qualified-name "Classes::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Classes"))))
    (element (id (node (document "d0") (qualified-name "Classes::f"))) (kind "feature decl") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "Classes"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
