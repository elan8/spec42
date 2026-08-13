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
  (document "memory://snapshot/classes.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 14))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 2) (end 6 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 6 2) (end 7 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 2) (end 8 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 24 1) (end 30 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 30 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:4c91479d93359c3d2f3ae0a07da52df75ff5acf6a2a9e45359b56454ef729b47") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (kind class-def) (membership (kind owning) (visibility public)))
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
