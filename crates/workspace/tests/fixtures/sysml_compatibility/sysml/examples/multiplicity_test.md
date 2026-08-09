# META
~~~ini
description=SysML Example (Simple Tests): MultiplicityTest
type=file
~~~
# SOURCE
~~~sysml
package MultiplicityTest {
	
	part def P;
	attribute n : ScalarValues::Integer = 5;
	
	part a[1];
	part b[0..2] : P;
	part c : P[2..*];
	part d[*];
	
	part e[n];
	part f[n..*];
	part g[1..n];

	attribute def A {
		attribute i :ScalarValues::Integer;
		attribute x : A[i];
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,Ident,DotDot,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,DotDot,Ident,CloseSquare,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MultiplicityTest'
    (part_def 'P')
    (attribute_usage 'n' : 'ScalarValues::Integer' value)
    (part_usage 'a' multiplicity)
    (part_usage 'b' : 'P' multiplicity)
    (part_usage 'c' : 'P' multiplicity)
    (part_usage 'd' multiplicity)
    (part_usage 'e' multiplicity)
    (part_usage 'f' multiplicity)
    (part_usage 'g' multiplicity)
    (attribute_def 'A'
      (attribute_usage 'i' : 'ScalarValues::Integer')
      (attribute_usage 'x' : 'A' multiplicity))))
~~~
# FORMAT
~~~sysml
package MultiplicityTest {
    part def P;
    attribute n : ScalarValues::Integer = 5;

    part a [1];
    part b : P [0..2];
    part c : P [2..*];
    part d [*];

    part e [n];
    part f [n..*];
    part g [1..n];

    attribute def A {
        attribute i : ScalarValues::Integer;
        attribute x : A [i];
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package 'MultiplicityTest'
      (part_def 'P')
      (attribute_usage 'n' : 'ScalarValues::Integer'[unresolved]
        (feature_value (=)))
      (part_usage 'a'
        (multiplicity_range [1]))
      (part_usage 'b' : 'MultiplicityTest::P'[part_def]
        (multiplicity_range [0..2]))
      (part_usage 'c' : 'MultiplicityTest::P'[part_def]
        (multiplicity_range [2..*]))
      (part_usage 'd'
        (multiplicity_range [*]))
      (part_usage 'e'
        (multiplicity_range [?]))
      (part_usage 'f'
        (multiplicity_range [?..*]))
      (part_usage 'g'
        (multiplicity_range [1..?]))
      (attribute_def 'A'
        (attribute_usage composite 'i' : 'ScalarValues::Integer'[unresolved])
        (attribute_usage composite 'x' : 'MultiplicityTest::A'[attribute_def]
          (multiplicity_range [?]))))))
~~~
