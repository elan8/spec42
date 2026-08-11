# META
~~~ini
description=Standard Library: Systems Library/Requirements
type=file
~~~
# SOURCE
~~~sysml
standard library package Requirements {
	doc
	/*
	 * This package defines the base types for requirements and related elements in the SysML language.
	 */

	private import Base::Anything;
	private import ScalarValues::String;
	private import ControlFunctions::allTrue;
	private import Constraints::constraintChecks;
	private import Constraints::assertedConstraintChecks;
	private import Constraints::negatedConstraintChecks;
	private import Parts::Part;
	private import Parts::parts;
	private import Actions::Action;
	private import Interfaces::Interface;
	private import Attributes::AttributeValue;
	
	private abstract constraint def RequirementConstraintCheck {
		doc
		/*
		 * RequirementConstraintCheck is the base ConstraintCheck for RequirementCheck, defining the
		 * separate assumptions and required constraints such that, if all the assumptions are true,
		 * then all the required constraints must be true.
		 */
	
		constraint assumptions[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * Assumptions that must hold for the required constraints to apply.
			 */
		}
		
		constraint constraints[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * The required constraints that are to be checked.
			 */
		}
		
		return result = allTrue(assumptions()) implies allTrue(constraints()) {
			doc
			/*
			 * If all the assumptions are true, then all the required constraints must hold.
			 */
		}
	}
	
	abstract requirement def RequirementCheck :> RequirementConstraintCheck {
		doc
		/*
		 * RequirementCheck is the most general class for requirements checking. RequirementsCheck is the base
		 * type of all requirement definitions.
		 */
	
		ref requirement :>> self: RequirementCheck;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The entity that is being checked for satisfaction of the required constraints.
			 */
		}
		
		ref part actors : Part[0..*] {
			doc
			/*
			 * The Parts that fill the role of actors for this RequirementCheck.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific RequirementChecks.)
			 */
		}
		
		ref part stakeholders : Part[0..*] {
			doc
			/*
			 * The Parts that represent stakeholders interested in the concern being checked.
			 * (Note: This is not itself a stakeholder parameter, because specific stakeholder
			 * parameters will be added for specific RequirementChecks.)
			 */
		}

		/* 
		 * Note: assumptions and constraints are redefined here solely to simplify the
		 * resolution of their qualified names as library elements.
		 */
		constraint assumptions :>> RequirementConstraintCheck::assumptions;
		constraint constraints :>> RequirementConstraintCheck::constraints;
		
		abstract requirement subrequirements[0..*] :> requirementChecks, constraints {
			doc
			/*
			 * Nested requirements, which are also required constraints.
			 */
		}
		
		abstract concern concerns[0..*] :> concernChecks, subrequirements {
			doc
			/*
			 * The checks of any concerns being addressed (as required constraints).
			 */
		}
		
	}
	
	requirement def FunctionalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A functional requirement specifies an action that a system, or part of a system, must perform.
		 */
	
		subject: Action;
	}
	
	requirement def InterfaceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * An interface requirement specifies an interface for connecting systems and system parts, which
		 * optionally may include item flows across the interface and/or interface constraints.
		 */
	
		subject: Interface;
	}
	
	requirement def PerformanceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A performance requirement quantitavely measures the extent to which a system, or a system part, 
		 * satisfies a required capability or condition.
		 */
	
		subject: AttributeValue;
	}
	
	requirement def PhysicalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A physical requirement specifies physical characteristics and/or physical constraints of the 
		 * system, or a system part.
		 */
	
		subject: Part;
	}
	
	requirement def DesignConstraintCheck :> RequirementCheck {
		doc
		/*
		 * A design constraint specifies a constraint on the implementation of the system or system part, 
		 * such as the system must use a commercial off the shelf component.
		 */
	
		subject: Part;
	}
	
	concern def ConcernCheck :> RequirementCheck {
		doc
		/*
		 * ConcernCheck is the most general class for concern checking. ConcernCheck is the base type of 
		 * all ConcernDefinitions.
		 */
	
		ref concern :>> self: ConcernCheck;
		
	}
	
	abstract requirement requirementChecks: RequirementCheck[0..*] nonunique :> constraintChecks {
		doc
		/*
		 * requirementChecks is the base feature of all requirement usages.
		 */
	}
	
	abstract requirement satisfiedRequirementChecks :> requirementChecks, assertedConstraintChecks {
		doc
		/*
		 * satisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be satisfied.
		 */
	}

	abstract requirement notSatisfiedRequirementChecks: RequirementCheck[0..*] :> requirementChecks, negatedConstraintChecks {
		doc
		/*
		 * notSatisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be not satisfied.
		 */
	}
	
	abstract concern concernChecks: ConcernCheck[0..*] nonunique :> requirementChecks {
		doc
		/*
		 * concernChecks is the base feature of all ConcernUsages.
		 */
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirements.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 41))
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
        (range (start 10 16) (end 10 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 42))
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwAbstract,KwConstraint,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwConstraint,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwConstraint,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwReturn,Ident,Eq,Ident,OpenParen,Ident,OpenParen,CloseParen,CloseParen,KwImplies,Ident,OpenParen,Ident,OpenParen,CloseParen,CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwRequirement,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwSubject,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwConstraint,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwConstraint,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwRequirement,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConcern,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Colon,Ident,Semicolon,
CloseCurly,
KwConcern,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwConcern,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwRequirement,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRequirement,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRequirement,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConcern,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Requirements'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::String')
    (import_decl private 'ControlFunctions::allTrue')
    (import_decl private 'Constraints::constraintChecks')
    (import_decl private 'Constraints::assertedConstraintChecks')
    (import_decl private 'Constraints::negatedConstraintChecks')
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (import_decl private 'Actions::Action')
    (import_decl private 'Interfaces::Interface')
    (import_decl private 'Attributes::AttributeValue')
    (constraint_def private abstract 'RequirementConstraintCheck'
      (documentation)
      (constraint_usage 'assumptions' multiplicity :> 'constraintChecks', 'subperformances'
        (documentation))
      (constraint_usage 'constraints' multiplicity :> 'constraintChecks', 'subperformances'
        (documentation))
      (return_member))
    (requirement_def abstract 'RequirementCheck' :> 'RequirementConstraintCheck'
      (documentation)
      (requirement_usage ref :>> 'self' : 'RequirementCheck')
      (sysml_decl 'subj' : 'Anything' multiplicity
        (documentation))
      (part_usage ref 'actors' : 'Part' multiplicity
        (documentation))
      (part_usage ref 'stakeholders' : 'Part' multiplicity
        (documentation))
      (comment)
      (constraint_usage 'assumptions' :>> 'RequirementConstraintCheck::assumptions')
      (constraint_usage 'constraints' :>> 'RequirementConstraintCheck::constraints')
      (requirement_usage abstract 'subrequirements' :> 'requirementChecks', 'constraints' multiplicity
        (documentation))
      (sysml_decl abstract 'concerns' :> 'concernChecks', 'subrequirements' multiplicity
        (documentation)))
    (requirement_def 'FunctionalRequirementCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl : 'Action'))
    (requirement_def 'InterfaceRequirementCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl : 'Interface'))
    (requirement_def 'PerformanceRequirementCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl : 'AttributeValue'))
    (requirement_def 'PhysicalRequirementCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl : 'Part'))
    (requirement_def 'DesignConstraintCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl : 'Part'))
    (concern_def 'ConcernCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl ref :>> 'self' : 'ConcernCheck'))
    (requirement_usage abstract 'requirementChecks' : 'RequirementCheck' :> 'constraintChecks' multiplicity nonunique
      (documentation))
    (requirement_usage abstract 'satisfiedRequirementChecks' :> 'requirementChecks', 'assertedConstraintChecks'
      (documentation))
    (requirement_usage abstract 'notSatisfiedRequirementChecks' : 'RequirementCheck' :> 'requirementChecks', 'negatedConstraintChecks' multiplicity
      (documentation))
    (sysml_decl abstract 'concernChecks' : 'ConcernCheck' :> 'requirementChecks' multiplicity nonunique
      (documentation))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Interface'
semantic.unresolved_name 'AttributeValue'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'assertedConstraintChecks'
semantic.unresolved_name 'negatedConstraintChecks'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Interface'
semantic.unresolved_name 'AttributeValue'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'constraintChecks'
semantic.unresolved_name 'assertedConstraintChecks'
semantic.unresolved_name 'negatedConstraintChecks'
~~~
# FORMAT
~~~sysml
standard library package Requirements {
	doc
	/*
	 * This package defines the base types for requirements and related elements in the SysML language.
	 */

	private import Base::Anything;
	private import ScalarValues::String;
	private import ControlFunctions::allTrue;
	private import Constraints::constraintChecks;
	private import Constraints::assertedConstraintChecks;
	private import Constraints::negatedConstraintChecks;
	private import Parts::Part;
	private import Parts::parts;
	private import Actions::Action;
	private import Interfaces::Interface;
	private import Attributes::AttributeValue;
	
	private abstract constraint def RequirementConstraintCheck {
		doc
		/*
		 * RequirementConstraintCheck is the base ConstraintCheck for RequirementCheck, defining the
		 * separate assumptions and required constraints such that, if all the assumptions are true,
		 * then all the required constraints must be true.
		 */
	
		constraint assumptions[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * Assumptions that must hold for the required constraints to apply.
			 */
		}
		
		constraint constraints[0..*] :> constraintChecks, subperformances {
			doc
			/*
			 * The required constraints that are to be checked.
			 */
		}
		
		return result = allTrue(assumptions()) implies allTrue(constraints()) {
			doc
			/*
			 * If all the assumptions are true, then all the required constraints must hold.
			 */
		}
	}
	
	abstract requirement def RequirementCheck :> RequirementConstraintCheck {
		doc
		/*
		 * RequirementCheck is the most general class for requirements checking. RequirementsCheck is the base
		 * type of all requirement definitions.
		 */
	
		ref requirement :>> self: RequirementCheck;
		
		subject subj : Anything[1] {
			doc
			/*
			 * The entity that is being checked for satisfaction of the required constraints.
			 */
		}
		
		ref part actors : Part[0..*] {
			doc
			/*
			 * The Parts that fill the role of actors for this RequirementCheck.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific RequirementChecks.)
			 */
		}
		
		ref part stakeholders : Part[0..*] {
			doc
			/*
			 * The Parts that represent stakeholders interested in the concern being checked.
			 * (Note: This is not itself a stakeholder parameter, because specific stakeholder
			 * parameters will be added for specific RequirementChecks.)
			 */
		}

		/* 
		 * Note: assumptions and constraints are redefined here solely to simplify the
		 * resolution of their qualified names as library elements.
		 */
		constraint assumptions :>> RequirementConstraintCheck::assumptions;
		constraint constraints :>> RequirementConstraintCheck::constraints;
		
		abstract requirement subrequirements[0..*] :> requirementChecks, constraints {
			doc
			/*
			 * Nested requirements, which are also required constraints.
			 */
		}
		
		abstract concern concerns[0..*] :> concernChecks, subrequirements {
			doc
			/*
			 * The checks of any concerns being addressed (as required constraints).
			 */
		}
		
	}
	
	requirement def FunctionalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A functional requirement specifies an action that a system, or part of a system, must perform.
		 */
	
		subject: Action;
	}
	
	requirement def InterfaceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * An interface requirement specifies an interface for connecting systems and system parts, which
		 * optionally may include item flows across the interface and/or interface constraints.
		 */
	
		subject: Interface;
	}
	
	requirement def PerformanceRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A performance requirement quantitavely measures the extent to which a system, or a system part, 
		 * satisfies a required capability or condition.
		 */
	
		subject: AttributeValue;
	}
	
	requirement def PhysicalRequirementCheck :> RequirementCheck {
		doc
		/*
		 * A physical requirement specifies physical characteristics and/or physical constraints of the 
		 * system, or a system part.
		 */
	
		subject: Part;
	}
	
	requirement def DesignConstraintCheck :> RequirementCheck {
		doc
		/*
		 * A design constraint specifies a constraint on the implementation of the system or system part, 
		 * such as the system must use a commercial off the shelf component.
		 */
	
		subject: Part;
	}
	
	concern def ConcernCheck :> RequirementCheck {
		doc
		/*
		 * ConcernCheck is the most general class for concern checking. ConcernCheck is the base type of 
		 * all ConcernDefinitions.
		 */
	
		ref concern :>> self: ConcernCheck;
		
	}
	
	abstract requirement requirementChecks: RequirementCheck[0..*] nonunique :> constraintChecks {
		doc
		/*
		 * requirementChecks is the base feature of all requirement usages.
		 */
	}
	
	abstract requirement satisfiedRequirementChecks :> requirementChecks, assertedConstraintChecks {
		doc
		/*
		 * satisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be satisfied.
		 */
	}

	abstract requirement notSatisfiedRequirementChecks: RequirementCheck[0..*] :> requirementChecks, negatedConstraintChecks {
		doc
		/*
		 * notSatisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be not satisfied.
		 */
	}
	
	abstract concern concernChecks: ConcernCheck[0..*] nonunique :> requirementChecks {
		doc
		/*
		 * concernChecks is the base feature of all ConcernUsages.
		 */
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e2c72a9d91c48a67bb4f9266e651b6affd8e1682a4bfa2d44d5754aab8498023") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirements"))) (kind "package") (name "Requirements") (declared-name "Requirements") (range (start (line 0) (character 0)) (end (line 0) (character 5466))))
    (element (id (node (document "d0") (qualified-name "Requirements::Action"))) (kind "import") (name "Action") (declared-name "Action") (range (start (line 14) (character 1)) (end (line 14) (character 32))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Requirements::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (kind "import") (name "AttributeValue") (declared-name "AttributeValue") (range (start (line 16) (character 1)) (end (line 16) (character 43))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Attributes::AttributeValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (kind "concern def") (name "ConcernCheck") (declared-name "ConcernCheck") (range (start (line 154) (character 1)) (end (line 154) (character 239))) (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::ConcernCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 154) (character 1)) (end (line 154) (character 239))) (parent (node (document "d0") (qualified-name "Requirements::ConcernCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind "requirement def") (name "DesignConstraintCheck") (declared-name "DesignConstraintCheck") (range (start (line 144) (character 1)) (end (line 144) (character 271))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 144) (character 42)) (end (line 144) (character 58)))) (subject (reference "Requirements::DesignConstraintCheck::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind "subject") (name "") (range (start (line 151) (character 2)) (end (line 151) (character 16))) (parent (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (authored (relationships (typing (reference "Part") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 144) (character 1)) (end (line 144) (character 271))) (parent (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind "requirement def") (name "FunctionalRequirementCheck") (declared-name "FunctionalRequirementCheck") (range (start (line 105) (character 1)) (end (line 105) (character 206))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 105) (character 47)) (end (line 105) (character 63)))) (subject (reference "Requirements::FunctionalRequirementCheck::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind "subject") (name "") (range (start (line 111) (character 2)) (end (line 111) (character 18))) (parent (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (authored (relationships (typing (reference "Action") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 105) (character 1)) (end (line 105) (character 206))) (parent (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::Interface"))) (kind "import") (name "Interface") (declared-name "Interface") (range (start (line 15) (character 1)) (end (line 15) (character 38))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interfaces::Interface") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind "requirement def") (name "InterfaceRequirementCheck") (declared-name "InterfaceRequirementCheck") (range (start (line 114) (character 1)) (end (line 114) (character 298))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 114) (character 46)) (end (line 114) (character 62)))) (subject (reference "Requirements::InterfaceRequirementCheck::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind "subject") (name "") (range (start (line 121) (character 2)) (end (line 121) (character 21))) (parent (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (authored (relationships (typing (reference "Interface") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 114) (character 1)) (end (line 114) (character 298))) (parent (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::Part"))) (kind "import") (name "Part") (declared-name "Part") (range (start (line 12) (character 1)) (end (line 12) (character 28))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind "requirement def") (name "PerformanceRequirementCheck") (declared-name "PerformanceRequirementCheck") (range (start (line 124) (character 1)) (end (line 124) (character 268))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 124) (character 48)) (end (line 124) (character 64)))) (subject (reference "Requirements::PerformanceRequirementCheck::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind "subject") (name "") (range (start (line 131) (character 2)) (end (line 131) (character 26))) (parent (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (authored (relationships (typing (reference "AttributeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 124) (character 1)) (end (line 124) (character 268))) (parent (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind "requirement def") (name "PhysicalRequirementCheck") (declared-name "PhysicalRequirementCheck") (range (start (line 134) (character 1)) (end (line 134) (character 232))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 134) (character 45)) (end (line 134) (character 61)))) (subject (reference "Requirements::PhysicalRequirementCheck::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind "subject") (name "") (range (start (line 141) (character 2)) (end (line 141) (character 16))) (parent (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (authored (relationships (typing (reference "Part") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 134) (character 1)) (end (line 134) (character 232))) (parent (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind "requirement def") (name "RequirementCheck") (declared-name "RequirementCheck") (range (start (line 48) (character 1)) (end (line 48) (character 1648))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementConstraintCheck") (range (start (line 48) (character 46)) (end (line 48) (character 72)))) (subject (reference "Requirements::RequirementCheck::subj") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 48) (character 1)) (end (line 48) (character 1648))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind "subject") (name "subj") (declared-name "subj") (range (start (line 57) (character 2)) (end (line 57) (character 139))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (authored (relationships (typing (reference "Anything") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind "requirement") (name "subrequirements") (declared-name "subrequirements") (range (start (line 89) (character 2)) (end (line 89) (character 168))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "requirementChecks") (range (start (line 89) (character 48)) (end (line 89) (character 65)))) (subsetting (reference "constraints") (range (start (line 89) (character 67)) (end (line 89) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements::_documentation"))) (kind "documentation") (name "") (range (start (line 89) (character 2)) (end (line 89) (character 168))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (kind "constraint def") (name "RequirementConstraintCheck") (declared-name "RequirementConstraintCheck") (range (start (line 18) (character 1)) (end (line 18) (character 829))) (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 1)) (end (line 18) (character 829))) (parent (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 7) (character 1)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Requirements::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 5466))) (parent (node (document "d0") (qualified-name "Requirements"))))
    (element (id (node (document "d0") (qualified-name "Requirements::allTrue"))) (kind "import") (name "allTrue") (declared-name "allTrue") (range (start (line 8) (character 1)) (end (line 8) (character 42))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::allTrue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (kind "import") (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks") (range (start (line 10) (character 1)) (end (line 10) (character 54))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::assertedConstraintChecks") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 53))))))
    (element (id (node (document "d0") (qualified-name "Requirements::assumptions"))) (kind "constraint") (name "assumptions") (declared-name "assumptions") (range (start (line 86) (character 2)) (end (line 86) (character 69))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind "concern") (name "concernChecks") (declared-name "concernChecks") (range (start (line 186) (character 1)) (end (line 186) (character 165))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConcernCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirements::concernChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 186) (character 1)) (end (line 186) (character 165))) (parent (node (document "d0") (qualified-name "Requirements::concernChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (kind "import") (name "constraintChecks") (declared-name "constraintChecks") (range (start (line 9) (character 1)) (end (line 9) (character 46))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::constraintChecks") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Requirements::constraints"))) (kind "constraint") (name "constraints") (declared-name "constraints") (range (start (line 87) (character 2)) (end (line 87) (character 69))) (parent (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
    (element (id (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (kind "import") (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks") (range (start (line 11) (character 1)) (end (line 11) (character 53))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Constraints::negatedConstraintChecks") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 52))))))
    (element (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind "requirement") (name "notSatisfiedRequirementChecks") (declared-name "notSatisfiedRequirementChecks") (range (start (line 179) (character 1)) (end (line 179) (character 260))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementCheck") (range none)) (subsetting (reference "requirementChecks") (range (start (line 179) (character 79)) (end (line 179) (character 96)))) (subsetting (reference "negatedConstraintChecks") (range (start (line 179) (character 98)) (end (line 179) (character 121)))))))
    (element (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 179) (character 1)) (end (line 179) (character 260))) (parent (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::parts"))) (kind "import") (name "parts") (declared-name "parts") (range (start (line 13) (character 1)) (end (line 13) (character 29))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind "requirement") (name "requirementChecks") (declared-name "requirementChecks") (range (start (line 165) (character 1)) (end (line 165) (character 185))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementCheck") (range none)) (subsetting (reference "constraintChecks") (range (start (line 165) (character 77)) (end (line 165) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "Requirements::requirementChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 165) (character 1)) (end (line 165) (character 185))) (parent (node (document "d0") (qualified-name "Requirements::requirementChecks"))))
    (element (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind "requirement") (name "satisfiedRequirementChecks") (declared-name "satisfiedRequirementChecks") (range (start (line 172) (character 1)) (end (line 172) (character 227))) (parent (node (document "d0") (qualified-name "Requirements"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "requirementChecks") (range (start (line 172) (character 52)) (end (line 172) (character 69)))) (subsetting (reference "assertedConstraintChecks") (range (start (line 172) (character 71)) (end (line 172) (character 95)))))))
    (element (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 172) (character 1)) (end (line 172) (character 227))) (parent (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (range (start (line 14) (character 16)) (end (line 14) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 16)) (end (line 6) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Attributes::AttributeValue") (range (start (line 16) (character 16)) (end (line 16) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 144) (character 42)) (end (line 144) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::DesignConstraintCheck::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 105) (character 47)) (end (line 105) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::FunctionalRequirementCheck::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Interface"))) (kind membershipImport) (ordinal 0)) (authored-target "Interfaces::Interface") (range (start (line 15) (character 16)) (end (line 15) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 114) (character 46)) (end (line 114) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::InterfaceRequirementCheck::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Interface") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Interface")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (range (start (line 12) (character 16)) (end (line 12) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 124) (character 48)) (end (line 124) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::PerformanceRequirementCheck::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "AttributeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::AttributeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 134) (character 45)) (end (line 134) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::PhysicalRequirementCheck::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind featureTyping) (ordinal 0)) (authored-target "Part") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementConstraintCheck") (range (start (line 48) (character 46)) (end (line 48) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirements::RequirementCheck::subj") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (range (start (line 89) (character 48)) (end (line 89) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1)) (authored-target "constraints") (range (start (line 89) (character 67)) (end (line 89) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::constraints")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 7) (character 16)) (end (line 7) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::allTrue"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::allTrue") (range (start (line 8) (character 16)) (end (line 8) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::assertedConstraintChecks") (range (start (line 10) (character 16)) (end (line 10) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "ConcernCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::ConcernCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::constraintChecks") (range (start (line 9) (character 16)) (end (line 9) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Constraints::negatedConstraintChecks") (range (start (line 11) (character 16)) (end (line 11) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (range (start (line 179) (character 79)) (end (line 179) (character 96))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)) (authored-target "negatedConstraintChecks") (range (start (line 179) (character 98)) (end (line 179) (character 121))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (range (start (line 13) (character 16)) (end (line 13) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "constraintChecks") (range (start (line 165) (character 77)) (end (line 165) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::constraintChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)) (authored-target "requirementChecks") (range (start (line 172) (character 52)) (end (line 172) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)) (authored-target "assertedConstraintChecks") (range (start (line 172) (character 71)) (end (line 172) (character 95))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Action"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Interface"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Interface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (target (node (document "d0") (qualified-name "Requirements::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "d0") (qualified-name "Requirements::Anything"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (target (node (document "d0") (qualified-name "Requirements::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (target (node (document "d0") (qualified-name "Requirements::constraints"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (target (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (target (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
