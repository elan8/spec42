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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Constraints"))) (name "Constraints") (declared-name "Constraints")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (name "BooleanEvaluation") (declared-name "BooleanEvaluation"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (name "ConstraintCheck") (declared-name "ConstraintCheck")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Constraints::ConstraintCheck")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Constraints::_documentation"))) (name ""))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))) (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks::_documentation"))) (name ""))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraints::booleanEvaluations"))) (name "booleanEvaluations") (declared-name "booleanEvaluations"))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (name "constraintChecks") (declared-name "constraintChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Constraints::constraintChecks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Constraints::ConstraintCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraints::falseEvaluations"))) (name "falseEvaluations") (declared-name "falseEvaluations"))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))) (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks::_documentation"))) (name ""))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraints::trueEvaluations"))) (name "trueEvaluations") (declared-name "trueEvaluations"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Constraints::ConstraintCheck::_documentation"))) (to (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Constraints::_documentation"))) (to (node (document "d0") (qualified-name "Constraints"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks::_documentation"))) (to (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Constraints::constraintChecks::_documentation"))) (to (node (document "d0") (qualified-name "Constraints::constraintChecks"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks::_documentation"))) (to (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (to (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
