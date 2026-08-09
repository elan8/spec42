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
    doc /*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */

    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Actions::Action;
    private import Actions::actions;

    abstract calc def Calculation :> Action, Evaluation {
        doc /*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */

        ref calc self : Calculation :>> Action::self, Evaluation::self;

        abstract calc subcalculations : Calculation :> calculations, subactions {
            doc /*
			 * The subactions of this Calculation that are Calculations.
			 */
        }
    }

    abstract calc calculations : Calculation [0..*] :> actions, evaluations nonunique {
        doc /*
		 * calculations is the base Feature for all CalculationUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Calculations'
      (documentation)
      (membership_import private -> 'Performances::Evaluation'[unresolved])
      (membership_import private -> 'Performances::evaluations'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Actions::actions'[unresolved])
      (calculation_def abstract 'Calculation' :> 'Action'[unresolved] :> 'Evaluation'[unresolved]
        (documentation)
        (calculation_usage reference 'self' : 'Calculations::Calculation'[calculation_def] :>> 'Action::self'[unresolved] :>> 'Evaluation::self'[unresolved] :> 'Calculations::calculations'[calculation_usage][implied])
        (calculation_usage abstract composite 'subcalculations' : 'Calculations::Calculation'[calculation_def] :> 'Calculations::calculations'[calculation_usage] :> 'subactions'[unresolved]
          (documentation)))
      (calculation_usage abstract 'calculations' : 'Calculations::Calculation'[calculation_def] :> 'actions'[unresolved] :> 'evaluations'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
