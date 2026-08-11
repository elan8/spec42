# META
~~~ini
description=SysML Example (Requirements): RequirementDerivationExample
type=file
~~~
# SOURCE
~~~sysml
package RequirementDerivationExample {
	private import RequirementDerivation::*;
	
	requirement def Req1;
	
	requirement def Req1_1;
	requirement def Req1_2;
	
	#derivation connection def Req1_Derivation {
		end #original r1 : Req1;
		end #derive r1_1 : Req1_1;
		end #derive r1_2 : Req1_2;
	}
	
	part def System;
	part def Subsystem1;
	part def Subsystem2;
	
	part system : System {
		part sub1 : Subsystem1;
		part sub2 : Subsystem2;
	}
	
	part satisfactionContext {
		ref :>> system;
		
		satisfy requirement req1 : Req1 by system;
		satisfy requirement req1_1 : Req1_1 by system.sub1;
		satisfy requirement req1_2 : Req1_2 by system.sub2;
		
		#derivation connection : Req1_Derivation {
			end r1 ::> req1;
			end r1_1 ::> req1_1;
			end r1_2 ::> req1_1;
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_derivation_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 22) (end 26 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 22) (end 27 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 22) (end 28 37))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementDerivationExample {
    private import RequirementDerivation::*;

    requirement def Req1;

    requirement def Req1_1;
    requirement def Req1_2;

    #derivation connection def Req1_Derivation {
        end #original r1 : Req1;
        end #derive r1_1 : Req1_1;
        end #derive r1_2 : Req1_2;
    }

    part def System;
    part def Subsystem1;
    part def Subsystem2;

    part system : System {
        part sub1 : Subsystem1;
        part sub2 : Subsystem2;
    }

    part satisfactionContext {
        ref :>> system;

        satisfy requirement req1 : Req1 by system;
        satisfy requirement req1_1 : Req1_1 by system.sub1;
        satisfy requirement req1_2 : Req1_2 by system.sub2;

        #derivation connection : Req1_Derivation {
            end r1 ::> req1;
            end r1_1 ::> req1_1;
            end r1_2 ::> req1_1;
        }

    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "719120dd17f1efdaddbfa5820b81988552fd7bfe38b65a02b8db13884e6edca1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample"))) (kind "package") (name "RequirementDerivationExample") (declared-name "RequirementDerivationExample") (range (start (line 0) (character 0)) (end (line 0) (character 772))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 41))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementDerivation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1"))) (kind "requirement def") (name "Req1") (declared-name "Req1") (range (start (line 3) (character 1)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1"))) (kind "requirement def") (name "Req1_1") (declared-name "Req1_1") (range (start (line 5) (character 1)) (end (line 5) (character 24))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2"))) (kind "requirement def") (name "Req1_2") (declared-name "Req1_2") (range (start (line 6) (character 1)) (end (line 6) (character 24))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (kind "derivation connection") (name "Req1_Derivation") (declared-name "Req1_Derivation") (range (start (line 8) (character 1)) (end (line 8) (character 133))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind "interface end") (name "r1") (declared-name "r1") (range (start (line 9) (character 2)) (end (line 9) (character 26))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (authored (relationships (typing (reference "Req1") (range none)))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind "interface end") (name "r1_1") (declared-name "r1_1") (range (start (line 10) (character 2)) (end (line 10) (character 28))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (authored (relationships (typing (reference "Req1_1") (range none)))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind "interface end") (name "r1_2") (declared-name "r1_2") (range (start (line 11) (character 2)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (authored (relationships (typing (reference "Req1_2") (range none)))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1"))) (kind "part def") (name "Subsystem1") (declared-name "Subsystem1") (range (start (line 15) (character 1)) (end (line 15) (character 21))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2"))) (kind "part def") (name "Subsystem2") (declared-name "Subsystem2") (range (start (line 16) (character 1)) (end (line 16) (character 21))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::System"))) (kind "part def") (name "System") (declared-name "System") (range (start (line 14) (character 1)) (end (line 14) (character 17))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind "part") (name "satisfactionContext") (declared-name "satisfactionContext") (range (start (line 23) (character 1)) (end (line 23) (character 327))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::system"))) (kind "ref") (name "system") (declared-name "system") (range (start (line 24) (character 2)) (end (line 24) (character 17))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (kind "part") (name "system") (declared-name "system") (range (start (line 18) (character 1)) (end (line 18) (character 78))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "System") (range (start (line 18) (character 15)) (end (line 18) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind "part") (name "sub1") (declared-name "sub1") (range (start (line 19) (character 2)) (end (line 19) (character 25))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Subsystem1") (range (start (line 19) (character 14)) (end (line 19) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind "part") (name "sub2") (declared-name "sub2") (range (start (line 20) (character 2)) (end (line 20) (character 25))) (parent (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Subsystem2") (range (start (line 20) (character 14)) (end (line 20) (character 24)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementDerivation::*") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "Req1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0)) (authored-target "Req1_1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0)) (authored-target "Req1_2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfySource) (ordinal 0)) (authored-target "req1") (range (start (line 26) (character 22)) (end (line 26) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfySource) (ordinal 1)) (authored-target "req1_1") (range (start (line 27) (character 22)) (end (line 27) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfySource) (ordinal 2)) (authored-target "req1_2") (range (start (line 28) (character 22)) (end (line 28) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "system") (range (start (line 26) (character 37)) (end (line 26) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::system")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfyTarget) (ordinal 1)) (authored-target "system::sub1") (range (start (line 27) (character 41)) (end (line 27) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (kind satisfyTarget) (ordinal 2)) (authored-target "system::sub2") (range (start (line 28) (character 41)) (end (line 28) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0)) (authored-target "System") (range (start (line 18) (character 15)) (end (line 18) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::System")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0)) (authored-target "Subsystem1") (range (start (line 19) (character 14)) (end (line 19) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0)) (authored-target "Subsystem2") (range (start (line 20) (character 14)) (end (line 20) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::System"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (target (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 15) (end 18 21)) (probe (position 18 15))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::system"))
        (kind featureTyping) (ordinal 0) (authored-target "System")
        (range (start 18 15) (end 18 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::System") (range (start 14 1) (end 14 17)))
        )
      )
    )
    (query (range (start 26 37) (end 26 43)) (probe (position 26 37))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfyTarget) (ordinal 0) (authored-target "system")
        (range (start 26 37) (end 26 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::system") (range (start 24 2) (end 24 17)))
        )
      )
    )
    (query (range (start 19 14) (end 19 24)) (probe (position 19 14))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))
        (kind featureTyping) (ordinal 0) (authored-target "Subsystem1")
        (range (start 19 14) (end 19 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1") (range (start 15 1) (end 15 21)))
        )
      )
    )
    (query (range (start 20 14) (end 20 24)) (probe (position 20 14))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))
        (kind featureTyping) (ordinal 0) (authored-target "Subsystem2")
        (range (start 20 14) (end 20 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2") (range (start 16 1) (end 16 21)))
        )
      )
    )
    (query (range (start 26 22) (end 26 33)) (probe (position 26 22))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfySource) (ordinal 0) (authored-target "req1")
        (range (start 26 22) (end 26 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 41) (end 27 52)) (probe (position 27 41))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfyTarget) (ordinal 1) (authored-target "system::sub1")
        (range (start 27 41) (end 27 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::system::sub1") (range (start 19 2) (end 19 25)))
        )
      )
    )
    (query (range (start 28 41) (end 28 52)) (probe (position 28 41))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfyTarget) (ordinal 2) (authored-target "system::sub2")
        (range (start 28 41) (end 28 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivationExample::system::sub2") (range (start 20 2) (end 20 25)))
        )
      )
    )
    (query (range (start 27 22) (end 27 37)) (probe (position 27 22))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfySource) (ordinal 1) (authored-target "req1_1")
        (range (start 27 22) (end 27 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 22) (end 28 37)) (probe (position 28 22))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))
        (kind satisfySource) (ordinal 2) (authored-target "req1_2")
        (range (start 28 22) (end 28 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 37)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivationExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation::*")
        (range (start 1 16) (end 1 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
