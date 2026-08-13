# META
~~~ini
description=SysML Validation (02-Parts Interconnection): 2c-Parts Interconnection-Multiple Decompositions
type=file
~~~
# SOURCE
~~~sysml
package '2c-Parts Interconnection-Multiple Decompositions' {
	
	part def A1;
	
	part def B11 {
		port pe;
	}
	part def B12 {
		port pf;
	}
	part def B21 {
		port pg;
	}
	part def B22 {
		port ph;
	}
	
	part def C1 {
		port pa;
		port pb;
	}	
	part def C2 {
		port pc;
	}
	part def C3 {
		port pd;
	}
	part def C4;
	
	part a11: A1 {
	doc
	/*
	 * Decomposition 1 - Subsystems b11, b12
	 */
	
		part b11: B11 {
			part c1: C1;			
			part c2: C2;
			
			connect c1.pa to c2.pc;
			
			port :>> pe = c1.pb {
				doc
				/*
				 * This combines the definition of a port with a binding
				 * connector. (It is the same notation used to bind a
				 * attribute to a attribute property or a reference to a reference
				 * property.)
				 */
			}
		}
		
		part b12: B12 {
			part c3: C3;			
			part c4: C4;
			
			port :>> pf = c3.pd;
		}
		
		connect b11.pe to b12.pf;
	}
	
	part a12: A1 {
		doc
		/*
		 * Decomposition 2 - Assemblies b21, b22
		 */
	
		part b21: B21 {
			/*
			 * The c-level entities are already composite parts within
			 * a11, so they cannot also be composite parts within a12.
			 */
			 
			ref c1: C1 = a11.b11.c1;			
			ref c3: C3 = a11.b12.c3;
			
			connect c1.pb to c3.pd;
			
			port :>> pg = c1.pa;
		}
		
		part b22: B22 {
			ref c2: C2 = a11.b11.c2;			
			ref c4: C4 = a11.b12.c4;
			
			port :>> ph = c2.pc;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 39 3) (end 39 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 59 2) (end 59 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 77 3) (end 77 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1f7031fbd9e1a916ab691756215b3fe735fec169b7c22bd0604861a618819785") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pa"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pb"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2::pc"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3::pd"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A1"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B11"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "pe"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B12"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "pf"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C3"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C4"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A1"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B21"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "pg"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C1"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C3"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B22"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "ph"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C2"))))
    (declaration (id (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C4"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0))
      (authored-target "A1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0))
      (authored-target "B11")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "pe")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0))
      (authored-target "B12")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "pf")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0))
      (authored-target "C3")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0))
      (authored-target "C4")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0))
      (authored-target "A1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0))
      (authored-target "B21")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "pg")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0))
      (authored-target "C3")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0))
      (authored-target "B22")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "ph")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0))
      (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0))
      (authored-target "C4")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 29 11) (end 29 13)) (probe (position 29 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0) (authored-target "A1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 35 12) (end 35 15)) (probe (position 35 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0) (authored-target "B11")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 41 12) (end 41 14)) (probe (position 41 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "pe")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 36 12) (end 36 14)) (probe (position 36 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 37 12) (end 37 14)) (probe (position 37 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 52 12) (end 52 15)) (probe (position 52 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0) (authored-target "B12")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 56 12) (end 56 14)) (probe (position 56 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "pf")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 53 12) (end 53 14)) (probe (position 53 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0) (authored-target "C3")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 54 12) (end 54 14)) (probe (position 54 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0) (authored-target "C4")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 62 11) (end 62 13)) (probe (position 62 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0) (authored-target "A1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 68 12) (end 68 15)) (probe (position 68 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0) (authored-target "B21")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 79 12) (end 79 14)) (probe (position 79 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "pg")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 74 11) (end 74 13)) (probe (position 74 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0) (authored-target "C1")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 75 11) (end 75 13)) (probe (position 75 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0) (authored-target "C3")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 82 12) (end 82 15)) (probe (position 82 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0) (authored-target "B22")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 86 12) (end 86 14)) (probe (position 86 12))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "ph")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 83 11) (end 83 13)) (probe (position 83 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0) (authored-target "C2")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
  )
  (query (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (range (start 84 11) (end 84 13)) (probe (position 84 11))
    (reference (id (source (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0) (authored-target "C4")
      (outcome (status resolved) (target (node (document "memory://snapshot/2c_parts_interconnection_multiple_decompositions.md") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
  )
)
~~~
