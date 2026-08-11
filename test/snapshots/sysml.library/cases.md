# META
~~~ini
description=Standard Library: Systems Library/Cases
type=file
~~~
# SOURCE
~~~sysml
standard library package Cases {
	doc
	/*
	 * This package defines the base types for cases and related behavioral elements 
	 * in the SysML language.
	 */

	private import Base::Anything;
	private import Requirements::RequirementCheck;
	private import Calculations::Calculation;
	private import Calculations::calculations;
	private import Parts::Part;
	private import Parts::parts;
	
	abstract case def Case :> Calculation {
		doc
		/*
		 * Case is the most general class of performances of CaseDefinitions. 
		 * Case is the base class of all CaseDefinitions.
		 */
	
		ref case self : Case :>> Calculation::self;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The subject that was investigated by this Case.
			 */
		}
		
		ref part actors : Part[0..*] :> parts {
			doc
			/*
			 * The Parts that fill the role of actors for this Case.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific Cases.)
			 */
		}
		
		objective obj : RequirementCheck[1] {
			doc
			/*
			 * A check of whether the objective RequirementUsage was satisfied for this Case.
			 */
		
			subject subj default Case::result;
		}
		
		return ref result[0..*] {
			doc
			/*
			 * The result determined by the case, which should satisfy the case objective.
			 */
		}
		
		abstract case subcases : Case[0..*] :> cases, subcalculations {
			doc
			/*
			 * Other Cases carried out as part of the performance of this Case.
			 */
		}
	
	}
	
	abstract case cases : Case[0..*] nonunique :> calculations {
		doc
		/*
		 * cases is the base Feature of all CaseUsages.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 28))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Cases {
	doc
	/*
	 * This package defines the base types for cases and related behavioral elements 
	 * in the SysML language.
	 */

	private import Base::Anything;
	private import Requirements::RequirementCheck;
	private import Calculations::Calculation;
	private import Calculations::calculations;
	private import Parts::Part;
	private import Parts::parts;
	
	abstract case def Case :> Calculation {
		doc
		/*
		 * Case is the most general class of performances of CaseDefinitions. 
		 * Case is the base class of all CaseDefinitions.
		 */
	
		ref case self : Case :>> Calculation::self;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The subject that was investigated by this Case.
			 */
		}
		
		ref part actors : Part[0..*] :> parts {
			doc
			/*
			 * The Parts that fill the role of actors for this Case.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific Cases.)
			 */
		}
		
		objective obj : RequirementCheck[1] {
			doc
			/*
			 * A check of whether the objective RequirementUsage was satisfied for this Case.
			 */
		
			subject subj default Case::result;
		}
		
		return ref result[0..*] {
			doc
			/*
			 * The result determined by the case, which should satisfy the case objective.
			 */
		}
		
		abstract case subcases : Case[0..*] :> cases, subcalculations {
			doc
			/*
			 * Other Cases carried out as part of the performance of this Case.
			 */
		}
	
	}
	
	abstract case cases : Case[0..*] nonunique :> calculations {
		doc
		/*
		 * cases is the base Feature of all CaseUsages.
		 */
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ab9cf7aa5a6e98d6e04b8ebf764b8014a04dac71db39a28b12576e158684ab56") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Cases"))) (kind "package") (name "Cases") (declared-name "Cases"))
    (element (id (node (document "d0") (qualified-name "Cases::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Cases::Calculation"))) (kind "import") (name "Calculation") (declared-name "Calculation") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::Calculation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Cases::Case"))) (kind "case def") (name "Case") (declared-name "Case") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Calculation")))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Cases::Case"))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind "ref") (name "actors") (declared-name "actors") (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part")) (subsetting (reference "parts")))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::actors::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Cases::Case::actors"))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::obj"))) (kind "objective") (name "obj") (declared-name "obj") (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (relationships (typing (reference "RequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::subj"))) (kind "subject") (name "subj") (declared-name "subj") (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (relationships (typing (reference "Anything")))))
    (element (id (node (document "d0") (qualified-name "Cases::Part"))) (kind "import") (name "Part") (declared-name "Part") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Cases::RequirementCheck"))) (kind "import") (name "RequirementCheck") (declared-name "RequirementCheck") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::RequirementCheck") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Cases::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Cases"))))
    (element (id (node (document "d0") (qualified-name "Cases::calculations"))) (kind "import") (name "calculations") (declared-name "calculations") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::calculations") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Cases::cases"))) (kind "case") (name "cases") (declared-name "cases") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Feature)) (relationships (typing (reference "Case")))))
    (element (id (node (document "d0") (qualified-name "Cases::cases::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Cases::cases"))))
    (element (id (node (document "d0") (qualified-name "Cases::parts"))) (kind "import") (name "parts") (declared-name "parts") (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Cases::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Calculation"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::Calculation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0)) (authored-target "Calculation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Calculation")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind subsetting) (ordinal 0)) (authored-target "parts") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::obj"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::subj"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::RequirementCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Requirements::RequirementCheck") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::calculations"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::calculations") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0)) (authored-target "Case") (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Cases::Case"))) (target (node (document "d0") (qualified-name "Cases::Calculation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (target (node (document "d0") (qualified-name "Cases::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (target (node (document "d0") (qualified-name "Cases::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Cases::Case::obj"))) (target (node (document "d0") (qualified-name "Cases::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::Case::obj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Cases::Case::subj"))) (target (node (document "d0") (qualified-name "Cases::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::Case::subj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Cases::cases"))) (target (node (document "d0") (qualified-name "Cases::Case"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 30 20) (end 30 24)) (probe (position 30 20))
      (reference
        (source (document "d0") (qualified-name "Cases::Case::actors"))
        (kind featureTyping) (ordinal 0) (authored-target "Part")
        (range (start 30 20) (end 30 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Cases::Part") (range (start 11 1) (end 11 28)))
        )
      )
    )
    (query (range (start 30 34) (end 30 39)) (probe (position 30 34))
      (reference
        (source (document "d0") (qualified-name "Cases::Case::actors"))
        (kind subsetting) (ordinal 0) (authored-target "parts")
        (range (start 30 34) (end 30 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Cases::parts") (range (start 12 1) (end 12 29)))
        )
      )
    )
    (query (range (start 11 16) (end 11 27)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Cases::Part"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
        (range (start 11 16) (end 11 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 27) (end 14 38)) (probe (position 14 27))
      (reference
        (source (document "d0") (qualified-name "Cases::Case"))
        (kind specialization) (ordinal 0) (authored-target "Calculation")
        (range (start 14 27) (end 14 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Cases::Calculation") (range (start 9 1) (end 9 42)))
        )
      )
    )
    (query (range (start 12 16) (end 12 28)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Cases::parts"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
        (range (start 12 16) (end 12 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Cases::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 41)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Cases::Calculation"))
        (kind membershipImport) (ordinal 0) (authored-target "Calculations::Calculation")
        (range (start 9 16) (end 9 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 42)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Cases::calculations"))
        (kind membershipImport) (ordinal 0) (authored-target "Calculations::calculations")
        (range (start 10 16) (end 10 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 46)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Cases::RequirementCheck"))
        (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
        (range (start 8 16) (end 8 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
