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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 12) (end 19 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 18) (end 19 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 24) (end 19 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 50) (end 23 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 76) (end 23 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 102) (end 23 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 15) (end 24 31))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 15) (end 88 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 15) (end 89 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 15) (end 90 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 15) (end 91 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 26) (end 95 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 28) (end 96 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 24) (end 97 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 27) (end 101 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 63) (end 101 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 74) (end 101 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 99) (end 101 119))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:320de94f6327b2bbe79d44377e84cb7b43b26059601b1470058e66918e50ceb3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::DimensionOneValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Thermodynamic Functions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thermodynamics Structure::Moving Ideal Gas Parcel"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "Density")) (memberAccessOperand (reference "Solve for Pressure1::rho"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "Specific Gas Constant")) (memberAccessOperand (reference "Solve for Pressure1::R_bar"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "Static Temperature")) (memberAccessOperand (reference "Solve for Pressure1::T"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "Static Pressure")) (memberAccessOperand (reference "Solve for Pressure1::p"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Static Pressure")) (expressionOperand (reference "Density")) (expressionOperand (reference "Specific Gas Constant")) (expressionOperand (reference "Static Temperature")) (invocationCallee (reference "Ideal Gas Law"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure1"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ideal Gas Law"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ideal Gas Law"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::R_bar"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "Specific Gas Constant"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::T"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "Static Temperature"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::rho"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "Density"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "rho")) (expressionOperand (reference "R_bar")) (expressionOperand (reference "T"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "rho_1")) (expressionOperand (reference "p_2")) (expressionOperand (reference "p_1")) (expressionOperand (reference "gamma"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "T_1")) (expressionOperand (reference "p_2")) (expressionOperand (reference "p_1")) (expressionOperand (reference "gamma")) (expressionOperand (reference "gamma"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "V")) (expressionOperand (reference "h_static"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "rho")) (expressionOperand (reference "V")) (expressionOperand (reference "P_static"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue")) (expressionOperand (reference "Cp")) (expressionOperand (reference "V")) (expressionOperand (reference "T_static"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DimensionOneValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VolumeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Thermodynamic Process"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "start"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "done"))))
    (declaration (id (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (comment (text "\n\t            Stagnation pressure is the pressure of the parcel if the kinetic energy defined by its\n\t            velocity in a given coordinate frame is converted to gas internal energy through deceleration\n\t            to a velocity that matches the current frame.\n\t            ")) (comment (text "\n\t            Static pressure is the pressure of the parcel as it moves\n\t            "))))
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
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Thermodynamic Functions")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "Static Pressure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Density")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 2))
      (authored-target "Static Temperature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0))
      (authored-target "Density")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0))
      (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind bindSource) (ordinal 0))
      (authored-target "Static Temperature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind bindSource) (ordinal 0))
      (authored-target "Static Pressure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "Solve for Pressure1::rho")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "Solve for Pressure1::R_bar")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "Solve for Pressure1::T")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "Solve for Pressure1::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::R_bar"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::T"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Static Temperature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::rho"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Density")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 0))
      (authored-target "rho")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 1))
      (authored-target "R_bar")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 2))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 0))
      (authored-target "rho_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 1))
      (authored-target "p_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 2))
      (authored-target "p_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 3))
      (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 0))
      (authored-target "T_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 1))
      (authored-target "p_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 2))
      (authored-target "p_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 3))
      (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 4))
      (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 1))
      (authored-target "h_static")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 0))
      (authored-target "rho")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 1))
      (authored-target "V")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 2))
      (authored-target "P_static")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Cp")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 2))
      (authored-target "T_static")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static")))))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeValue")
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
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure1"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 4)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::R_bar"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::T"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::rho"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 1 16) (end 1 47)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 2 16) (end 2 56)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 83 24) (end 83 52)) (probe (position 83 24))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Thermodynamic Functions")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 85 28) (end 85 81)) (probe (position 85 28))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0) (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 27) (end 101 44)) (probe (position 101 27))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "Static Pressure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 63) (end 101 72)) (probe (position 101 63))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "Density")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 74) (end 101 97)) (probe (position 101 74))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 99) (end 101 119)) (probe (position 101 99))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 2) (authored-target "Static Temperature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 88 15) (end 88 24)) (probe (position 88 15))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0) (authored-target "Density")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 89 15) (end 89 38)) (probe (position 89 15))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0) (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 90 15) (end 90 35)) (probe (position 90 15))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind bindSource) (ordinal 0) (authored-target "Static Temperature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 91 15) (end 91 32)) (probe (position 91 15))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind bindSource) (ordinal 0) (authored-target "Static Pressure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 88 27) (end 88 52)) (probe (position 88 27))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "Solve for Pressure1::rho")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 89 41) (end 89 68)) (probe (position 89 41))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "Solve for Pressure1::R_bar")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 90 38) (end 90 61)) (probe (position 90 38))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "Solve for Pressure1::T")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 91 35) (end 91 58)) (probe (position 91 35))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind bind) (ordinal 3)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "Solve for Pressure1::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 101 47) (end 101 62)) (probe (position 101 47))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Low-Pressure Compressor Analysis")) (named (kind part) (name "Analysis Context")) (named (kind part) (name "Inlet Gas")) (anonymous (kind attribute) (ordinal 0)))))) (kind invocationCallee) (ordinal 0) (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 87 39) (end 87 54)) (probe (position 87 39))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure1"))) (kind featureTyping) (ordinal 0) (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 94 42) (end 94 57)) (probe (position 94 42))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2"))) (kind featureTyping) (ordinal 0) (authored-target "Ideal Gas Law")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 96 28) (end 96 51)) (probe (position 96 28))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::R_bar"))) (kind expressionOperand) (ordinal 0) (authored-target "Specific Gas Constant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 97 24) (end 97 44)) (probe (position 97 24))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::T"))) (kind expressionOperand) (ordinal 0) (authored-target "Static Temperature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 95 26) (end 95 35)) (probe (position 95 26))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Solve for Pressure2::rho"))) (kind expressionOperand) (ordinal 0) (authored-target "Density")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 7 17) (end 7 20)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 0) (authored-target "rho")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 7 23) (end 7 28)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 1) (authored-target "R_bar")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 7 31) (end 7 32)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::p"))) (kind expressionOperand) (ordinal 2) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 11 21) (end 11 26)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 0) (authored-target "rho_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 11 30) (end 11 33)) (probe (position 11 30))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 1) (authored-target "p_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 11 36) (end 11 39)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 2) (authored-target "p_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 11 44) (end 11 49)) (probe (position 11 44))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_2"))) (kind expressionOperand) (ordinal 3) (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 15 19) (end 15 22)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 0) (authored-target "T_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 15 26) (end 15 29)) (probe (position 15 26))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 1) (authored-target "p_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 15 32) (end 15 35)) (probe (position 15 32))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 2) (authored-target "p_1")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 15 40) (end 15 45)) (probe (position 15 40))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 3) (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 15 53) (end 15 58)) (probe (position 15 53))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_2"))) (kind expressionOperand) (ordinal 4) (authored-target "gamma")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 28 29) (end 28 30)) (probe (position 28 29))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 28 35) (end 28 43)) (probe (position 28 35))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::H_total"))) (kind expressionOperand) (ordinal 1) (authored-target "h_static")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 19 12) (end 19 15)) (probe (position 19 12))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 0) (authored-target "rho")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 19 18) (end 19 19)) (probe (position 19 18))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 1) (authored-target "V")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 19 24) (end 19 32)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind expressionOperand) (ordinal 2) (authored-target "P_static")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 24 15) (end 24 31)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 24 41) (end 24 43)) (probe (position 24 41))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "Cp")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 24 47) (end 24 48)) (probe (position 24 47))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 24 53) (end 24 61)) (probe (position 24 53))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (path (named (kind package) (name "Turbojet Stage Analysis")) (named (kind package) (name "Thermodynamic Functions")) (named (kind calc-def) (name "Total Temperature")) (anonymous (kind parameter) (ordinal 0)))))) (kind expressionOperand) (ordinal 2) (authored-target "T_static")
      (outcome (status resolved) (target (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static")))))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 23 76) (end 23 93)) (probe (position 23 76))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 23 50) (end 23 66)) (probe (position 23 50))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/turbojet_stage_analysis.md") (range (start 23 102) (end 23 113)) (probe (position 23 102))
    (reference (id (source (node (document "memory://snapshot/turbojet_stage_analysis.md") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeValue")
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
