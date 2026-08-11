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
  (document "2c_parts_interconnection_multiple_decompositions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 11) (end 39 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 20) (end 39 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 11) (end 77 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 20) (end 77 25))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c4a328d07c3688e6cc666795d191ceb46b7af4ea9e5f0bea2401ddd57285ef13") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (kind "package") (name "2c-Parts Interconnection-Multiple Decompositions") (declared-name "2c-Parts Interconnection-Multiple Decompositions"))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (kind "part def") (name "A1") (declared-name "A1") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (kind "part def") (name "B11") (declared-name "B11") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11::pe"))) (kind "port") (name "pe") (declared-name "pe") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (kind "part def") (name "B12") (declared-name "B12") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12::pf"))) (kind "port") (name "pf") (declared-name "pf") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (kind "part def") (name "B21") (declared-name "B21") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21::pg"))) (kind "port") (name "pg") (declared-name "pg") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (kind "part def") (name "B22") (declared-name "B22") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22::ph"))) (kind "port") (name "ph") (declared-name "ph") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (kind "part def") (name "C1") (declared-name "C1") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pa"))) (kind "port") (name "pa") (declared-name "pa") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1::pb"))) (kind "port") (name "pb") (declared-name "pb") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (kind "part def") (name "C2") (declared-name "C2") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2::pc"))) (kind "port") (name "pc") (declared-name "pc") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (kind "part def") (name "C3") (declared-name "C3") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3::pd"))) (kind "port") (name "pd") (declared-name "pd") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (kind "part def") (name "C4") (declared-name "C4") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind "part") (name "a11") (declared-name "a11") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (authored (membership (kind Feature)) (relationships (typing (reference "A1")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind "part") (name "b11") (declared-name "b11") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (authored (membership (kind Feature)) (relationships (typing (reference "B11")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind "part") (name "c1") (declared-name "c1") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (typing (reference "C1")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind "part") (name "c2") (declared-name "c2") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (typing (reference "C2")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind "port") (name "pe") (declared-name "pe") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pe")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind "part") (name "b12") (declared-name "b12") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (authored (membership (kind Feature)) (relationships (typing (reference "B12")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind "part") (name "c3") (declared-name "c3") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (typing (reference "C3")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind "part") (name "c4") (declared-name "c4") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (typing (reference "C4")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind "port") (name "pf") (declared-name "pf") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pf")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind "part") (name "a12") (declared-name "a12") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions"))) (authored (membership (kind Feature)) (relationships (typing (reference "A1")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind "part") (name "b21") (declared-name "b21") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (authored (membership (kind Feature)) (relationships (typing (reference "B21")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind "ref") (name "c1") (declared-name "c1") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (typing (reference "C1")) (reference (reference "a11.b11.c1")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind "ref") (name "c3") (declared-name "c3") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (typing (reference "C3")) (reference (reference "a11.b12.c3")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind "port") (name "pg") (declared-name "pg") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "pg")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind "part") (name "b22") (declared-name "b22") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (authored (membership (kind Feature)) (relationships (typing (reference "B22")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind "ref") (name "c2") (declared-name "c2") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (typing (reference "C2")) (reference (reference "a11.b11.c2")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind "ref") (name "c4") (declared-name "c4") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (typing (reference "C4")) (reference (reference "a11.b12.c4")))))
    (element (id (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind "port") (name "ph") (declared-name "ph") (parent (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ph")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0)) (authored-target "A1") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionSource) (ordinal 0)) (authored-target "b11::pe") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionTarget) (ordinal 0)) (authored-target "b12::pf") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0)) (authored-target "B11") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind connectionSource) (ordinal 0)) (authored-target "c1::pa") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind connectionTarget) (ordinal 0)) (authored-target "c2::pc") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C1") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0)) (authored-target "C2") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind redefinition) (ordinal 0)) (authored-target "pe") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0)) (authored-target "B12") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0)) (authored-target "C3") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0)) (authored-target "C4") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind redefinition) (ordinal 0)) (authored-target "pf") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0)) (authored-target "A1") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0)) (authored-target "B21") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind connectionSource) (ordinal 0)) (authored-target "c1::pb") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind connectionTarget) (ordinal 0)) (authored-target "c3::pd") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C1") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b11.c1") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0)) (authored-target "C3") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b12.c3") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind redefinition) (ordinal 0)) (authored-target "pg") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0)) (authored-target "B22") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0)) (authored-target "C2") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b11.c2") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0)) (authored-target "C4") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind referenceSource) (ordinal 0)) (authored-target "a11.b12.c4") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4")))))
    (reference (id (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind redefinition) (ordinal 0)) (authored-target "ph") (outcome (status resolved) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "b11::pe") (target "b12::pf")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind reference) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))) (kind referenceSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (target (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 29 11) (end 29 13)) (probe (position 29 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))
        (kind featureTyping) (ordinal 0) (authored-target "A1")
        (range (start 29 11) (end 29 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1") (range (start 2 1) (end 2 13)))
        )
      )
    )
    (query (range (start 36 12) (end 36 14)) (probe (position 36 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c1"))
        (kind featureTyping) (ordinal 0) (authored-target "C1")
        (range (start 36 12) (end 36 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1") (range (start 17 1) (end 17 39)))
        )
      )
    )
    (query (range (start 37 12) (end 37 14)) (probe (position 37 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::c2"))
        (kind featureTyping) (ordinal 0) (authored-target "C2")
        (range (start 37 12) (end 37 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2") (range (start 21 1) (end 21 28)))
        )
      )
    )
    (query (range (start 41 12) (end 41 14)) (probe (position 41 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe"))
        (kind redefinition) (ordinal 0) (authored-target "pe")
        (range (start 41 12) (end 41 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe") (range (start 41 3) (end 41 260)))
        )
      )
    )
    (query (range (start 53 12) (end 53 14)) (probe (position 53 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c3"))
        (kind featureTyping) (ordinal 0) (authored-target "C3")
        (range (start 53 12) (end 53 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3") (range (start 24 1) (end 24 28)))
        )
      )
    )
    (query (range (start 54 12) (end 54 14)) (probe (position 54 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::c4"))
        (kind featureTyping) (ordinal 0) (authored-target "C4")
        (range (start 54 12) (end 54 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4") (range (start 27 1) (end 27 13)))
        )
      )
    )
    (query (range (start 56 12) (end 56 14)) (probe (position 56 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf"))
        (kind redefinition) (ordinal 0) (authored-target "pf")
        (range (start 56 12) (end 56 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf") (range (start 56 3) (end 56 23)))
        )
      )
    )
    (query (range (start 62 11) (end 62 13)) (probe (position 62 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12"))
        (kind featureTyping) (ordinal 0) (authored-target "A1")
        (range (start 62 11) (end 62 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::A1") (range (start 2 1) (end 2 13)))
        )
      )
    )
    (query (range (start 74 11) (end 74 13)) (probe (position 74 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c1"))
        (kind featureTyping) (ordinal 0) (authored-target "C1")
        (range (start 74 11) (end 74 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C1") (range (start 17 1) (end 17 39)))
        )
      )
    )
    (query (range (start 75 11) (end 75 13)) (probe (position 75 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::c3"))
        (kind featureTyping) (ordinal 0) (authored-target "C3")
        (range (start 75 11) (end 75 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C3") (range (start 24 1) (end 24 28)))
        )
      )
    )
    (query (range (start 79 12) (end 79 14)) (probe (position 79 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg"))
        (kind redefinition) (ordinal 0) (authored-target "pg")
        (range (start 79 12) (end 79 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21::pg") (range (start 79 3) (end 79 23)))
        )
      )
    )
    (query (range (start 83 11) (end 83 13)) (probe (position 83 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c2"))
        (kind featureTyping) (ordinal 0) (authored-target "C2")
        (range (start 83 11) (end 83 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C2") (range (start 21 1) (end 21 28)))
        )
      )
    )
    (query (range (start 84 11) (end 84 13)) (probe (position 84 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::c4"))
        (kind featureTyping) (ordinal 0) (authored-target "C4")
        (range (start 84 11) (end 84 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::C4") (range (start 27 1) (end 27 13)))
        )
      )
    )
    (query (range (start 86 12) (end 86 14)) (probe (position 86 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph"))
        (kind redefinition) (ordinal 0) (authored-target "ph")
        (range (start 86 12) (end 86 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22::ph") (range (start 86 3) (end 86 23)))
        )
      )
    )
    (query (range (start 35 12) (end 35 15)) (probe (position 35 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))
        (kind featureTyping) (ordinal 0) (authored-target "B11")
        (range (start 35 12) (end 35 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B11") (range (start 4 1) (end 4 29)))
        )
      )
    )
    (query (range (start 52 12) (end 52 15)) (probe (position 52 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12"))
        (kind featureTyping) (ordinal 0) (authored-target "B12")
        (range (start 52 12) (end 52 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B12") (range (start 7 1) (end 7 29)))
        )
      )
    )
    (query (range (start 68 12) (end 68 15)) (probe (position 68 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))
        (kind featureTyping) (ordinal 0) (authored-target "B21")
        (range (start 68 12) (end 68 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B21") (range (start 10 1) (end 10 29)))
        )
      )
    )
    (query (range (start 82 12) (end 82 15)) (probe (position 82 12))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b22"))
        (kind featureTyping) (ordinal 0) (authored-target "B22")
        (range (start 82 12) (end 82 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::B22") (range (start 13 1) (end 13 29)))
        )
      )
    )
    (query (range (start 39 11) (end 39 16)) (probe (position 39 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))
        (kind connectionSource) (ordinal 0) (authored-target "c1::pa")
        (range (start 39 11) (end 39 16))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 20) (end 39 25)) (probe (position 39 20))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11"))
        (kind connectionTarget) (ordinal 0) (authored-target "c2::pc")
        (range (start 39 20) (end 39 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 77 11) (end 77 16)) (probe (position 77 11))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))
        (kind connectionSource) (ordinal 0) (authored-target "c1::pb")
        (range (start 77 11) (end 77 16))
        (outcome (status unresolved))
      )
    )
    (query (range (start 77 20) (end 77 25)) (probe (position 77 20))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a12::b21"))
        (kind connectionTarget) (ordinal 0) (authored-target "c3::pd")
        (range (start 77 20) (end 77 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 59 10) (end 59 16)) (probe (position 59 10))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))
        (kind connectionSource) (ordinal 0) (authored-target "b11::pe")
        (range (start 59 10) (end 59 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b11::pe") (range (start 41 3) (end 41 260)))
        )
      )
    )
    (query (range (start 59 20) (end 59 26)) (probe (position 59 20))
      (reference
        (source (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11"))
        (kind connectionTarget) (ordinal 0) (authored-target "b12::pf")
        (range (start 59 20) (end 59 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2c-Parts Interconnection-Multiple Decompositions::a11::b12::pf") (range (start 56 3) (end 56 23)))
        )
      )
    )
  )
)
~~~
