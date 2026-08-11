# META
~~~ini
description=SysML Example (Requirements): HSUVRequirements
type=file
~~~
# SOURCE
~~~sysml
package HSUVRequirements {
	private import Requirements::*;
	
	requirement <'UR1.1'> Load: FunctionalRequirementCheck {
		// The following requirements are composite sub-requirements.
		requirement Passengers;
		requirement FuelCapacity;
		requirement Cargo;
	}
	
	requirement <'UR1.2'> EcoFriendliness: PerformanceRequirementCheck {
		requirement <'URI1.2.1'> Emissions: PerformanceRequirementCheck {
			/* The car shall meet 2010 Kyoto Accord emissions standards. */
		}
	}
	
	requirement <'UR1.3'> Performance: PerformanceRequirementCheck {
		requirement Acceleration;
		requirement <'UR1.3.1'> FuelEconomy: PerformanceRequirementCheck {
			/* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
		}
		requirement Braking;
		requirement Range;
		requirement Power;
	}
	
	requirement <'UR1.4'> Ergonomics;
	
	// Syntactically, should this be explicitly marked as a "group"?
	requirement HybridSUVSpec {		
		// The following requirements are required by reference.
		require Load;
		require EcoFriendliness;
		require Performance;
		require Ergonomics;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "hsuvrequirements.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 1) (end 3 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 1) (end 10 211))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 138))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 1) (end 16 340))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 178))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
LineComment,
KwRequirement,Ident,Semicolon,
KwRequirement,Ident,Semicolon,
KwRequirement,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwRequirement,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwRequirement,Ident,Semicolon,
KwRequirement,Ident,Semicolon,
KwRequirement,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Semicolon,
LineComment,
KwRequirement,Ident,OpenCurly,
LineComment,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'HSUVRequirements'
    (import_decl private 'Requirements::*')
    (requirement_usage 'Load' : 'FunctionalRequirementCheck'
      (line_comment)
      (requirement_usage 'Passengers')
      (requirement_usage 'FuelCapacity')
      (requirement_usage 'Cargo'))
    (requirement_usage 'EcoFriendliness' : 'PerformanceRequirementCheck'
      (requirement_usage 'Emissions' : 'PerformanceRequirementCheck'
        (comment)))
    (requirement_usage 'Performance' : 'PerformanceRequirementCheck'
      (requirement_usage 'Acceleration')
      (requirement_usage 'FuelEconomy' : 'PerformanceRequirementCheck'
        (comment))
      (requirement_usage 'Braking')
      (requirement_usage 'Range')
      (requirement_usage 'Power'))
    (requirement_usage 'Ergonomics')
    (line_comment)
    (requirement_usage 'HybridSUVSpec'
      (line_comment)
      (sysml_decl 'Load')
      (sysml_decl 'EcoFriendliness')
      (sysml_decl 'Performance')
      (sysml_decl 'Ergonomics'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'FunctionalRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'FunctionalRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
semantic.unresolved_name 'PerformanceRequirementCheck'
~~~
# FORMAT
~~~sysml
package HSUVRequirements {
    private import Requirements::*;

    requirement <'UR1.1'> Load: FunctionalRequirementCheck {
        // The following requirements are composite sub-requirements.
        requirement Passengers;
        requirement FuelCapacity;
        requirement Cargo;
    }

    requirement <'UR1.2'> EcoFriendliness: PerformanceRequirementCheck {
        requirement <'URI1.2.1'> Emissions: PerformanceRequirementCheck {
            /* The car shall meet 2010 Kyoto Accord emissions standards. */
        }
    }

    requirement <'UR1.3'> Performance: PerformanceRequirementCheck {
        requirement Acceleration;
        requirement <'UR1.3.1'> FuelEconomy: PerformanceRequirementCheck {
            /* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
        }
        requirement Braking;
        requirement Range;
        requirement Power;
    }

    requirement <'UR1.4'> Ergonomics;

    // Syntactically, should this be explicitly marked as a "group"?
    requirement HybridSUVSpec {
        // The following requirements are required by reference.
        require Load;
        require EcoFriendliness;
        require Performance;
        require Ergonomics;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6107ba67270f3a4a9ea70a3b80dc45d365735418a8b447ec0b7a76336b0b835f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "HSUVRequirements"))) (kind "package") (name "HSUVRequirements") (declared-name "HSUVRequirements") (range (start (line 0) (character 0)) (end (line 0) (character 1106))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind "requirement") (name "EcoFriendliness") (declared-name "EcoFriendliness") (range (start (line 10) (character 1)) (end (line 10) (character 211))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind "requirement") (name "Emissions") (declared-name "Emissions") (range (start (line 11) (character 2)) (end (line 11) (character 138))) (parent (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Ergonomics"))) (kind "requirement") (name "Ergonomics") (declared-name "Ergonomics") (range (start (line 28) (character 1)) (end (line 28) (character 34))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (kind "requirement") (name "HybridSUVSpec") (declared-name "HybridSUVSpec") (range (start (line 31) (character 1)) (end (line 31) (character 180))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 33) (character 2)) (end (line 33) (character 15))) (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_1"))) (kind "require constraint") (name "_requireConstraint_1") (declared-name "_requireConstraint_1") (range (start (line 34) (character 2)) (end (line 34) (character 26))) (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_2"))) (kind "require constraint") (name "_requireConstraint_2") (declared-name "_requireConstraint_2") (range (start (line 35) (character 2)) (end (line 35) (character 22))) (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_3"))) (kind "require constraint") (name "_requireConstraint_3") (declared-name "_requireConstraint_3") (range (start (line 36) (character 2)) (end (line 36) (character 21))) (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load"))) (kind "requirement") (name "Load") (declared-name "Load") (range (start (line 3) (character 1)) (end (line 3) (character 199))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "FunctionalRequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::Cargo"))) (kind "requirement") (name "Cargo") (declared-name "Cargo") (range (start (line 7) (character 2)) (end (line 7) (character 20))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::FuelCapacity"))) (kind "requirement") (name "FuelCapacity") (declared-name "FuelCapacity") (range (start (line 6) (character 2)) (end (line 6) (character 27))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::Passengers"))) (kind "requirement") (name "Passengers") (declared-name "Passengers") (range (start (line 5) (character 2)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (kind "requirement") (name "Performance") (declared-name "Performance") (range (start (line 16) (character 1)) (end (line 16) (character 340))) (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Acceleration"))) (kind "requirement") (name "Acceleration") (declared-name "Acceleration") (range (start (line 17) (character 2)) (end (line 17) (character 27))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Braking"))) (kind "requirement") (name "Braking") (declared-name "Braking") (range (start (line 23) (character 2)) (end (line 23) (character 22))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind "requirement") (name "FuelEconomy") (declared-name "FuelEconomy") (range (start (line 18) (character 2)) (end (line 18) (character 178))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Power"))) (kind "requirement") (name "Power") (declared-name "Power") (range (start (line 25) (character 2)) (end (line 25) (character 20))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Range"))) (kind "requirement") (name "Range") (declared-name "Range") (range (start (line 24) (character 2)) (end (line 24) (character 20))) (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirements::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Load"))) (kind featureTyping) (ordinal 0)) (authored-target "FunctionalRequirementCheck") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
