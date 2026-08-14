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
  (document "memory://snapshot/use_cases.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 25) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 2) (end 16 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 6) (end 16 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 10) (end 16 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 15) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 16 20) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 2) (end 20 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 6) (end 20 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 10) (end 20 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 15) (end 20 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 20 20) (end 25 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 2) (end 27 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 6) (end 27 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 10) (end 27 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 15) (end 27 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 27 19) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 2) (end 34 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 11) (end 34 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 15) (end 34 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 20) (end 34 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 34 32) (end 39 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 2) (end 41 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 11) (end 41 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 15) (end 41 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 19) (end 41 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 24) (end 41 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 41 41) (end 47 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 1) (end 55 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:225c689a9e4e133dd9e6a66f52741deb42e8982ec5b10788c20a35ee551a36f7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case")) (expressionOperand (reference "ref")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "self")) (expressionOperand (reference "ref")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "start")) (expressionOperand (reference "ref")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "done")) (expressionOperand (reference "abstract")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "subUseCases")) (expressionOperand (reference "abstract")) (expressionOperand (reference "ref")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "includedUseCases"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::obj"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::cases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 1))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 2))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 3))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 4))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 5))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 6))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 7))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 8))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 9))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 10))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 11))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 12))
      (authored-target "abstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 13))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 14))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 15))
      (authored-target "subUseCases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 16))
      (authored-target "abstract")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 17))
      (authored-target "ref")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 18))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 19))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 20))
      (authored-target "includedUseCases")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/use_cases.md") (range (start 6 16) (end 6 27)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 7 16) (end 7 28)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 2) (end 16 5)) (probe (position 16 2))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 0) (authored-target "ref")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 6) (end 16 9)) (probe (position 16 6))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 1) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 10) (end 16 14)) (probe (position 16 10))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 2) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 15) (end 16 19)) (probe (position 16 15))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 3) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 2) (end 20 5)) (probe (position 20 2))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 4) (authored-target "ref")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 6) (end 20 9)) (probe (position 20 6))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 5) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 10) (end 20 14)) (probe (position 20 10))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 6) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 15) (end 20 20)) (probe (position 20 15))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 7) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 2) (end 27 5)) (probe (position 27 2))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 8) (authored-target "ref")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 6) (end 27 9)) (probe (position 27 6))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 9) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 10) (end 27 14)) (probe (position 27 10))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 10) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 15) (end 27 19)) (probe (position 27 15))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 11) (authored-target "done")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 2) (end 34 10)) (probe (position 34 2))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 12) (authored-target "abstract")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 11) (end 34 14)) (probe (position 34 11))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 13) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 15) (end 34 19)) (probe (position 34 15))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 14) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 20) (end 34 31)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 15) (authored-target "subUseCases")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 2) (end 41 10)) (probe (position 41 2))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 16) (authored-target "abstract")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 11) (end 41 14)) (probe (position 41 11))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 17) (authored-target "ref")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 15) (end 41 18)) (probe (position 41 15))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 18) (authored-target "use")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 19) (end 41 23)) (probe (position 41 19))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 19) (authored-target "case")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 24) (end 41 40)) (probe (position 41 24))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind expressionOperand) (ordinal 20) (authored-target "includedUseCases")
      (outcome (status unresolved)))
  )
)
~~~
