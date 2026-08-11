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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b2ca124abc87af849ba3ac4fa9cc088d2ced70836e5f732d217f8daa17bdffc1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DefaultValueTest"))) (kind "package") (name "DefaultValueTest") (declared-name "DefaultValueTest"))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V"))) (kind "part def") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "DefaultValueTest"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "DefaultValueTest::V"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::V::n"))) (kind "attribute") (name "n") (declared-name "n") (parent (node (document "d0") (qualified-name "DefaultValueTest::V"))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::W"))) (kind "part def") (name "W") (declared-name "W") (parent (node (document "d0") (qualified-name "DefaultValueTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "V")))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "DefaultValueTest::W"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (kind "part") (name "v1") (declared-name "v1") (parent (node (document "d0") (qualified-name "DefaultValueTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "V")))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (kind "attribute") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "m")))))
    (element (id (node (document "d0") (qualified-name "DefaultValueTest::v2"))) (kind "part") (name "v2") (declared-name "v2") (parent (node (document "d0") (qualified-name "DefaultValueTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0)) (authored-target "V") (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::W::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::W::m")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "DefaultValueTest::v1::m"))) (kind redefinition) (ordinal 0)) (authored-target "m") (outcome (status resolved) (target (node (document "d0") (qualified-name "DefaultValueTest::v1::m")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 11) (end 7 12)) (probe (position 7 11))
      (reference
        (source (document "d0") (qualified-name "DefaultValueTest::v1"))
        (kind featureTyping) (ordinal 0) (authored-target "V")
        (range (start 7 11) (end 7 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DefaultValueTest::V") (range (start 2 1) (end 2 64)))
        )
      )
    )
    (query (range (start 8 16) (end 8 17)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "DefaultValueTest::v1::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 8 16) (end 8 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DefaultValueTest::v1::m") (range (start 8 2) (end 8 23)))
        )
      )
    )
    (query (range (start 11 15) (end 11 16)) (probe (position 11 15))
      (reference
        (source (document "d0") (qualified-name "DefaultValueTest::W"))
        (kind specialization) (ordinal 0) (authored-target "V")
        (range (start 11 15) (end 11 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DefaultValueTest::V") (range (start 2 1) (end 2 64)))
        )
      )
    )
    (query (range (start 12 16) (end 12 17)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "DefaultValueTest::W::m"))
        (kind redefinition) (ordinal 0) (authored-target "m")
        (range (start 12 16) (end 12 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DefaultValueTest::W::m") (range (start 12 2) (end 12 30)))
        )
      )
    )
  )
)
~~~
