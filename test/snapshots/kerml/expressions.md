# META
~~~ini
description=KerML Simple Tests: Expressions
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
	private import ScalarFunctions::*;
	private import BaseFunctions::ToString;
	private import ControlFunctions::*;
	
	a: Integer;
	aa : Boolean;
	x = ToString(a * a + 3 == 4);
	y = NumericalFunctions::'+'(1,2);
	z : Boolean = aa & true xor zz | false implies z;
	zz : Boolean = aa and true xor aa or false implies z;
	grp = -x + x * y * y + a ** 3 ^ 4;
	
	b = if x > y? x-y else y-x;
	c = x->collect {in xx; xx + 1}; 
	c1 = x.{in xx; xx + 1}; 
	d = x->select {in xx; xx != null};
	d1 = x.?{in xx; xx != null};
	e = x->reduce {in s; in t; s + t}->reduce '+';
	
	behavior w { inout v : Integer;
	    step : ControlPerformances::LoopPerformance {
    		in expr whileTest {v > 3}
    		in step body {
    			step decrement {
    				out v_decr : Integer = v - 1;			
    			}
    			succession decrement then update;
    			step update : FeatureReferencingPerformances::FeatureWritePerformance {
    				in onOccurrence = w::self {
    					feature redefines startingAt : w {
    						inout feature redefines accessedFeature redefines v;
    					}
    				}
    				inout replacementValues = decrement.v_decr;
    			}
    		}
		}
	}
	
	xx = if x == 1 and y == 2? a
	     else if x == 2? b
	     else if x == 3? c
	     else 0;
    
    function TotalMass { in partMass; in subparts;
		partMass + (subparts->collect {in p; totalMass(partMass, subparts)}->reduce '+' ?? 0.0)
	}
	
	expr totalMass: TotalMass { in mass; in sub; }
	
	feature f {
		expr s { in x; return : Boolean; }
	}
	
	bb : Boolean = f.s(1);
	
	class C {
		var count : ScalarValues::Integer := 0;
	}
	
	feature obj1 : C;
	feature obj2 : C;
	
	test1 = obj1 === obj2;
	test2 = x !== obj2;
	
	class L {
		feature c : C[*];
		feature count : ScalarValues::Integer =  c#(1).count;
	}
	
	feature l = new L();
	feature w1 = w(xx);
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 35))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 1) (end 51 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 55 1) (end 57 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 58 2) (end 59 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 9) (end 65 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 68 2) (end 69 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 69 2) (end 70 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 14) (end 73 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 16) (end 73 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:bfa7aa826e271b51c2ad9c25d5aaecec56931d371ebe5d7832d7022f67d44072") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "BaseFunctions::ToString") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::L"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::f"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (invocationCallee (reference "L")))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "obj1")) (expressionOperand (reference "obj2")))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")) (expressionOperand (reference "obj2")))))
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "xx")) (invocationCallee (reference "w")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (kind invocationCallee) (ordinal 0))
      (authored-target "L")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::L")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 0))
      (authored-target "obj1")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 1))
      (authored-target "obj2")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind expressionOperand) (ordinal 1))
      (authored-target "obj2")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2")))))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (kind expressionOperand) (ordinal 0))
      (authored-target "xx")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (kind invocationCallee) (ordinal 0))
      (authored-target "w")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::L"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind expressionOperand) (ordinal 1)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1")))
      (supertype (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2")))
      (supertype (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/expressions.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 3 16) (end 3 35)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 2 16) (end 2 39)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (path (named (kind package) (name "Expressions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 72 17) (end 72 18)) (probe (position 72 17))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::l"))) (kind invocationCallee) (ordinal 0) (authored-target "L")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::L")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 61 16) (end 61 17)) (probe (position 61 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 62 16) (end 62 17)) (probe (position 62 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::C")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 64 9) (end 64 13)) (probe (position 64 9))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 0) (authored-target "obj1")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj1")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 64 18) (end 64 22)) (probe (position 64 18))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test1"))) (kind expressionOperand) (ordinal 1) (authored-target "obj2")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 65 9) (end 65 10)) (probe (position 65 9))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 65 15) (end 65 19)) (probe (position 65 15))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::test2"))) (kind expressionOperand) (ordinal 1) (authored-target "obj2")
      (outcome (status resolved) (target (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::obj2")))))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 73 16) (end 73 18)) (probe (position 73 16))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (kind expressionOperand) (ordinal 0) (authored-target "xx")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expressions.md") (range (start 73 14) (end 73 15)) (probe (position 73 14))
    (reference (id (source (node (document "memory://snapshot/expressions.md") (qualified-name "Expressions::w1"))) (kind invocationCallee) (ordinal 0) (authored-target "w")
      (outcome (status unresolved)))
    )
  )
)
~~~
