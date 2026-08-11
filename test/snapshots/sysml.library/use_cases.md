# META
~~~ini
description=Standard Library: Systems Library/UseCases
type=file
~~~
# SOURCE
~~~sysml
standard library package UseCases {
	doc
	/*
	 * This package defines the base types for use cases and related behavioral elements in the SysML language.
	 */
	 
	private import Cases::Case;
	private import Cases::cases;
	
	use case def UseCase :> Case {
		doc
		/*
		 * UseCase is the most general class of performances of UseCaseDefinitions. 
		 * UseCase is the base class of all UseCaseDefinitions.
		 */
	
		ref use case self : UseCase :>> Case::self;
		subject subj :>> Case::subj;
		objective obj :>> Case::obj;
		
		ref use case start: UseCase :>> start {
			doc
			/*
			 * The starting snapshot of a Use Case. 
			 */
		}
		
		ref use case done: UseCase :>> done {
			doc
			/*
			 * The ending snapshot of a Use Case.
			 */
		}

		abstract use case subUseCases : UseCase[0..*] :> useCases, subcases {
			doc
			/*
			 * Other UseCases carried out as part of the performance of this UseCase.
			 */
		}
		
		abstract ref use case includedUseCases : UseCase[0..*] :> useCases, enclosedPerformances {
			doc
			/*
			 * Other UseCases included by this UseCase (i.e., as modeled by an 
			 * IncludeUseCaseUsage).
			 */
		}
	}
	
	use case useCases : UseCase[0..*] nonunique :> cases {
		doc
		/*
		 * useCases is the base feature of all UseCaseUsages.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "use_cases.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 28))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package UseCases {
	doc
	/*
	 * This package defines the base types for use cases and related behavioral elements in the SysML language.
	 */
	 
	private import Cases::Case;
	private import Cases::cases;
	
	use case def UseCase :> Case {
		doc
		/*
		 * UseCase is the most general class of performances of UseCaseDefinitions. 
		 * UseCase is the base class of all UseCaseDefinitions.
		 */
	
		ref use case self : UseCase :>> Case::self;
		subject subj :>> Case::subj;
		objective obj :>> Case::obj;
		
		ref use case start: UseCase :>> start {
			doc
			/*
			 * The starting snapshot of a Use Case. 
			 */
		}
		
		ref use case done: UseCase :>> done {
			doc
			/*
			 * The ending snapshot of a Use Case.
			 */
		}

		abstract use case subUseCases : UseCase[0..*] :> useCases, subcases {
			doc
			/*
			 * Other UseCases carried out as part of the performance of this UseCase.
			 */
		}
		
		abstract ref use case includedUseCases : UseCase[0..*] :> useCases, enclosedPerformances {
			doc
			/*
			 * Other UseCases included by this UseCase (i.e., as modeled by an 
			 * IncludeUseCaseUsage).
			 */
		}
	}
	
	use case useCases : UseCase[0..*] nonunique :> cases {
		doc
		/*
		 * useCases is the base feature of all UseCaseUsages.
		 */
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a9d475b7ddb2300aee0fe6e98bcd11a649565f61a10630099957a8359c1e2079") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "UseCases"))) (kind "package") (name "UseCases") (declared-name "UseCases") (range (start (line 0) (character 0)) (end (line 0) (character 1271))))
    (element (id (node (document "d0") (qualified-name "UseCases::Case"))) (kind "import") (name "Case") (declared-name "Case") (range (start (line 6) (character 1)) (end (line 6) (character 28))) (parent (node (document "d0") (qualified-name "UseCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::Case") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 27))))))
    (element (id (node (document "d0") (qualified-name "UseCases::UseCase"))) (kind "use case def") (name "UseCase") (declared-name "UseCase") (range (start (line 9) (character 1)) (end (line 9) (character 912))) (parent (node (document "d0") (qualified-name "UseCases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Case") (range (start (line 9) (character 25)) (end (line 9) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "UseCases::UseCase::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 1)) (end (line 9) (character 912))) (parent (node (document "d0") (qualified-name "UseCases::UseCase"))))
    (element (id (node (document "d0") (qualified-name "UseCases::UseCase::obj"))) (kind "objective") (name "obj") (declared-name "obj") (range (start (line 18) (character 2)) (end (line 18) (character 30))) (parent (node (document "d0") (qualified-name "UseCases::UseCase"))))
    (element (id (node (document "d0") (qualified-name "UseCases::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1271))) (parent (node (document "d0") (qualified-name "UseCases"))))
    (element (id (node (document "d0") (qualified-name "UseCases::cases"))) (kind "import") (name "cases") (declared-name "cases") (range (start (line 7) (character 1)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "UseCases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Cases::cases") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 28))))))
    (element (id (node (document "d0") (qualified-name "UseCases::useCases"))) (kind "use case") (name "useCases") (declared-name "useCases") (range (start (line 50) (character 1)) (end (line 50) (character 131))) (parent (node (document "d0") (qualified-name "UseCases"))) (authored (membership (kind Feature)) (relationships (typing (reference "UseCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCases::useCases::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 1)) (end (line 50) (character 131))) (parent (node (document "d0") (qualified-name "UseCases::useCases"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "UseCases::Case"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::Case") (range (start (line 6) (character 16)) (end (line 6) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0)) (authored-target "Case") (range (start (line 9) (character 25)) (end (line 9) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCases::cases"))) (kind membershipImport) (ordinal 0)) (authored-target "Cases::cases") (range (start (line 7) (character 16)) (end (line 7) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "UseCases::useCases"))) (kind featureTyping) (ordinal 0)) (authored-target "UseCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCases::UseCase")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "UseCases::UseCase"))) (target (node (document "d0") (qualified-name "UseCases::Case"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCases::useCases"))) (target (node (document "d0") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCases::useCases"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 25) (end 9 29)) (probe (position 9 25))
      (reference
        (source (document "d0") (qualified-name "UseCases::UseCase"))
        (kind specialization) (ordinal 0) (authored-target "Case")
        (range (start 9 25) (end 9 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "UseCases::Case") (range (start 6 1) (end 6 28)))
        )
      )
    )
    (query (range (start 6 16) (end 6 27)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "UseCases::Case"))
        (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
        (range (start 6 16) (end 6 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 28)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "UseCases::cases"))
        (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
        (range (start 7 16) (end 7 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
