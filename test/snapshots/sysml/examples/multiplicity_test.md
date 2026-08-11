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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 1) (end 3 41))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 8 1) (end 8 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 1) (end 10 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 1) (end 11 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 12 1) (end 12 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 37))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "59a2167d72bc5bd96dcbbfb5b3d2d23f50bbe005786fafdf3e49513d8cd98ff5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MultiplicityTest"))) (kind "package") (name "MultiplicityTest") (declared-name "MultiplicityTest") (range (start (line 0) (character 0)) (end (line 0) (character 276))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A"))) (kind "attribute def") (name "A") (declared-name "A") (range (start (line 14) (character 1)) (end (line 14) (character 81))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A::i"))) (kind "attribute") (name "i") (declared-name "i") (range (start (line 15) (character 2)) (end (line 15) (character 37))) (parent (node (document "d0") (qualified-name "MultiplicityTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 16) (character 2)) (end (line 16) (character 21))) (parent (node (document "d0") (qualified-name "MultiplicityTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::P"))) (kind "part def") (name "P") (declared-name "P") (range (start (line 2) (character 1)) (end (line 2) (character 12))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 5) (character 1)) (end (line 5) (character 11))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 6) (character 1)) (end (line 6) (character 18))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range (start (line 6) (character 16)) (end (line 6) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 7) (character 1)) (end (line 7) (character 18))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range (start (line 7) (character 10)) (end (line 7) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::d"))) (kind "part") (name "d") (declared-name "d") (range (start (line 8) (character 1)) (end (line 8) (character 11))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::e"))) (kind "part") (name "e") (declared-name "e") (range (start (line 10) (character 1)) (end (line 10) (character 11))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::f"))) (kind "part") (name "f") (declared-name "f") (range (start (line 11) (character 1)) (end (line 11) (character 14))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::g"))) (kind "part") (name "g") (declared-name "g") (range (start (line 12) (character 1)) (end (line 12) (character 14))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))))
    (element (id (node (document "d0") (qualified-name "MultiplicityTest::n"))) (kind "attribute def") (name "n") (declared-name "n") (range (start (line 3) (character 1)) (end (line 3) (character 41))) (parent (node (document "d0") (qualified-name "MultiplicityTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "Integer") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::A::i"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range (start (line 6) (character 16)) (end (line 6) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range (start (line 7) (character 10)) (end (line 7) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MultiplicityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "MultiplicityTest::n"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (target (node (document "d0") (qualified-name "MultiplicityTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::A::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (target (node (document "d0") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (target (node (document "d0") (qualified-name "MultiplicityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MultiplicityTest::c"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MultiplicityTest::n")) (expression (status "ok") (value (integer 5))))
  )
)
~~~
