# META
~~~ini
description=SysML Example (Simple Tests): ParameterTest
type=file
~~~
# SOURCE
~~~sysml
package ParameterTest {
	attribute def A {
		attribute x : ScalarValues::String;
		attribute y : A;
	}
	
	attribute a : A;
	
	calc def F { in p : A; in q : ScalarValues::Integer; return :  ScalarValues::Integer; }
	
	attribute f = F(a, 2);
	attribute g = F(q = 1, p = a);
	
	attribute b = new A(y=a, x=""); 
	attribute c = new A("test2");
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,KwReturn,Colon,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,Eq,DecimalValue,Comma,Ident,Eq,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,Comma,Ident,Eq,StringValue,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,Ident,OpenParen,StringValue,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ParameterTest'
    (attribute_def 'A'
      (attribute_usage 'x' : 'ScalarValues::String')
      (attribute_usage 'y' : 'A'))
    (attribute_usage 'a' : 'A')
    (calc_def 'F'
      (default_ref_usage in 'p' : 'A')
      (default_ref_usage in 'q' : 'ScalarValues::Integer')
      (return_member))
    (attribute_usage 'f' value)
    (attribute_usage 'g' value)
    (attribute_usage 'b' value)
    (attribute_usage 'c' value)))
~~~
# FORMAT
~~~sysml
package ParameterTest {
    attribute def A {
        attribute x : ScalarValues::String;
        attribute y : A;
    }

    attribute a : A;

    calc def F {
        in p : A;
        in q : ScalarValues::Integer;
        return :  ScalarValues::Integer;
    }

    attribute f = F(a, 2);
    attribute g = F(q = 1, p = a);

    attribute b = new A(y=a, x="");
    attribute c = new A("test2");
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ParameterTest'
      (attribute_def 'A'
        (attribute_usage composite 'x' : 'ScalarValues::String'[unresolved])
        (attribute_usage composite 'y' : 'ParameterTest::A'[attribute_def]))
      (attribute_usage 'a' : 'ParameterTest::A'[attribute_def])
      (calculation_def 'F'
        (reference_usage in reference 'p' : 'ParameterTest::A'[attribute_def])
        (reference_usage in reference 'q' : 'ScalarValues::Integer'[unresolved])
        (return_parameter_membership
          (feature_def out : 'ScalarValues::Integer'[unresolved])))
      (attribute_usage 'f'
        (feature_value (=)))
      (attribute_usage 'g'
        (feature_value (=)))
      (attribute_usage 'b'
        (feature_value (=)))
      (attribute_usage 'c'
        (feature_value (=))))))
~~~
