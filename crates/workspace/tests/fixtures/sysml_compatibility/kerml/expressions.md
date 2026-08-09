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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Eq,Ident,OpenParen,Ident,Star,Ident,Plus,DecimalValue,EqEq,DecimalValue,CloseParen,Semicolon,
Ident,Eq,Ident,ColonColon,UnrestrictedName,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
Ident,Colon,Ident,Eq,Ident,Ampersand,KwTrue,KwXor,Ident,Pipe,KwFalse,KwImplies,Ident,Semicolon,
Ident,Colon,Ident,Eq,Ident,KwAnd,KwTrue,KwXor,Ident,KwOr,KwFalse,KwImplies,Ident,Semicolon,
Ident,Eq,Minus,Ident,Plus,Ident,Star,Ident,Star,Ident,Plus,Ident,StarStar,DecimalValue,Caret,DecimalValue,Semicolon,
Ident,Eq,KwIf,Ident,CloseAngle,Ident,Question,Ident,Minus,Ident,KwElse,Ident,Minus,Ident,Semicolon,
Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Plus,DecimalValue,CloseCurly,Semicolon,
Ident,Eq,Ident,Dot,OpenCurly,KwIn,Ident,Semicolon,Ident,Plus,DecimalValue,CloseCurly,Semicolon,
Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,BangEq,KwNull,CloseCurly,Semicolon,
Ident,Eq,Ident,DotQuestion,OpenCurly,KwIn,Ident,Semicolon,Ident,BangEq,KwNull,CloseCurly,Semicolon,
Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,Ident,Plus,Ident,CloseCurly,Arrow,Ident,UnrestrictedName,Semicolon,
KwBehavior,Ident,OpenCurly,KwInout,Ident,Colon,Ident,Semicolon,
KwStep,Colon,Ident,ColonColon,Ident,OpenCurly,
KwIn,KwExpr,Ident,OpenCurly,Ident,CloseAngle,DecimalValue,CloseCurly,
KwIn,KwStep,Ident,OpenCurly,
KwStep,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Eq,Ident,Minus,DecimalValue,Semicolon,
CloseCurly,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwStep,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwInout,KwFeature,KwRedefines,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInout,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
Ident,Eq,KwIf,Ident,EqEq,DecimalValue,KwAnd,Ident,EqEq,DecimalValue,Question,Ident,
KwElse,KwIf,Ident,EqEq,DecimalValue,Question,Ident,
KwElse,KwIf,Ident,EqEq,DecimalValue,Question,Ident,
KwElse,DecimalValue,Semicolon,
KwFunction,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
Ident,Plus,OpenParen,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseCurly,Arrow,Ident,UnrestrictedName,QuestionQuestion,DecimalValue,Dot,DecimalValue,CloseParen,
CloseCurly,
KwExpr,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,CloseCurly,
KwFeature,Ident,OpenCurly,
KwExpr,Ident,OpenCurly,KwIn,Ident,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
CloseCurly,
Ident,Colon,Ident,Eq,Ident,Dot,Ident,OpenParen,DecimalValue,CloseParen,Semicolon,
KwClass,Ident,OpenCurly,
KwVar,Ident,Colon,Ident,ColonColon,Ident,ColonEq,DecimalValue,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
Ident,Eq,Ident,EqEqEq,Ident,Semicolon,
Ident,Eq,Ident,BangEqEq,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Semicolon,
KwFeature,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Expressions'
    (import_decl private 'ScalarFunctions::*')
    (import_decl private 'BaseFunctions::ToString')
    (import_decl private 'ControlFunctions::*')
    (feature_def 'a' : 'Integer')
    (feature_def 'aa' : 'Boolean')
    (feature_def 'x' value)
    (feature_def 'y' value)
    (feature_def 'z' : 'Boolean' value)
    (feature_def 'zz' : 'Boolean' value)
    (feature_def 'grp' value)
    (feature_def 'b' value)
    (feature_def 'c' value)
    (feature_def 'c1' value)
    (feature_def 'd' value)
    (feature_def 'd1' value)
    (feature_def 'e' value)
    (behavior_def
      (feature_def inout 'v' : 'Integer')
      (step_def
        (expression_usage
          (result_expr_member))
        (step_def
          (step_def
            (feature_def out 'v_decr' : 'Integer' value))
          (succession_def
            (connector_end)
            (connector_end))
          (step_def
            (feature_def in 'onOccurrence' value
              (feature_def :>> 'startingAt' : 'w'
                (feature_def inout :>> 'accessedFeature' :>> 'v')))
            (feature_def inout 'replacementValues' value)))))
    (feature_def 'xx' value)
    (function_def
      (feature_def in 'partMass')
      (feature_def in 'subparts')
      (result_expr_member))
    (expression_def
      (feature_def in 'mass')
      (feature_def in 'sub'))
    (feature_def 'f'
      (expression_def
        (feature_def in 'x')
        (return_member)))
    (feature_def 'bb' : 'Boolean' value)
    (class_def 'C'
      (feature_def var 'count' : 'ScalarValues::Integer' value))
    (feature_def 'obj1' : 'C')
    (feature_def 'obj2' : 'C')
    (feature_def 'test1' value)
    (feature_def 'test2' value)
    (class_def 'L'
      (feature_def 'c' : 'C' multiplicity)
      (feature_def 'count' : 'ScalarValues::Integer' value))
    (feature_def 'l' value)
    (feature_def 'w1' value)))
