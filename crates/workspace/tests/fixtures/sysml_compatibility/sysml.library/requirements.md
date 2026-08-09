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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Requirements"))) (name "Requirements") (declared-name "Requirements")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::AttributeValue"))) (name "AttributeValue") (declared-name "AttributeValue"))
        (element (kind "concern def") (id (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (name "ConcernCheck") (declared-name "ConcernCheck")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::ConcernCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::ConcernCheck")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (name "DesignConstraintCheck") (declared-name "DesignConstraintCheck")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (name "FunctionalRequirementCheck") (declared-name "FunctionalRequirementCheck")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::Interface"))) (name "Interface") (declared-name "Interface"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (name "InterfaceRequirementCheck") (declared-name "InterfaceRequirementCheck")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::Part"))) (name "Part") (declared-name "Part"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (name "PerformanceRequirementCheck") (declared-name "PerformanceRequirementCheck")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (name "PhysicalRequirementCheck") (declared-name "PhysicalRequirementCheck")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (name "RequirementCheck") (declared-name "RequirementCheck")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (name "subj") (declared-name "subj") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (name "subrequirements") (declared-name "subrequirements") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
              )
            )
            (element (kind "constraint") (id (node (document "d0") (qualified-name "Requirements::assumptions"))) (name "assumptions") (declared-name "assumptions") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
            (element (kind "constraint") (id (node (document "d0") (qualified-name "Requirements::constraints"))) (name "constraints") (declared-name "constraints") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
          )
        )
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (name "RequirementConstraintCheck") (declared-name "RequirementConstraintCheck")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::String"))) (name "String") (declared-name "String"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::allTrue"))) (name "allTrue") (declared-name "allTrue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (name "assertedConstraintChecks") (declared-name "assertedConstraintChecks"))
        (element (kind "concern") (id (node (document "d0") (qualified-name "Requirements::concernChecks"))) (name "concernChecks") (declared-name "concernChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::concernChecks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::ConcernCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (name "constraintChecks") (declared-name "constraintChecks"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (name "negatedConstraintChecks") (declared-name "negatedConstraintChecks"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (name "notSatisfiedRequirementChecks") (declared-name "notSatisfiedRequirementChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirements::parts"))) (name "parts") (declared-name "parts"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (name "requirementChecks") (declared-name "requirementChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::requirementChecks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirements::RequirementCheck")))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (name "satisfiedRequirementChecks") (declared-name "satisfiedRequirementChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks::_documentation"))) (name ""))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::ConcernCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::_documentation"))) (to (node (document "d0") (qualified-name "Requirements"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::concernChecks::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::concernChecks"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::requirementChecks::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks::_documentation"))) (to (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (to (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (to (node (document "d0") (qualified-name "Requirements::constraints"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::negatedConstraintChecks"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::constraintChecks"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::assertedConstraintChecks"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirements::concernChecks"))) (to (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (to (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::ConcernCheck"))) (status missing-prerequisite) (target "Requirements::ConcernCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::DesignConstraintCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::FunctionalRequirementCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::InterfaceRequirementCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::PerformanceRequirementCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::PhysicalRequirementCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::RequirementCheck"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::RequirementConstraintCheck"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::assumptions"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::concernChecks"))) (status missing-prerequisite) (target "Requirements::concernChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::constraints"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::requirementChecks"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirements::satisfiedRequirementChecks"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/requirements.md"
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
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 48 1) (end 48 1648))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 2) (end 57 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 2) (end 111 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 2) (end 121 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 2) (end 131 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 141 2) (end 141 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 2) (end 151 16))
      )
    )
  )
)
~~~
