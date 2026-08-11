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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 46))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5176b3c0710cb0bc1e77ed290e2f8243a0d38f327caf0b0dd750deb45ef8bb33") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Constraints"))) (kind "package") (name "Constraints") (declared-name "Constraints") (range (start (line 0) (character 0)) (end (line 0) (character 1263))))
    (element (id (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (kind "import") (name "BooleanEvaluation") (declared-name "BooleanEvaluation") (range (start (line 7) (character 1)) (end (line 7) (character 48))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::BooleanEvaluation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 47))))))
    (element (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind "constraint def") (name "ConstraintCheck") (declared-name "ConstraintCheck") (range (start (line 12) (character 1)) (end (line 12) (character 293))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BooleanEvaluation") (range (start (line 12) (character 44)) (end (line 12) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "Constraints::ConstraintCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 12) (character 1)) (end (line 12) (character 293))) (parent (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Constraints::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1263))) (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))) (kind "constraint") (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks") (range (start (line 29) (character 1)) (end (line 29) (character 210))) (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 29) (character 1)) (end (line 29) (character 210))) (parent (node (document "d0") (qualified-name "Constraints::assertedConstraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::booleanEvaluations"))) (kind "import") (name "booleanEvaluations") (declared-name "booleanEvaluations") (range (start (line 8) (character 1)) (end (line 8) (character 49))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::booleanEvaluations") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 48))))))
    (element (id (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind "constraint") (name "constraintChecks") (declared-name "constraintChecks") (range (start (line 22) (character 1)) (end (line 22) (character 181))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConstraintCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "Constraints::constraintChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 22) (character 1)) (end (line 22) (character 181))) (parent (node (document "d0") (qualified-name "Constraints::constraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::falseEvaluations"))) (kind "import") (name "falseEvaluations") (declared-name "falseEvaluations") (range (start (line 10) (character 1)) (end (line 10) (character 47))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::falseEvaluations") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))) (kind "constraint") (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks") (range (start (line 36) (character 1)) (end (line 36) (character 210))) (parent (node (document "d0") (qualified-name "Constraints"))))
    (element (id (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 36) (character 1)) (end (line 36) (character 210))) (parent (node (document "d0") (qualified-name "Constraints::negatedConstraintChecks"))))
    (element (id (node (document "d0") (qualified-name "Constraints::trueEvaluations"))) (kind "import") (name "trueEvaluations") (declared-name "trueEvaluations") (range (start (line 9) (character 1)) (end (line 9) (character 46))) (parent (node (document "d0") (qualified-name "Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::trueEvaluations") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 45))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::BooleanEvaluation") (range (start (line 7) (character 16)) (end (line 7) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0)) (authored-target "BooleanEvaluation") (range (start (line 12) (character 44)) (end (line 12) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints::BooleanEvaluation")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::booleanEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::booleanEvaluations") (range (start (line 8) (character 16)) (end (line 8) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "ConstraintCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraints::ConstraintCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::falseEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::falseEvaluations") (range (start (line 10) (character 16)) (end (line 10) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraints::trueEvaluations"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::trueEvaluations") (range (start (line 9) (character 16)) (end (line 9) (character 45))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (target (node (document "d0") (qualified-name "Constraints::BooleanEvaluation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (target (node (document "d0") (qualified-name "Constraints::ConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraints::constraintChecks"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Constraints::ConstraintCheck")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
