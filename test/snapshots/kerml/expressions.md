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
  (document "expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
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
        (range (start 3 16) (end 3 32))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 5 1) (end 5 1670))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "034d42e5bbc2c856ffe13b65b5885161b7603ac93cbba1ef4b2b77bc81e49fdb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Expressions"))) (kind "package") (name "Expressions") (declared-name "Expressions") (range (start (line 0) (character 0)) (end (line 0) (character 1809))))
    (element (id (node (document "d0") (qualified-name "Expressions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Expressions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Expressions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 36))) (parent (node (document "d0") (qualified-name "Expressions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Expressions::ToString"))) (kind "import") (name "ToString") (declared-name "ToString") (range (start (line 2) (character 1)) (end (line 2) (character 40))) (parent (node (document "d0") (qualified-name "Expressions"))) (authored (membership (kind Import) (visibility "private") (import (reference "BaseFunctions::ToString") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Expressions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarFunctions::*") (range (start (line 1) (character 16)) (end (line 1) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Expressions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Expressions::ToString"))) (kind membershipImport) (ordinal 0)) (authored-target "BaseFunctions::ToString") (range (start (line 2) (character 16)) (end (line 2) (character 39))) (outcome (status unresolved)))
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
  (document "d0"
    (query (range (start 1 16) (end 1 31)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Expressions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions::*")
        (range (start 1 16) (end 1 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 32)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Expressions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 3 16) (end 3 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 39)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Expressions::ToString"))
        (kind membershipImport) (ordinal 0) (authored-target "BaseFunctions::ToString")
        (range (start 2 16) (end 2 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
