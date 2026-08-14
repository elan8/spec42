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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 2) (end 21 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 10) (end 21 11))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 21 12) (end 22 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 21 12) (end 22 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 24 30) (end 24 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 2) (end 25 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 10) (end 25 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 2) (end 26 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 8) (end 26 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 10) (end 26 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 14) (end 26 15))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:4c91479d93359c3d2f3ae0a07da52df75ff5acf6a2a9e45359b56454ef729b47") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (kind class-def) (membership (kind owning) (visibility public)) (facts (short-name "1")))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (kind kerml-class) (membership (kind owning) (visibility default)) (facts (short-name "2") (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "package")) (expressionOperand (reference "P"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a"))) (kind kerml-feature) (membership (kind feature) (visibility public)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1"))) (kind kerml-feature) (membership (kind feature) (visibility public)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind kerml-structure) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (specialization (reference "Classes::2")) (expressionOperand (reference "private")) (expressionOperand (reference "y")) (expressionOperand (reference "alias")) (expressionOperand (reference "z")) (expressionOperand (reference "for")) (expressionOperand (reference "y"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C"))))
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (kind expressionOperand) (ordinal 0))
      (authored-target "package")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (kind expressionOperand) (ordinal 1))
      (authored-target "P")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind specialization) (ordinal 0))
      (authored-target "Classes::2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 1))
      (authored-target "y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 2))
      (authored-target "alias")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 3))
      (authored-target "z")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 4))
      (authored-target "for")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 5))
      (authored-target "y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")))))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f"))) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f")))
      (supertype (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/classes.md") (range (start 21 2) (end 21 9)) (probe (position 21 2))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (kind expressionOperand) (ordinal 0) (authored-target "package")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 21 10) (end 21 11)) (probe (position 21 10))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B"))) (kind expressionOperand) (ordinal 1) (authored-target "P")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 11 29) (end 11 30)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 14 31) (end 14 32)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a1"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 12 25) (end 12 26)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::a::aa"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 16 24) (end 16 25)) (probe (position 16 24))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 17 27) (end 17 28)) (probe (position 17 27))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::a::q"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 19 23) (end 19 24)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::B::x::q"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 24 30) (end 24 42)) (probe (position 24 30))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind specialization) (ordinal 0) (authored-target "Classes::2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 25 2) (end 25 9)) (probe (position 25 2))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 25 10) (end 25 11)) (probe (position 25 10))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 1) (authored-target "y")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 26 2) (end 26 7)) (probe (position 26 2))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 2) (authored-target "alias")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 26 8) (end 26 9)) (probe (position 26 8))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 3) (authored-target "z")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 26 10) (end 26 13)) (probe (position 26 10))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 4) (authored-target "for")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 26 14) (end 26 15)) (probe (position 26 14))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C"))) (kind expressionOperand) (ordinal 5) (authored-target "y")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/classes.md") (range (start 27 24) (end 27 25)) (probe (position 27 24))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 28 26) (end 28 27)) (probe (position 28 26))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C::c::cc"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::C")))))
  )
  (query (document "memory://snapshot/classes.md") (range (start 2 12) (end 2 13)) (probe (position 2 12))
    (reference (id (source (node (document "memory://snapshot/classes.md") (qualified-name "Classes::f"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classes.md") (qualified-name "Classes::A")))))
  )
)
~~~
