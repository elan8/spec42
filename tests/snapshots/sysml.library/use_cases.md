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
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 16) (end 6 27))
      )
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
        (range (start 16 34) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 19) (end 17 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 34) (end 20 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 33) (end 27 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 61) (end 34 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 70) (end 41 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 48) (end 50 53))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a8dc5ba9484a6fc3a40f90a07b7781017fc56a8e268b915a70f40d927c19a663"))
  (declarations
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "This package defines the base types for use cases and related behavioral elements in the SysML language.\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::Case") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Cases::cases") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Case")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "UseCase is the most general class of performances of UseCaseDefinitions. \nUseCase is the base class of all UseCaseDefinitions.\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (redefinition (reference "done")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "done")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "The ending snapshot of a Use Case.\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers abstract reference) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (subsetting (reference "useCases")) (subsetting (reference "enclosedPerformances")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Other UseCases included by this UseCase (i.e., as modeled by an \nIncludeUseCaseUsage).\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (kind kerml-literal-infinity) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::obj"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (redefinition (reference "Case::self")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (redefinition (reference "start")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "start")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "The starting snapshot of a Use Case. \n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (subsetting (reference "useCases")) (subsetting (reference "subcases")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Other UseCases carried out as part of the performance of this UseCase.\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (kind kerml-literal-infinity) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subj"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Case::subj")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseCase")) (subsetting (reference "cases")))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "useCases is the base feature of all UseCaseUsages.\n"))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (kind kerml-literal-infinity) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Cases::cases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0))
      (authored-target "Case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind redefinition) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind subsetting) (ordinal 0))
      (authored-target "useCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind subsetting) (ordinal 1))
      (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Case::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind redefinition) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind subsetting) (ordinal 0))
      (authored-target "useCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind subsetting) (ordinal 1))
      (authored-target "subcases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subj"))) (kind redefinition) (ordinal 0))
      (authored-target "Case::subj")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind subsetting) (ordinal 0))
      (authored-target "cases")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::obj"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subj"))) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source inherited) (from (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "includedUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::obj")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source inherited) (from (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case-def) (name "UseCase")) (named (kind use-case) (name "subUseCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subj")))
      (featured-by (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")))
      (type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (source direct))
      (supertype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases")) (scopes any feature))
      (subtype (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (named (kind use-case) (name "useCases")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/use_cases.md") (range (start 6 16) (end 6 27)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 7 16) (end 7 28)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (path (named (kind library-package) (name "UseCases")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Cases::cases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase"))) (kind specialization) (ordinal 0) (authored-target "Case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 21) (end 27 28)) (probe (position 27 21))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 27 33) (end 27 37)) (probe (position 27 33))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::done"))) (kind redefinition) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 43) (end 41 50)) (probe (position 41 43))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 60) (end 41 68)) (probe (position 41 60))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind subsetting) (ordinal 0) (authored-target "useCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 41 70) (end 41 90)) (probe (position 41 70))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::includedUseCases"))) (kind subsetting) (ordinal 1) (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 22) (end 16 29)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 16 34) (end 16 44)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::self"))) (kind redefinition) (ordinal 0) (authored-target "Case::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 22) (end 20 29)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 20 34) (end 20 39)) (probe (position 20 34))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::start"))) (kind redefinition) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 34) (end 34 41)) (probe (position 34 34))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 51) (end 34 59)) (probe (position 34 51))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind subsetting) (ordinal 0) (authored-target "useCases")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 34 61) (end 34 69)) (probe (position 34 61))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subUseCases"))) (kind subsetting) (ordinal 1) (authored-target "subcases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 17 19) (end 17 29)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase::subj"))) (kind redefinition) (ordinal 0) (authored-target "Case::subj")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 50 21) (end 50 28)) (probe (position 50 21))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind featureTyping) (ordinal 0) (authored-target "UseCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::UseCase")))))
    )
  )
  (query (document "memory://snapshot/use_cases.md") (range (start 50 48) (end 50 53)) (probe (position 50 48))
    (reference (id (source (node (document "memory://snapshot/use_cases.md") (qualified-name "UseCases::useCases"))) (kind subsetting) (ordinal 0) (authored-target "cases")
      (outcome (status unresolved)))
    )
  )
)
~~~
