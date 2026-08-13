# META
~~~ini
description=SysML Example (Analysis): Turbojet Stage Analysis
type=file
~~~
# SOURCE
~~~sysml
package 'Turbojet Stage Analysis' {
	private import Quantities::ScalarQuantityValue;
	private import MeasurementReferences::DimensionOneValue;
	private import ISQ::*;
	
	package 'Thermodynamic Functions' {
	    calc def 'Ideal Gas Law' { in rho; in R_bar; in T;
	    	return p = rho * R_bar * T;
	    }
	    
	    calc def 'Reversible Adiabatic Compression Density' { in rho_1; in p_1; in p_2; in gamma;
	    	return rho_2 = rho_1 * (p_2 / p_1)^(1/gamma);
	    }
	    
	    calc def 'Reversible Adiabatic Compression Temperature' { in T_1; in p_1; in p_2; in gamma;
	    	return T_2 = T_1 * (p_2 / p_1)**((gamma - 1) / gamma);
	    }
	    
	    calc def 'Total Pressure' { in P_static; in rho; in V;
	    	1/2 * rho * V^2 + P_static
	    }
	    
	    // Showing explicit parameter typing
	    calc def 'Total Temperature' { in T_static : TemperatureValue; in Cp : DimensionOneValue; in V : VolumeValue;
	    	return : TemperatureValue = 1/(2 * Cp) * V^2 + T_static;
	    }
	    
	    calc def 'Total Enthalpy' { in h_total; in h_static; in V;
	    	return H_total = 1/2 * V^2 + h_static;
	    }
	}
	
	package 'Thermodynamics Structure' {
	    part def 'Ideal Gas Parcel' {
	        comment
	            /*
	            The parcel is an infinitesimal volume used to analyze points in a flow
	            */
	        attribute 'Molar Mass';
	        attribute 'Density';
	        attribute 'Pressure';
	        attribute 'Temperature';
	        attribute 'Enthalpy';
	        attribute 'Specific Gas Constant';
	    }
	    
	    part def 'Moving Ideal Gas Parcel' specializes 'Ideal Gas Parcel' {
	        comment about 'Stagnation Pressure'
	            /*
	            Stagnation pressure is the pressure of the parcel if the kinetic energy defined by its
	            velocity in a given coordinate frame is converted to gas internal energy through deceleration
	            to a velocity that matches the current frame.
	            */
	        attribute 'Stagnation Pressure';
	        attribute 'Stagnation Temperature';
	        attribute 'Stagnation Enthalpy';
	        
	        comment about 'Static Pressure'
	            /*
	            Static pressure is the pressure of the parcel as it moves
	            */
	        attribute 'Static Pressure' redefines 'Ideal Gas Parcel'::'Pressure';
	        attribute 'Static Temperature' redefines 'Ideal Gas Parcel'::'Temperature';
	        attribute 'Static Enthalpy' redefines 'Ideal Gas Parcel'::'Enthalpy';
	    }
	    
	    action def 'Thermodynamic Process'; // need start and end shots to show beginning and end attributes
	    
	    action def 'Adiabatic Process' specializes 'Thermodynamic Process' {
	        /*
	        Thermodynamic process typically have their states defined at beginning and end
	        of the process (since these starts are path-independent)
	        */
	        action 'Stage 1' :>> start;
	        action 'Stage 2' :>> done;
	    }
	    
	    action def 'Reversible Adiabatic Process' specializes 'Adiabatic Process';
	}
	
	package 'Low-Pressure Compressor Analysis' {
	    
	    part 'Analysis Context' {
	        private import 'Thermodynamic Functions'::*;
	        
	        part 'Inlet Gas' : 'Thermodynamics Structure'::'Moving Ideal Gas Parcel' {
	        	// Explicit binding notation
	        	calc 'Solve for Pressure1' : 'Ideal Gas Law';
	        	bind 'Density' = 'Solve for Pressure1'.rho;
	        	bind 'Specific Gas Constant' = 'Solve for Pressure1'.R_bar;
	        	bind 'Static Temperature' = 'Solve for Pressure1'.T;
	        	bind 'Static Pressure' = 'Solve for Pressure1'.p;	        	
	        	
	        	// Shorthand parameter binding notation
	            calc 'Solve for Pressure2' : 'Ideal Gas Law' {
	                in rho = 'Density';
	                in R_bar = 'Specific Gas Constant';
	                in T = 'Static Temperature';
				}				
				            
	            // Invocation expression notation
	            attribute :>> 'Static Pressure' = 'Ideal Gas Law'('Density', 'Specific Gas Constant', 'Static Temperature');

	            // Equation as a constraint (note "==")
	            constraint { 'Static Pressure' == 'Ideal Gas Law'('Density', 'Specific Gas Constant', 'Static Temperature') }
	        }
	    }
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/turbojet_stage_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 5) (end 8 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 5) (end 12 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 5) (end 16 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 5) (end 20 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 5) (end 25 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 5) (end 29 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 47) (end 61 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 50) (end 62 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 47) (end 63 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 30) (end 73 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 30) (end 74 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 28) (end 85 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 87 10) (end 87 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 88 10) (end 88 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 89 10) (end 89 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 90 10) (end 90 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 91 10) (end 91 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 94 13) (end 98 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 27) (end 101 44))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 104 13) (end 105 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:320de94f6327b2bbe79d44377e84cb7b43b26059601b1470058e66918e50ceb3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::DimensionOneValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Thermodynamic Functions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermodynamics Structure::Moving Ideal Gas Parcel"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Static Pressure"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Thermodynamic Process"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "start"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "done"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Enthalpy"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Pressure"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Temperature"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Ideal Gas Parcel::Enthalpy"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Ideal Gas Parcel::Pressure"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Ideal Gas Parcel::Temperature"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Adiabatic Process"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (kind action-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Thermodynamic Functions")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "Static Pressure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0))
      (authored-target "Thermodynamic Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind redefinition) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind redefinition) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind redefinition) (ordinal 0))
      (authored-target "Ideal Gas Parcel::Enthalpy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind redefinition) (ordinal 0))
      (authored-target "Ideal Gas Parcel::Pressure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind redefinition) (ordinal 0))
      (authored-target "Ideal Gas Parcel::Temperature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0))
      (authored-target "Adiabatic Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 1 16) (end 1 47)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 2 16) (end 2 56)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 83 24) (end 83 52)) (probe (position 83 24))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Thermodynamic Functions")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 85 28) (end 85 81)) (probe (position 85 28))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0) (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 27) (end 101 44)) (probe (position 101 27))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "Static Pressure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 68 48) (end 68 71)) (probe (position 68 48))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0) (authored-target "Thermodynamic Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 73 30) (end 73 35)) (probe (position 73 30))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind redefinition) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 74 30) (end 74 34)) (probe (position 74 30))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind redefinition) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 63 47) (end 63 77)) (probe (position 63 47))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Enthalpy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 61 47) (end 61 77)) (probe (position 61 47))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Pressure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 62 50) (end 62 83)) (probe (position 62 50))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Temperature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 77 59) (end 77 78)) (probe (position 77 59))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0) (authored-target "Adiabatic Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
  )
)
~~~
