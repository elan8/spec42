# META
~~~ini
description=SysML Example (Simple Tests): DefaultValueTest
type=file
~~~
# SOURCE
~~~sysml
package DefaultValueTest {
	
	part def V {
		attribute m default = 10;
		attribute n = 20;
	}
	
	part v1 : V {
		attribute :>> m = 20;
	}
	
	part def W :> V {
		attribute :>> m default = n;
	}
	
	part v2 = new W();
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,KwDefault,Eq,DecimalValue,Semicolon,
KwAttribute,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,KwDefault,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'DefaultValueTest'
    (part_def 'V'
      (attribute_usage 'm' value)
      (attribute_usage 'n' value))
    (part_usage 'v1' : 'V'
      (attribute_usage :>> 'm' value))
    (part_def 'W' :> 'V'
      (attribute_usage :>> 'm' value))
    (part_usage 'v2' value)))
~~~
# FORMAT
~~~sysml
package DefaultValueTest {

    part def V {
        attribute m default = 10;
        attribute n = 20;
    }

    part v1 : V {
        attribute :>> m = 20;
    }

    part def W :> V {
        attribute :>> m default = n;
    }

    part v2 = new W();

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
    (element (kind "package") (id (node (document "d0") (qualified-name "DefaultValueTest"))) (name "DefaultValueTest") (declared-name "DefaultValueTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "DefaultValueTest::V"))) (name "V") (declared-name "V") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "DefaultValueTest::V::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "integerLiteral") (literal 10)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "DefaultValueTest::V")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "DefaultValueTest::V::n"))) (name "n") (declared-name "n") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 20)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "DefaultValueTest::V"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "DefaultValueTest::V::n"))) (role feature-value))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "DefaultValueTest::W"))) (name "W") (declared-name "W") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "featureReference") (reference "n")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "DefaultValueTest::W")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (name "v1") (declared-name "v1") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 20)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "DefaultValueTest::V"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (role feature-value))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "DefaultValueTest::v2"))) (name "v2") (declared-name "v2") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "constructor") (reference "W")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "DefaultValueTest::v2"))) (role feature-value))))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (to (node (document "d0") (qualified-name "DefaultValueTest::V::m"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (to (node (document "d0") (qualified-name "DefaultValueTest::V::m"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "DefaultValueTest::W"))) (to (node (document "d0") (qualified-name "DefaultValueTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (to (node (document "d0") (qualified-name "DefaultValueTest::V"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/default_value_test.md"
    (diagnostics
    )
  )
)
~~~
