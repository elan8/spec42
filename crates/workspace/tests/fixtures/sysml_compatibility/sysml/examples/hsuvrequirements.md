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
# FORMAT
~~~sysml
package HSUVRequirements {
    private import Requirements::*;

    requirement <'UR1.1'> Load : FunctionalRequirementCheck {
        // The following requirements are composite sub-requirements.
        requirement Passengers;
        requirement FuelCapacity;
        requirement Cargo;
    }

    requirement <'UR1.2'> EcoFriendliness : PerformanceRequirementCheck {
        requirement <'URI1.2.1'> Emissions : PerformanceRequirementCheck {
            /* The car shall meet 2010 Kyoto Accord emissions standards. */
        }
    }

    requirement <'UR1.3'> Performance : PerformanceRequirementCheck {
        requirement Acceleration;
        requirement <'UR1.3.1'> FuelEconomy : PerformanceRequirementCheck {
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
        require constraint Load;
        require constraint EcoFriendliness;
        require constraint Performance;
        require constraint Ergonomics;
    }
}
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
# SMG
~~~
(model
  (namespace
    (package 'HSUVRequirements'
      (namespace_import private -> 'Requirements'[unresolved])
      (requirement_usage 'Load' : 'FunctionalRequirementCheck'[unresolved]
        (requirement_usage composite 'Passengers')
        (requirement_usage composite 'FuelCapacity')
        (requirement_usage composite 'Cargo'))
      (requirement_usage 'EcoFriendliness' : 'PerformanceRequirementCheck'[unresolved]
        (requirement_usage composite 'Emissions' : 'PerformanceRequirementCheck'[unresolved]))
      (requirement_usage 'Performance' : 'PerformanceRequirementCheck'[unresolved]
        (requirement_usage composite 'Acceleration')
        (requirement_usage composite 'FuelEconomy' : 'PerformanceRequirementCheck'[unresolved])
        (requirement_usage composite 'Braking')
        (requirement_usage composite 'Range')
        (requirement_usage composite 'Power'))
      (requirement_usage 'Ergonomics')
      (requirement_usage 'HybridSUVSpec'
        (require_constraint_usage composite 'Load')
        (require_constraint_usage composite 'EcoFriendliness')
        (require_constraint_usage composite 'Performance')
        (require_constraint_usage composite 'Ergonomics')))))
~~~