~~~
# FORMAT
~~~sysml
package Expressions {
    private import ScalarFunctions::*;
    private import BaseFunctions::ToString;
    private import ControlFunctions::*;

    a: Integer;
    aa: Boolean;
    x = ToString(a * a + 3 == 4);
    y = NumericalFunctions::'+'(1,2);
    z: Boolean = aa & true xor zz | false implies z;
    zz: Boolean = aa and true xor aa or false implies z;
    grp = -x + x * y * y + a ** 3 ^ 4;

    b = if x > y? x-y else y-x;
    c = x->collect {in xx; xx + 1};
    c1 = x.{in xx; xx + 1};
    d = x->select {in xx; xx != null};
    d1 = x.?{in xx; xx != null};
    e = x->reduce {in s; in t; s + t}->reduce '+';

    behavior w {
        inout v: Integer;
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

    bb: Boolean = f.s(1);

    class C {
        var count: ScalarValues::Integer := 0;
    }

    feature obj1 : C;
    feature obj2 : C;

    test1 = obj1 === obj2;
    test2 = x !== obj2;

    class L {
        feature c : C [*];
        feature count : ScalarValues::Integer = c#(1).count;
    }

    feature l = new L();
    feature w1 = w(xx);
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ControlPerformances::LoopPerformance'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'FeatureReferencingPerformances::FeatureWritePerformance'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ControlPerformances::LoopPerformance'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'FeatureReferencingPerformances::FeatureWritePerformance'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Expressions'
      (namespace_import private -> 'ScalarFunctions'[unresolved])
      (membership_import private -> 'BaseFunctions::ToString'[unresolved])
      (namespace_import private -> 'ControlFunctions'[unresolved])
      (feature_def 'a' : 'Integer'[unresolved])
      (feature_def 'aa' : 'Boolean'[unresolved])
      (feature_def 'x'
        (feature_value (=)))
      (feature_def 'y'
        (feature_value (=)))
      (feature_def 'z' : 'Boolean'[unresolved]
        (feature_value (=)))
      (feature_def 'zz' : 'Boolean'[unresolved]
        (feature_value (=)))
      (feature_def 'grp'
        (feature_value (=)))
      (feature_def 'b'
        (feature_value (=)))
      (feature_def 'c'
        (feature_value (=)))
      (feature_def 'c1'
        (feature_value (=)))
      (feature_def 'd'
        (feature_value (=)))
      (feature_def 'd1'
        (feature_value (=)))
      (feature_def 'e'
        (feature_value (=)))
      (behavior_def 'w'
        (feature_def inout 'v' : 'Integer'[unresolved])
        (step_def : 'ControlPerformances::LoopPerformance'[unresolved]
          (expression_usage in 'whileTest'
            (result_expr_membership))
          (step_def in 'body'
            (step_def 'decrement'
              (feature_def out 'v_decr' : 'Integer'[unresolved]
                (feature_value (=))))
            (succession_def
              (connector_end 'decrement')
              (connector_end 'update'))
            (step_def 'update' : 'FeatureReferencingPerformances::FeatureWritePerformance'[unresolved]
              (feature_def in 'onOccurrence'
                (feature_value (=))
                (feature_def :>> 'startingAt'[unresolved] : 'Expressions::w'[behavior_def]
                  (feature_def inout :>> 'accessedFeature'[unresolved] :>> 'Expressions::w::v'[feature_def])))
              (feature_def inout 'replacementValues'
                (feature_value (=)))))))
      (feature_def 'xx'
        (feature_value (=)))
      (function_def 'TotalMass'
        (feature_def in 'partMass')
        (feature_def in 'subparts')
        (result_expr_membership))
      (expression_def 'totalMass' : 'Expressions::TotalMass'[function_def]
        (feature_def in 'mass')
        (feature_def in 'sub'))
      (feature_def 'f'
        (expression_def 's'
          (feature_def in 'x')
          (return_parameter_membership
            (feature_def out : 'Boolean'[unresolved]))))
      (feature_def 'bb' : 'Boolean'[unresolved]
        (feature_value (=)))
      (class_def 'C'
        (feature_def 'count' : 'ScalarValues::Integer'[unresolved]
          (feature_value (:=))))
      (feature_def 'obj1' : 'Expressions::C'[class_def])
      (feature_def 'obj2' : 'Expressions::C'[class_def])
      (feature_def 'test1'
        (feature_value (=)))
      (feature_def 'test2'
        (feature_value (=)))
      (class_def 'L'
        (feature_def 'c' : 'Expressions::C'[class_def]
          (multiplicity_range [*]))
        (feature_def 'count' : 'ScalarValues::Integer'[unresolved]
          (feature_value (=))))
      (feature_def 'l'
        (feature_value (=)))
      (feature_def 'w1'
        (feature_value (=))))))
~~~
