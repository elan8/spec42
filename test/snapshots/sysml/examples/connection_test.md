# META
~~~ini
description=SysML Example (Simple Tests): ConnectionTest
type=file
~~~
# SOURCE
~~~sysml
package ConnectionTest {
	
	part p {
		part x {
			part x1;
		}
	}
	
	part def P {
		part y;

		connect p to y;
		
		part p1 :> p;
	
		connect p1.x to y;
		connect p1.x.x1 to y;
	}

	abstract connection def C {
		part p;
		end end1;
		end end2;
		end end3;
	}
	
	part d1;
	part d2;
	part d3;
	part d4;
	
	connection bus : C connect (d1, d2, d3, d4);
	
	connection : C {
	    end :>> end1 ::> d1;
	    end end2 ::> d2;
	    end end3 ::> d3;
	}
	
	connection {
		part q;
		end ref end1 ::> d1 :> q;
		end end2 ::> d2;
	}
	
	abstract flow def F;
	
	message : F from p to p;
	
	part def A {
	    ref b : B;
	}
	
	part def B;
	
	connection def AB {
	    end [1] item a : A {
	    	@M;
	    }
	    end b : B;
	}
	
	metadata def M;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connection_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 2) (end 11 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 13 13) (end 13 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 1) (end 24 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 21 2) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 21 2) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 31 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 37 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 43 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 45 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 47 1) (end 47 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 1) (end 47 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 50 5) (end 50 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 55 1) (end 60 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 56 5) (end 59 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8ffd9e9a3b8219cd1cdbb709c567a116f46bf279bc129d37a5672d4f87e7219d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::M"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::y"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d4"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p::x"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p::x::x1"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0))
      (authored-target "p")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/connection_test.md") (range (start 13 13) (end 13 14)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0) (authored-target "p")
      (outcome (status unsupported)))
  )
)
~~~
