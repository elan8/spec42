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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_value_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 3 2) (end 3 27))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 4 2) (end 4 19))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 15 1) (end 15 19))
      )
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "437bd5e0fef37f2cc3cbf303c321e00d9282c1053b3dd1bc96e106993633a1d9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DefaultValueTest"))) (kind "package") (name "DefaultValueTest") (declared-name "DefaultValueTest") (range (start (line 0) (character 0)) (end (line 0) (character 218))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V"))) (kind "part def") (name "V") (declared-name "V") (range (start (line 2) (character 1)) (end (line 2) (character 64))) (parent (node (document "d0") (qualified-name "DefaultValueTest"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 3) (character 2)) (end (line 3) (character 27))) (parent (node (document "d0") (qualified-name "DefaultValueTest::V"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V::n"))) (kind "attribute") (name "n") (declared-name "n") (range (start (line 4) (character 2)) (end (line 4) (character 19))) (parent (node (document "d0") (qualified-name "DefaultValueTest::V"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::W"))) (kind "part def") (name "W") (declared-name "W") (range (start (line 11) (character 1)) (end (line 11) (character 52))) (parent (node (document "d0") (qualified-name "DefaultValueTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "V") (range (start (line 11) (character 15)) (end (line 11) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 12) (character 2)) (end (line 12) (character 30))) (parent (node (document "d0") (qualified-name "DefaultValueTest::W"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "m") (range (start (line 12) (character 16)) (end (line 12) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (kind "part") (name "v1") (declared-name "v1") (range (start (line 7) (character 1)) (end (line 7) (character 41))) (parent (node (document "d0") (qualified-name "DefaultValueTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "V") (range (start (line 7) (character 11)) (end (line 7) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 8) (character 2)) (end (line 8) (character 23))) (parent (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "m") (range (start (line 8) (character 16)) (end (line 8) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v2"))) (kind "part") (name "v2") (declared-name "v2") (range (start (line 15) (character 1)) (end (line 15) (character 19))) (parent (node (document "d0") (qualified-name "DefaultValueTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0)) (authored-target "V") (range (start (line 11) (character 15)) (end (line 11) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 12) (character 16)) (end (line 12) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::W::m")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range (start (line 7) (character 11)) (end (line 7) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (range (start (line 8) (character 16)) (end (line 8) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::v1::m")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "DefaultValueTest::W"))) (target (node (document "d0") (qualified-name "DefaultValueTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (target (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (target (node (document "d0") (qualified-name "DefaultValueTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (target (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "DefaultValueTest::V::m")) (expression (status "ok") (value (integer 10))))
    (node (node (document "d0") (qualified-name "DefaultValueTest::V::n")) (expression (status "ok") (value (integer 20))))
    (node (node (document "d0") (qualified-name "DefaultValueTest::W::m")) (expression (status "ok") (value (integer 20))))
    (node (node (document "d0") (qualified-name "DefaultValueTest::v1::m")) (expression (status "ok") (value (integer 20))))
    (node (node (document "d0") (qualified-name "DefaultValueTest::v2")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
