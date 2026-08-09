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
# EXPECTED
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'Case::obj'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'cases'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Case'
semantic.unresolved_name 'Case::self'
semantic.unresolved_name 'Case::subj'
semantic.unresolved_name 'Case::obj'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
semantic.unresolved_name 'subcases'
semantic.unresolved_name 'enclosedPerformances'
semantic.unresolved_name 'cases'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwUse,KwCase,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwUse,KwCase,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwSubject,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwObjective,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwRef,KwUse,KwCase,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,KwUse,KwCase,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwUse,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRef,KwUse,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwUse,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'UseCases'
    (documentation)
    (import_decl private 'Cases::Case')
    (import_decl private 'Cases::cases')
    (use_case_def 'UseCase' :> 'Case'
      (documentation)
      (sysml_decl ref 'self' : 'UseCase' :>> 'Case::self')
      (sysml_decl 'subj' :>> 'Case::subj')
      (objective_member)
      (sysml_decl ref 'start' : 'UseCase' :>> 'start'
        (documentation))
      (sysml_decl ref 'done' : 'UseCase' :>> 'done'
        (documentation))
      (sysml_decl abstract 'subUseCases' : 'UseCase' :> 'useCases', 'subcases' multiplicity
        (documentation))
      (sysml_decl abstract ref 'includedUseCases' : 'UseCase' :> 'useCases', 'enclosedPerformances' multiplicity
        (documentation)))
    (sysml_decl 'useCases' : 'UseCase' :> 'cases' multiplicity nonunique
      (documentation))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "UseCases"))) (name "UseCases") (declared-name "UseCases")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "UseCases::Case"))) (name "Case") (declared-name "Case"))
        (element (kind "use case def") (id (node (document "d0") (qualified-name "UseCases::UseCase"))) (name "UseCase") (declared-name "UseCase")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "UseCases::UseCase::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "UseCases::UseCase")))))
            (element (kind "objective") (id (node (document "d0") (qualified-name "UseCases::UseCase::obj"))) (name "obj") (declared-name "obj") (effective (featuring-type (node (document "d0") (qualified-name "UseCases::UseCase")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "UseCases::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "UseCases::cases"))) (name "cases") (declared-name "cases"))
        (element (kind "use case") (id (node (document "d0") (qualified-name "UseCases::useCases"))) (name "useCases") (declared-name "useCases")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "UseCases::useCases::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "UseCases::UseCase")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "UseCases::UseCase::_documentation"))) (to (node (document "d0") (qualified-name "UseCases::UseCase"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "UseCases::_documentation"))) (to (node (document "d0") (qualified-name "UseCases"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "UseCases::useCases::_documentation"))) (to (node (document "d0") (qualified-name "UseCases::useCases"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCases::useCases"))) (to (node (document "d0") (qualified-name "UseCases::UseCase"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/use_cases.md"
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
        (range (start 9 1) (end 9 912))
      )
    )
  )
)
~~~
