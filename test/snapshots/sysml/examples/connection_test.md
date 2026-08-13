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
    (declaration (id (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection-def) (ordinal 0))))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "d2"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "d2"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end3"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "d3"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::q"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB::b"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::F"))) (kind flow-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::M"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::y"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")) (connectorEnd (reference "d1")) (connectorEnd (reference "d2")) (connectorEnd (reference "d3")) (connectorEnd (reference "d4"))))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d4"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p::x"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::p::x::x1"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0))
      (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0))
      (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end3"))) (kind connectorEnd) (ordinal 0))
      (authored-target "d3")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::B")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0))
      (authored-target "p")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 0))
      (authored-target "d1")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 1))
      (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 2))
      (authored-target "d3")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 3))
      (authored-target "d4")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end3"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end3"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB::b"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 2)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 3)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connection_test.md") (range (start 33 14) (end 33 15)) (probe (position 33 14))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 35 18) (end 35 20)) (probe (position 35 18))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0) (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 42 15) (end 42 17)) (probe (position 42 15))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end2"))) (kind connectorEnd) (ordinal 0) (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 36 18) (end 36 20)) (probe (position 36 18))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::::end3"))) (kind connectorEnd) (ordinal 0) (authored-target "d3")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 59 13) (end 59 14)) (probe (position 59 13))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::B")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 13 13) (end 13 14)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0) (authored-target "p")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 31 18) (end 31 19)) (probe (position 31 18))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::C")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 31 29) (end 31 31)) (probe (position 31 29))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 0) (authored-target "d1")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d1")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 31 33) (end 31 35)) (probe (position 31 33))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 1) (authored-target "d2")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d2")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 31 37) (end 31 39)) (probe (position 31 37))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 2) (authored-target "d3")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d3")))))
  )
  (query (document "memory://snapshot/connection_test.md") (range (start 31 41) (end 31 43)) (probe (position 31 41))
    (reference (id (source (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::bus"))) (kind connectorEnd) (ordinal 3) (authored-target "d4")
      (outcome (status resolved) (target (node (document "memory://snapshot/connection_test.md") (qualified-name "ConnectionTest::d4")))))
  )
)
~~~
