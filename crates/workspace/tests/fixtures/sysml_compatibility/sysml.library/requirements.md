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
    doc /*
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
        doc /*
		 * RequirementConstraintCheck is the base ConstraintCheck for RequirementCheck, defining the
		 * separate assumptions and required constraints such that, if all the assumptions are true,
		 * then all the required constraints must be true.
		 */

        constraint assumptions[0..*] :> constraintChecks, subperformances {
            doc /*
			 * Assumptions that must hold for the required constraints to apply.
			 */
        }

        constraint constraints[0..*] :> constraintChecks, subperformances {
            doc /*
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
        doc /*
		 * RequirementCheck is the most general class for requirements checking. RequirementsCheck is the base
		 * type of all requirement definitions.
		 */

        ref requirement :>> self : RequirementCheck;

        subject subj : Anything [1] {
            doc /*
			 * The entity that is being checked for satisfaction of the required constraints.
			 */
        }

        ref part actors : Part [0..*] {
            doc /*
			 * The Parts that fill the role of actors for this RequirementCheck.
			 * (Note: This is not itself an actor parameter, because specific actor
			 * parameters will be added for specific RequirementChecks.)
			 */
        }

        ref part stakeholders : Part [0..*] {
            doc /*
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

        abstract requirement subrequirements :> requirementChecks, constraints [0..*] {
            doc /*
			 * Nested requirements, which are also required constraints.
			 */
        }

        abstract concern concerns :> concernChecks, subrequirements [0..*] {
            doc /*
			 * The checks of any concerns being addressed (as required constraints).
			 */
        }
    }

    requirement def FunctionalRequirementCheck :> RequirementCheck {
        doc /*
		 * A functional requirement specifies an action that a system, or part of a system, must perform.
		 */

        subject : Action;
    }

    requirement def InterfaceRequirementCheck :> RequirementCheck {
        doc /*
		 * An interface requirement specifies an interface for connecting systems and system parts, which
		 * optionally may include item flows across the interface and/or interface constraints.
		 */

        subject : Interface;
    }

    requirement def PerformanceRequirementCheck :> RequirementCheck {
        doc /*
		 * A performance requirement quantitavely measures the extent to which a system, or a system part, 
		 * satisfies a required capability or condition.
		 */

        subject : AttributeValue;
    }

    requirement def PhysicalRequirementCheck :> RequirementCheck {
        doc /*
		 * A physical requirement specifies physical characteristics and/or physical constraints of the 
		 * system, or a system part.
		 */

        subject : Part;
    }

    requirement def DesignConstraintCheck :> RequirementCheck {
        doc /*
		 * A design constraint specifies a constraint on the implementation of the system or system part, 
		 * such as the system must use a commercial off the shelf component.
		 */

        subject : Part;
    }

    concern def ConcernCheck :> RequirementCheck {
        doc /*
		 * ConcernCheck is the most general class for concern checking. ConcernCheck is the base type of 
		 * all ConcernDefinitions.
		 */

        ref concern :>> self : ConcernCheck;
    }

    abstract requirement requirementChecks : RequirementCheck :> constraintChecks [0..*] nonunique {
        doc /*
		 * requirementChecks is the base feature of all requirement usages.
		 */
    }

    abstract requirement satisfiedRequirementChecks :> requirementChecks, assertedConstraintChecks {
        doc /*
		 * satisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be satisfied.
		 */
    }

    abstract requirement notSatisfiedRequirementChecks : RequirementCheck :> requirementChecks, negatedConstraintChecks [0..*] {
        doc /*
		 * notSatisfiedRequirementChecks is the subset of requirementChecks for Requirements asserted to be not satisfied.
		 */
    }

    abstract concern concernChecks : ConcernCheck :> requirementChecks [0..*] nonunique {
        doc /*
		 * concernChecks is the base feature of all ConcernUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Requirements'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'ControlFunctions::allTrue'[unresolved])
      (membership_import private -> 'Constraints::constraintChecks'[unresolved])
      (membership_import private -> 'Constraints::assertedConstraintChecks'[unresolved])
      (membership_import private -> 'Constraints::negatedConstraintChecks'[unresolved])
      (membership_import private -> 'Parts::Part'[unresolved])
      (membership_import private -> 'Parts::parts'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Interfaces::Interface'[unresolved])
      (membership_import private -> 'Attributes::AttributeValue'[unresolved])
      (constraint_def abstract 'RequirementConstraintCheck'
        (documentation)
        (constraint_usage composite 'assumptions' :> 'constraintChecks'[unresolved] :> 'subperformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (constraint_usage composite 'constraints' :> 'constraintChecks'[unresolved] :> 'subperformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (return_parameter_membership
          (feature_def out 'result'
            (feature_value (=))
            (documentation))))
      (requirement_def abstract 'RequirementCheck' :> 'Requirements::RequirementConstraintCheck'[constraint_def]
        (documentation)
        (requirement_usage reference :>> 'self'[unresolved] : 'Requirements::RequirementCheck'[requirement_def] :> 'Requirements::requirementChecks'[requirement_usage][implied])
        (subject_membership in 'subj' : 'Anything'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (part_usage reference 'actors' : 'Part'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (part_usage reference 'stakeholders' : 'Part'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (constraint_usage composite 'assumptions' :>> 'Requirements::RequirementConstraintCheck::assumptions'[constraint_usage])
        (constraint_usage composite 'constraints' :>> 'Requirements::RequirementConstraintCheck::constraints'[constraint_usage])
        (requirement_usage abstract composite 'subrequirements' :> 'Requirements::requirementChecks'[requirement_usage] :> 'Requirements::RequirementCheck::constraints'[constraint_usage]
          (multiplicity_range [0..*])
          (documentation))
        (concern_usage abstract composite 'concerns' :> 'Requirements::concernChecks'[concern_usage] :> 'Requirements::RequirementCheck::subrequirements'[requirement_usage]
          (multiplicity_range [0..*])
          (documentation)))
      (requirement_def 'FunctionalRequirementCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (subject_membership in : 'Action'[unresolved] :>> 'Requirements::RequirementCheck::subj'[subject_membership][implied]))
      (requirement_def 'InterfaceRequirementCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (subject_membership in : 'Interface'[unresolved] :>> 'Requirements::RequirementCheck::subj'[subject_membership][implied]))
      (requirement_def 'PerformanceRequirementCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (subject_membership in : 'AttributeValue'[unresolved] :>> 'Requirements::RequirementCheck::subj'[subject_membership][implied]))
      (requirement_def 'PhysicalRequirementCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (subject_membership in : 'Part'[unresolved] :>> 'Requirements::RequirementCheck::subj'[subject_membership][implied]))
      (requirement_def 'DesignConstraintCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (subject_membership in : 'Part'[unresolved] :>> 'Requirements::RequirementCheck::subj'[subject_membership][implied]))
      (concern_def 'ConcernCheck' :> 'Requirements::RequirementCheck'[requirement_def]
        (documentation)
        (concern_usage reference :>> 'self'[unresolved] : 'Requirements::ConcernCheck'[concern_def]))
      (requirement_usage abstract 'requirementChecks' : 'Requirements::RequirementCheck'[requirement_def] :> 'constraintChecks'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (requirement_usage abstract 'satisfiedRequirementChecks' :> 'Requirements::requirementChecks'[requirement_usage] :> 'assertedConstraintChecks'[unresolved]
        (documentation))
      (requirement_usage abstract 'notSatisfiedRequirementChecks' : 'Requirements::RequirementCheck'[requirement_def] :> 'Requirements::requirementChecks'[requirement_usage] :> 'negatedConstraintChecks'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (concern_usage abstract 'concernChecks' : 'Requirements::ConcernCheck'[concern_def] :> 'Requirements::requirementChecks'[requirement_usage]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
