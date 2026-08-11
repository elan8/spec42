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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwCase,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwCase,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwSubject,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwObjective,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Ident,KwDefault,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwReturn,KwRef,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwCase,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Cases'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Requirements::RequirementCheck')
    (import_decl private 'Calculations::Calculation')
    (import_decl private 'Calculations::calculations')
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (case_def abstract 'Case' :> 'Calculation'
      (documentation)
      (sysml_decl ref 'self' : 'Case' :>> 'Calculation::self')
      (sysml_decl 'subj' : 'Anything' multiplicity
        (documentation))
      (part_usage ref 'actors' : 'Part' :> 'parts' multiplicity
        (documentation))
      (objective_member)
      (return_member)
      (sysml_decl abstract 'subcases' : 'Case' :> 'cases', 'subcalculations' multiplicity
        (documentation)))
    (sysml_decl abstract 'cases' : 'Case' :> 'calculations' multiplicity nonunique
      (documentation))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Calculation'
semantic.unresolved_name 'Calculation::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcalculations'
semantic.unresolved_name 'calculations'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Calculation'
semantic.unresolved_name 'Calculation::self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'subcalculations'
semantic.unresolved_name 'calculations'
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
    (element (id (node (document "d0") (qualified-name "Cases"))) (kind "package") (name "Cases") (declared-name "Cases") (range (start (line 0) (character 0)) (end (line 0) (character 1619))))
    (element (id (node (document "d0") (qualified-name "Cases::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Cases::Calculation"))) (kind "import") (name "Calculation") (declared-name "Calculation") (range (start (line 9) (character 1)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::Calculation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Cases::Case"))) (kind "case def") (name "Case") (declared-name "Case") (range (start (line 14) (character 1)) (end (line 14) (character 1097))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Calculation") (range (start (line 14) (character 27)) (end (line 14) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::_documentation"))) (kind "documentation") (name "") (range (start (line 14) (character 1)) (end (line 14) (character 1097))) (parent (node (document "d0") (qualified-name "Cases::Case"))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind "ref") (name "actors") (declared-name "actors") (range (start (line 30) (character 2)) (end (line 30) (character 252))) (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part") (range (start (line 30) (character 20)) (end (line 30) (character 24)))) (subsetting (reference "parts") (range (start (line 30) (character 34)) (end (line 30) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::actors::_documentation"))) (kind "documentation") (name "") (range (start (line 30) (character 2)) (end (line 30) (character 252))) (parent (node (document "d0") (qualified-name "Cases::Case::actors"))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::obj"))) (kind "objective") (name "obj") (declared-name "obj") (range (start (line 39) (character 2)) (end (line 39) (character 189))) (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (relationships (typing (reference "RequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "Cases::Case::subj"))) (kind "subject") (name "subj") (declared-name "subj") (range (start (line 23) (character 2)) (end (line 23) (character 108))) (parent (node (document "d0") (qualified-name "Cases::Case"))) (authored (relationships (typing (reference "Anything") (range none)))))
    (element (id (node (document "d0") (qualified-name "Cases::Part"))) (kind "import") (name "Part") (declared-name "Part") (range (start (line 11) (character 1)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Cases::RequirementCheck"))) (kind "import") (name "RequirementCheck") (declared-name "RequirementCheck") (range (start (line 8) (character 1)) (end (line 8) (character 47))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::RequirementCheck") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Cases::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1619))) (parent (node (document "d0") (qualified-name "Cases"))))
    (element (id (node (document "d0") (qualified-name "Cases::calculations"))) (kind "import") (name "calculations") (declared-name "calculations") (range (start (line 10) (character 1)) (end (line 10) (character 43))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculations::calculations") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Cases::cases"))) (kind "case") (name "cases") (declared-name "cases") (range (start (line 64) (character 1)) (end (line 64) (character 131))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Feature)) (relationships (typing (reference "Case") (range none)))))
    (element (id (node (document "d0") (qualified-name "Cases::cases::_documentation"))) (kind "documentation") (name "") (range (start (line 64) (character 1)) (end (line 64) (character 131))) (parent (node (document "d0") (qualified-name "Cases::cases"))))
    (element (id (node (document "d0") (qualified-name "Cases::parts"))) (kind "import") (name "parts") (declared-name "parts") (range (start (line 12) (character 1)) (end (line 12) (character 29))) (parent (node (document "d0") (qualified-name "Cases"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 28))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Cases::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Calculation"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::Calculation") (range (start (line 9) (character 16)) (end (line 9) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case"))) (kind specialization) (ordinal 0)) (authored-target "Calculation") (range (start (line 14) (character 27)) (end (line 14) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Calculation")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range (start (line 30) (character 20)) (end (line 30) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::actors"))) (kind subsetting) (ordinal 0)) (authored-target "parts") (range (start (line 30) (character 34)) (end (line 30) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::obj"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Case::subj"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (range (start (line 11) (character 16)) (end (line 11) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::RequirementCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Requirements::RequirementCheck") (range (start (line 8) (character 16)) (end (line 8) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::calculations"))) (kind membershipImport) (ordinal 0)) (authored-target "Calculations::calculations") (range (start (line 10) (character 16)) (end (line 10) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Cases::cases"))) (kind featureTyping) (ordinal 0)) (authored-target "Case") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Cases::Case")))))
    (reference (id (source (node (document "d0") (qualified-name "Cases::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (range (start (line 12) (character 16)) (end (line 12) (character 28))) (outcome (status unresolved)))
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
