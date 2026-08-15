# META
~~~ini
description=SysML Example (Simple Tests): ViewTest
type=file
~~~
# SOURCE
~~~sysml
package ViewTest {
	package P {
		public part p1;
		private part p2;
	}
	
	part def S;
	
	concern def C {
	    subject;
		stakeholder s : S;
	}
	
	concern c : C {
	    subject;
		stakeholder s1;
	}
	
	viewpoint def VP {
		frame c;
	}
	
	rendering def R;
	
	rendering r : R;
	
	view def V {
		viewpoint vp: VP {
			frame concern c1;
			concern c2;
		}
		render rendering r1: R[0..1]; 
		
		view v: V[0..*] {
			expose P::*;
			render r;
			
			rendering r2;
			
			alias vp1 for p1;
			// Note: "expose" imports all.
			alias vp2 for p2;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 14) (end 15 16))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 27 2) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 31 2) (end 31 31))
      )
      (diagnostic
        (severity error)
        (code "missing_body_or_semicolon")
        (source "parser")
        (range (start 33 2) (end 43 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:150602796e98ab955e756693987d3669c877d4c7667d84a2dcf3071d6b5af48f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C"))) (kind concern-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "S")))))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::P::p1"))) (kind part) (membership (kind feature) (visibility public)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::P::p2"))) (kind part) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::V"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::VP"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::VP::c"))) (kind frame) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c"))) (kind concern) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/view_test.md") (path (named (kind package) (name "ViewTest")) (named (kind concern) (name "c")) (anonymous (kind stakeholder) (ordinal 0))))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (stakeholderTarget (reference "s1")))))
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r"))) (kind rendering) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")))))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")))))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (path (named (kind package) (name "ViewTest")) (named (kind concern) (name "c")) (anonymous (kind stakeholder) (ordinal 0))))) (kind stakeholderTarget) (ordinal 0))
      (authored-target "s1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s"))) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c"))) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r"))) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")))
      (subtype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s")))
      (featured-by (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")))
      (type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")) (source direct))
      (supertype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")))
      (subtype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")))
      (subtype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::VP::c")))
      (featured-by (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::VP")))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c")))
      (type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (path (named (kind package) (name "ViewTest")) (named (kind concern) (name "c")) (anonymous (kind stakeholder) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c")))
    )
    (declaration (id (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r")))
      (type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")) (source direct))
      (supertype (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_test.md") (range (start 10 18) (end 10 19)) (probe (position 10 18))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0) (authored-target "S")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::S")))))
    )
  )
  (query (document "memory://snapshot/view_test.md") (range (start 13 13) (end 13 14)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::C")))))
    )
  )
  (query (document "memory://snapshot/view_test.md") (range (start 15 14) (end 15 16)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (path (named (kind package) (name "ViewTest")) (named (kind concern) (name "c")) (anonymous (kind stakeholder) (ordinal 0))))) (kind stakeholderTarget) (ordinal 0) (authored-target "s1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/view_test.md") (range (start 24 15) (end 24 16)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_test.md") (qualified-name "ViewTest::R")))))
    )
  )
)
~~~
