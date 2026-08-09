# META
~~~ini
description=Standard Library: Systems Library/Constraints
type=file
~~~
# SOURCE
~~~sysml
standard library package Constraints {
	doc
	/*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */

	private import Performances::BooleanEvaluation;
	private import Performances::booleanEvaluations;
	private import Performances::trueEvaluations;
	private import Performances::falseEvaluations;
	
	abstract constraint def ConstraintCheck :> BooleanEvaluation {
		doc
		/*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */
	
		ref constraint self: ConstraintCheck :>> BooleanEvaluation::self;
	}
	
	abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {
		doc
		/*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
	}
	
	abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
		doc
		/*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
	}
		
	abstract constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
		doc
		/*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
	}
		
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'BooleanEvaluation::self'
semantic.unresolved_name 'booleanEvaluations'
semantic.unresolved_name 'trueEvaluations'
semantic.unresolved_name 'falseEvaluations'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BooleanEvaluation'
semantic.unresolved_name 'BooleanEvaluation::self'
semantic.unresolved_name 'booleanEvaluations'
semantic.unresolved_name 'trueEvaluations'
semantic.unresolved_name 'falseEvaluations'
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
KwAbstract,KwConstraint,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwConstraint,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConstraint,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConstraint,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConstraint,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Constraints'
    (documentation)
    (import_decl private 'Performances::BooleanEvaluation')
    (import_decl private 'Performances::booleanEvaluations')
    (import_decl private 'Performances::trueEvaluations')
    (import_decl private 'Performances::falseEvaluations')
    (constraint_def abstract 'ConstraintCheck' :> 'BooleanEvaluation'
      (documentation)
      (constraint_usage ref 'self' : 'ConstraintCheck' :>> 'BooleanEvaluation::self'))
    (constraint_usage abstract 'constraintChecks' : 'ConstraintCheck' multiplicity :> 'booleanEvaluations' nonunique
      (documentation))
    (constraint_usage abstract 'assertedConstraintChecks' :> 'constraintChecks', 'trueEvaluations'
      (documentation))
    (constraint_usage abstract 'negatedConstraintChecks' :> 'constraintChecks', 'falseEvaluations'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Constraints {
    doc /*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */

    private import Performances::BooleanEvaluation;
    private import Performances::booleanEvaluations;
    private import Performances::trueEvaluations;
    private import Performances::falseEvaluations;

    abstract constraint def ConstraintCheck :> BooleanEvaluation {
        doc /*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */

        ref constraint self : ConstraintCheck :>> BooleanEvaluation::self;
    }

    abstract constraint constraintChecks : ConstraintCheck [0..*] :> booleanEvaluations nonunique {
        doc /*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
    }

    abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
        doc /*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
    }

    abstract constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
        doc /*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Constraints'
      (documentation)
      (membership_import private -> 'Performances::BooleanEvaluation'[unresolved])
      (membership_import private -> 'Performances::booleanEvaluations'[unresolved])
      (membership_import private -> 'Performances::trueEvaluations'[unresolved])
      (membership_import private -> 'Performances::falseEvaluations'[unresolved])
      (constraint_def abstract 'ConstraintCheck' :> 'BooleanEvaluation'[unresolved]
        (documentation)
        (constraint_usage reference 'self' : 'Constraints::ConstraintCheck'[constraint_def] :>> 'BooleanEvaluation::self'[unresolved] :> 'Constraints::constraintChecks'[constraint_usage][implied]))
      (constraint_usage abstract 'constraintChecks' : 'Constraints::ConstraintCheck'[constraint_def] :> 'booleanEvaluations'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (constraint_usage abstract 'assertedConstraintChecks' :> 'Constraints::constraintChecks'[constraint_usage] :> 'trueEvaluations'[unresolved]
        (documentation))
      (constraint_usage abstract 'negatedConstraintChecks' :> 'Constraints::constraintChecks'[constraint_usage] :> 'falseEvaluations'[unresolved]
        (documentation)))))
~~~
