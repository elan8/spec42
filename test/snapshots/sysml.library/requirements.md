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
  (document "memory://snapshot/requirements.md"
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
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 26 2) (end 31 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 33 2) (end 38 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 40 2) (end 40 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 40 9) (end 40 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 40 16) (end 46 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 55 2) (end 55 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 57 2) (end 62 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 64 2) (end 71 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 73 2) (end 80 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 86 2) (end 86 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 87 2) (end 87 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 67) (end 89 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 96 2) (end 101 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 111 2) (end 111 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 121 2) (end 121 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 131 2) (end 131 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 141 2) (end 141 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 151 2) (end 151 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 161 2) (end 161 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 165 77) (end 165 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 172 71) (end 172 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 98) (end 179 121))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d88ac67d997f213f4f536ab0338a318b2b9d53074d455c67debf6f3ac07173da") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::allTrue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Constraints::constraintChecks") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Constraints::assertedConstraintChecks") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Constraints::negatedConstraintChecks") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Interfaces::Interface") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Attributes::AttributeValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (kind concern-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (subsetting (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::DesignConstraintCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementConstraintCheck"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "requirementChecks")) (subsetting (reference "constraints"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck"))) (kind constraint-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind concern) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConcernCheck")) (subsetting (reference "requirementChecks"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")) (subsetting (reference "requirementChecks")) (subsetting (reference "negatedConstraintChecks"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementCheck")) (subsetting (reference "constraintChecks"))))
    (declaration (id (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "requirementChecks")) (subsetting (reference "assertedConstraintChecks"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Constraints::constraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Constraints::assertedConstraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Constraints::negatedConstraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Interfaces::Interface")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Attributes::AttributeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (kind subsetting) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0))
      (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1))
      (authored-target "constraints")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConcernCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1))
      (authored-target "negatedConstraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "constraintChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1))
      (authored-target "assertedConstraintChecks")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::DesignConstraintCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::FunctionalRequirementCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::InterfaceRequirementCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PerformanceRequirementCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PhysicalRequirementCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirements.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 7 16) (end 7 36)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 9 16) (end 9 45)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Constraints::constraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 10 16) (end 10 53)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Constraints::assertedConstraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 11 16) (end 11 52)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Constraints::negatedConstraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 12 16) (end 12 27)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 13 16) (end 13 28)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 14 16) (end 14 31)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 15 16) (end 15 37)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Interfaces::Interface")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 16 16) (end 16 42)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Attributes::AttributeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 154 29) (end 154 45)) (probe (position 154 29))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (kind subsetting) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 144 42) (end 144 58)) (probe (position 144 42))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::DesignConstraintCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 105 47) (end 105 63)) (probe (position 105 47))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::FunctionalRequirementCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 114 46) (end 114 62)) (probe (position 114 46))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::InterfaceRequirementCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 124 48) (end 124 64)) (probe (position 124 48))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PerformanceRequirementCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 134 45) (end 134 61)) (probe (position 134 45))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::PhysicalRequirementCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 48 46) (end 48 72)) (probe (position 48 46))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementConstraintCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 89 48) (end 89 65)) (probe (position 89 48))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 89 67) (end 89 78)) (probe (position 89 67))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements"))) (kind subsetting) (ordinal 1) (authored-target "constraints")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 186 33) (end 186 45)) (probe (position 186 33))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind featureTyping) (ordinal 0) (authored-target "ConcernCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::ConcernCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 186 65) (end 186 82)) (probe (position 186 65))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::concernChecks"))) (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 179 53) (end 179 69)) (probe (position 179 53))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 179 79) (end 179 96)) (probe (position 179 79))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 179 98) (end 179 121)) (probe (position 179 98))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (kind subsetting) (ordinal 1) (authored-target "negatedConstraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 165 41) (end 165 57)) (probe (position 165 41))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::RequirementCheck")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 165 77) (end 165 93)) (probe (position 165 77))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks"))) (kind subsetting) (ordinal 0) (authored-target "constraintChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 172 52) (end 172 69)) (probe (position 172 52))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::requirementChecks")))))
  )
  (query (document "memory://snapshot/requirements.md") (range (start 172 71) (end 172 95)) (probe (position 172 71))
    (reference (id (source (node (document "memory://snapshot/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (kind subsetting) (ordinal 1) (authored-target "assertedConstraintChecks")
      (outcome (status unresolved)))
  )
)
~~~
