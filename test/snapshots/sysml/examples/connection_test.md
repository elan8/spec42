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
  (document "connection_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 3) (end 4 11))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 9 2) (end 9 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 10) (end 15 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 10) (end 16 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 20 2) (end 20 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 12))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 14))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 21 2) (end 21 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 1) (end 26 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 27 1) (end 27 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 28 1) (end 28 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 29 1) (end 29 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 40 2) (end 40 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 40 2) (end 40 12))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 41 2) (end 41 30))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 47 1) (end 47 29))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 56 5) (end 56 48))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c814e30487edcc33beb3320d55723f1c18b2293b2626c1b7c727577cda40acf5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConnectionTest"))) (kind "package") (name "ConnectionTest") (declared-name "ConnectionTest"))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::A"))) (kind "part def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind "ref") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "ConnectionTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "B")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::AB"))) (kind "connection def") (name "AB") (declared-name "AB") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind "interface end") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "ConnectionTest::AB"))) (authored (relationships (typing (reference "B")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::B"))) (kind "part def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::C"))) (kind "connection def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::F"))) (kind "flow def") (name "F") (declared-name "F") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::M"))) (kind "metadata def") (name "M") (declared-name "M") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind "part def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind "part") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ConnectionTest::P"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (kind "part") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "ConnectionTest::P"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind "connection") (name "_connection") (declared-name "_connection") (parent (node (document "d0") (qualified-name "ConnectionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind "interface end") (name "end2") (declared-name "end2") (parent (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (authored (relationships (reference-subsetting (reference "d2")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind "interface end") (name "end3") (declared-name "end3") (parent (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (authored (relationships (reference-subsetting (reference "d3")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (kind "connection def") (name "_connectionDef") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind "interface end") (name "end2") (declared-name "end2") (parent (node (document "d0") (qualified-name "ConnectionTest::_connectionDef"))) (authored (relationships (reference-subsetting (reference "d2")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind "connection") (name "bus") (declared-name "bus") (parent (node (document "d0") (qualified-name "ConnectionTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d1"))) (kind "part") (name "d1") (declared-name "d1") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d2"))) (kind "part") (name "d2") (declared-name "d2") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d3"))) (kind "part") (name "d3") (declared-name "d3") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::d4"))) (kind "part") (name "d4") (declared-name "d4") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "ConnectionTest"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p::x"))) (kind "part") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "ConnectionTest::p"))))
    (element (id (node (document "d0") (qualified-name "ConnectionTest::p::x::x1"))) (kind "part") (name "x1") (declared-name "x1") (parent (node (document "d0") (qualified-name "ConnectionTest::p::x"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 0)) (authored-target "d1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 1)) (authored-target "d1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 2)) (authored-target "d1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 0)) (authored-target "d2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 1)) (authored-target "d3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionTarget) (ordinal 2)) (authored-target "d4") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d4")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 0)) (authored-target "p") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 1)) (authored-target "p1::x") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 2)) (authored-target "p1::x::x1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 0)) (authored-target "y") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 1)) (authored-target "y") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionTarget) (ordinal 2)) (authored-target "y") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::P::y")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0)) (authored-target "p") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d3")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "d2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::d2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConnectionTest::C")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (target (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (target (node (document "d0") (qualified-name "ConnectionTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::AB::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (target (node (document "d0") (qualified-name "ConnectionTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::P::p1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (target (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (target (node (document "d0") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connection::end3"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (target (node (document "d0") (qualified-name "ConnectionTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "d1") (target "d2")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "d1") (target "d3")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::d1"))) (target (node (document "d0") (qualified-name "ConnectionTest::d4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest"))) (kind connectionSource) (ordinal 2)) (expression (kind connection) (source "d1") (target "d4")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "ConnectionTest::p"))) (target (node (document "d0") (qualified-name "ConnectionTest::P::y"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConnectionTest::P"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "p") (target "y")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 11 10) (end 11 11)) (probe (position 11 10))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionSource) (ordinal 0) (authored-target "p")
        (range (start 11 10) (end 11 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::p") (range (start 2 1) (end 2 39)))
        )
      )
    )
    (query (range (start 11 15) (end 11 16)) (probe (position 11 15))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionTarget) (ordinal 0) (authored-target "y")
        (range (start 11 15) (end 11 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::P::y") (range (start 9 2) (end 9 9)))
        )
      )
    )
    (query (range (start 13 13) (end 13 14)) (probe (position 13 13))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P::p1"))
        (kind subsetting) (ordinal 0) (authored-target "p")
        (range (start 13 13) (end 13 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::p") (range (start 2 1) (end 2 39)))
        )
      )
    )
    (query (range (start 15 18) (end 15 19)) (probe (position 15 18))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionTarget) (ordinal 1) (authored-target "y")
        (range (start 15 18) (end 15 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::P::y") (range (start 9 2) (end 9 9)))
        )
      )
    )
    (query (range (start 16 21) (end 16 22)) (probe (position 16 21))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionTarget) (ordinal 2) (authored-target "y")
        (range (start 16 21) (end 16 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::P::y") (range (start 9 2) (end 9 9)))
        )
      )
    )
    (query (range (start 50 13) (end 50 14)) (probe (position 50 13))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::A::b"))
        (kind featureTyping) (ordinal 0) (authored-target "B")
        (range (start 50 13) (end 50 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::B") (range (start 53 1) (end 53 12)))
        )
      )
    )
    (query (range (start 31 29) (end 31 31)) (probe (position 31 29))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionSource) (ordinal 0) (authored-target "d1")
        (range (start 31 29) (end 31 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d1") (range (start 26 1) (end 26 9)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionSource) (ordinal 1) (authored-target "d1")
        (range (start 31 29) (end 31 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d1") (range (start 26 1) (end 26 9)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionSource) (ordinal 2) (authored-target "d1")
        (range (start 31 29) (end 31 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d1") (range (start 26 1) (end 26 9)))
        )
      )
    )
    (query (range (start 31 33) (end 31 35)) (probe (position 31 33))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionTarget) (ordinal 0) (authored-target "d2")
        (range (start 31 33) (end 31 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d2") (range (start 27 1) (end 27 9)))
        )
      )
    )
    (query (range (start 31 37) (end 31 39)) (probe (position 31 37))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionTarget) (ordinal 1) (authored-target "d3")
        (range (start 31 37) (end 31 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d3") (range (start 28 1) (end 28 9)))
        )
      )
    )
    (query (range (start 31 41) (end 31 43)) (probe (position 31 41))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest"))
        (kind connectionTarget) (ordinal 2) (authored-target "d4")
        (range (start 31 41) (end 31 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d4") (range (start 29 1) (end 29 9)))
        )
      )
    )
    (query (range (start 35 18) (end 35 20)) (probe (position 35 18))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::_connection::end2"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "d2")
        (range (start 35 18) (end 35 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d2") (range (start 27 1) (end 27 9)))
        )
      )
    )
    (query (range (start 36 18) (end 36 20)) (probe (position 36 18))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::_connection::end3"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "d3")
        (range (start 36 18) (end 36 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d3") (range (start 28 1) (end 28 9)))
        )
      )
    )
    (query (range (start 42 15) (end 42 17)) (probe (position 42 15))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::_connectionDef::end2"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "d2")
        (range (start 42 15) (end 42 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConnectionTest::d2") (range (start 27 1) (end 27 9)))
        )
      )
    )
    (query (range (start 15 10) (end 15 14)) (probe (position 15 10))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionSource) (ordinal 1) (authored-target "p1::x")
        (range (start 15 10) (end 15 14))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 10) (end 16 17)) (probe (position 16 10))
      (reference
        (source (document "d0") (qualified-name "ConnectionTest::P"))
        (kind connectionSource) (ordinal 2) (authored-target "p1::x::x1")
        (range (start 16 10) (end 16 17))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
