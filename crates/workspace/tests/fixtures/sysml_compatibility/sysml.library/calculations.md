# META
~~~ini
description=Standard Library: Systems Library/Calculations
type=file
~~~
# SOURCE
~~~sysml
standard library package Calculations {
	doc
	/*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */

	private import Performances::Evaluation;
	private import Performances::evaluations;
	private import Actions::Action;
	private import Actions::actions;
	
	abstract calc def Calculation :> Action, Evaluation {
		doc
		/*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */
	
		ref calc self: Calculation :>> Action::self, Evaluation::self;
		
		abstract calc subcalculations: Calculation :> calculations, subactions {
			doc
			/*
			 * The subactions of this Calculation that are Calculations.
			 */
		}
		
	}
	
	abstract calc calculations: Calculation[0..*] nonunique :> actions, evaluations {
		doc
		/*
		 * calculations is the base Feature for all CalculationUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Action::self'
semantic.unresolved_name 'Evaluation::self'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'evaluations'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Action::self'
semantic.unresolved_name 'Evaluation::self'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'evaluations'
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
KwAbstract,KwCalc,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwCalc,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwCalc,Ident,Colon,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwCalc,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Calculations'
    (documentation)
    (import_decl private 'Performances::Evaluation')
    (import_decl private 'Performances::evaluations')
    (import_decl private 'Actions::Action')
    (import_decl private 'Actions::actions')
    (calc_def abstract 'Calculation' :> 'Action', 'Evaluation'
      (documentation)
      (calc_usage ref 'self' : 'Calculation' :>> 'Action::self', 'Evaluation::self')
      (calc_usage abstract 'subcalculations' : 'Calculation' :> 'calculations', 'subactions'
        (documentation)))
    (calc_usage abstract 'calculations' : 'Calculation' multiplicity :> 'actions', 'evaluations' nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Calculations {
    doc
    /*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */

    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Actions::Action;
    private import Actions::actions;

    abstract calc def Calculation :> Action, Evaluation {
        doc
        /*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */

        ref calc self: Calculation :>> Action::self, Evaluation::self;

        abstract calc subcalculations: Calculation :> calculations, subactions {
            doc
            /*
			 * The subactions of this Calculation that are Calculations.
			 */
        }

    }

    abstract calc calculations: Calculation[0..*] nonunique :> actions, evaluations {
        doc
        /*
		 * calculations is the base Feature for all CalculationUsages.
		 */
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Calculations"))) (name "Calculations") (declared-name "Calculations")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculations::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculations::Calculation"))) (name "Calculation") (declared-name "Calculation")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Calculations::Calculation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculations::Calculation")))))
            (element (kind "calc") (id (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (name "self") (declared-name "self") (effective (featuring-type (node (document "d0") (qualified-name "Calculations::Calculation")))))
            (element (kind "calc") (id (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (name "subcalculations") (declared-name "subcalculations") (effective (featuring-type (node (document "d0") (qualified-name "Calculations::Calculation"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculations::Calculation")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculations::Evaluation"))) (name "Evaluation") (declared-name "Evaluation"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Calculations::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculations::actions"))) (name "actions") (declared-name "actions"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculations::calculations"))) (name "calculations") (declared-name "calculations")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Calculations::calculations::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculations::calculations")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculations::evaluations"))) (name "evaluations") (declared-name "evaluations"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Calculations::Calculation::_documentation"))) (to (node (document "d0") (qualified-name "Calculations::Calculation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations::_documentation"))) (to (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Calculations::_documentation"))) (to (node (document "d0") (qualified-name "Calculations"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Calculations::calculations::_documentation"))) (to (node (document "d0") (qualified-name "Calculations::calculations"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Calculations::Calculation::self"))) (to (node (document "d0") (qualified-name "Calculations::Calculation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Calculations::Calculation::subcalculations"))) (to (node (document "d0") (qualified-name "Calculations::Calculation"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
