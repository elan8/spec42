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
  (document "turbojet_stage_analysis.md"
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
        (range (start 3 16) (end 3 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 32) (end 6 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 40) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 50) (end 6 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 59) (end 10 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 69) (end 10 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 77) (end 10 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 85) (end 10 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 63) (end 14 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 71) (end 14 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 79) (end 14 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 87) (end 14 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 33) (end 18 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 46) (end 18 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 54) (end 18 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 36) (end 23 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 95) (end 23 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 6) (end 24 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 33) (end 27 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 45) (end 27 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 58) (end 27 63))
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
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 87 10) (end 87 66))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 87 10) (end 87 66))
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
        (range (start 88 27) (end 88 52))
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
        (range (start 89 41) (end 89 68))
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
        (range (start 90 38) (end 90 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 35) (end 91 58))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "fd9324ab4ba4577b38c23c8f5dbb963c0c8ffb5532916479a74aef2accc97e4f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (kind "package") (name "Turbojet Stage Analysis") (declared-name "Turbojet Stage Analysis"))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (kind "import") (name "DimensionOneValue") (declared-name "DimensionOneValue") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::DimensionOneValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))) (kind "package") (name "Low-Pressure Compressor Analysis") (declared-name "Low-Pressure Compressor Analysis") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (kind "part") (name "Analysis Context") (declared-name "Analysis Context") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind "part") (name "Inlet Gas") (declared-name "Inlet Gas") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermodynamics Structure::Moving Ideal Gas Parcel")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (kind "attribute") (name "Static Pressure") (declared-name "Static Pressure") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Static Pressure")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))) (kind "package") (name "Thermodynamic Functions") (declared-name "Thermodynamic Functions") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (kind "calc def") (name "Ideal Gas Law") (declared-name "Ideal Gas Law") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (kind "in out parameter") (name "R_bar") (declared-name "R_bar") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (kind "in out parameter") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (kind "in out parameter") (name "rho") (declared-name "rho") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (kind "calc def") (name "Reversible Adiabatic Compression Density") (declared-name "Reversible Adiabatic Compression Density") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (kind "in out parameter") (name "gamma") (declared-name "gamma") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (kind "in out parameter") (name "p_1") (declared-name "p_1") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (kind "in out parameter") (name "p_2") (declared-name "p_2") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (kind "in out parameter") (name "rho_1") (declared-name "rho_1") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (kind "calc def") (name "Reversible Adiabatic Compression Temperature") (declared-name "Reversible Adiabatic Compression Temperature") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (kind "in out parameter") (name "T_1") (declared-name "T_1") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (kind "in out parameter") (name "gamma") (declared-name "gamma") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (kind "in out parameter") (name "p_1") (declared-name "p_1") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (kind "in out parameter") (name "p_2") (declared-name "p_2") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (kind "calc def") (name "Total Enthalpy") (declared-name "Total Enthalpy") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (kind "in out parameter") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (kind "in out parameter") (name "h_static") (declared-name "h_static") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (kind "in out parameter") (name "h_total") (declared-name "h_total") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind "calc def") (name "Total Pressure") (declared-name "Total Pressure") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (kind "in out parameter") (name "P_static") (declared-name "P_static") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (kind "in out parameter") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (kind "in out parameter") (name "rho") (declared-name "rho") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (kind "calc def") (name "Total Temperature") (declared-name "Total Temperature") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind "in out parameter") (name "Cp") (declared-name "Cp") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind "in out parameter") (name "T_static") (declared-name "T_static") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind "in out parameter") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "VolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (kind "package") (name "Thermodynamics Structure") (declared-name "Thermodynamics Structure") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind "action def") (name "Adiabatic Process") (declared-name "Adiabatic Process") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Thermodynamic Process")) (specializes (reference "Thermodynamic Process")) (specializes (reference "Thermodynamic Process")) (perform (reference "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1")) (perform (reference "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind "action") (name "Stage 1") (declared-name "Stage 1") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind "action") (name "Stage 2") (declared-name "Stage 2") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (kind "part def") (name "Ideal Gas Parcel") (declared-name "Ideal Gas Parcel") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Enthalpy"))) (kind "attribute") (name "Stagnation Enthalpy") (declared-name "Stagnation Enthalpy") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Pressure"))) (kind "attribute") (name "Stagnation Pressure") (declared-name "Stagnation Pressure") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Temperature"))) (kind "attribute") (name "Stagnation Temperature") (declared-name "Stagnation Temperature") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind "attribute") (name "Static Enthalpy") (declared-name "Static Enthalpy") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Enthalpy")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind "attribute") (name "Static Pressure") (declared-name "Static Pressure") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Pressure")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind "attribute") (name "Static Temperature") (declared-name "Static Temperature") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Temperature")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind "action def") (name "Reversible Adiabatic Process") (declared-name "Reversible Adiabatic Process") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Adiabatic Process")) (specializes (reference "Adiabatic Process")) (specializes (reference "Adiabatic Process")))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (kind "action def") (name "Thermodynamic Process") (declared-name "Thermodynamic Process") (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::DimensionOneValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 0)) (authored-target "Density") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 1)) (authored-target "Specific Gas Constant") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 2)) (authored-target "Static Temperature") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 3)) (authored-target "Static Pressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 0)) (authored-target "Solve for Pressure1::rho") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 1)) (authored-target "Solve for Pressure1::R_bar") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 2)) (authored-target "Solve for Pressure1::T") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 3)) (authored-target "Solve for Pressure1::p") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (kind redefinition) (ordinal 0)) (authored-target "Static Pressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0)) (authored-target "Thermodynamic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 1)) (authored-target "Thermodynamic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 2)) (authored-target "Thermodynamic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 0)) (authored-target "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 1)) (authored-target "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind redefinition) (ordinal 0)) (authored-target "start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind redefinition) (ordinal 0)) (authored-target "done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Enthalpy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Pressure") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Temperature") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0)) (authored-target "Adiabatic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 1)) (authored-target "Adiabatic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 2)) (authored-target "Adiabatic Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 2)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 19)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 3 16) (end 3 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 30) (end 74 34)) (probe (position 74 30))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))
        (kind redefinition) (ordinal 0) (authored-target "done")
        (range (start 74 30) (end 74 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 73 30) (end 73 35)) (probe (position 73 30))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))
        (kind redefinition) (ordinal 0) (authored-target "start")
        (range (start 73 30) (end 73 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 15) (end 88 24)) (probe (position 88 15))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindSource) (ordinal 0) (authored-target "Density")
        (range (start 88 15) (end 88 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 91 15) (end 91 32)) (probe (position 91 15))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindSource) (ordinal 3) (authored-target "Static Pressure")
        (range (start 91 15) (end 91 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure") (range (start 101 13) (end 101 121)))
        )
      )
    )
    (query (range (start 101 27) (end 101 44)) (probe (position 101 27))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))
        (kind redefinition) (ordinal 0) (authored-target "Static Pressure")
        (range (start 101 27) (end 101 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure") (range (start 101 13) (end 101 121)))
        )
      )
    )
    (query (range (start 77 59) (end 77 78)) (probe (position 77 59))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))
        (kind specialization) (ordinal 2) (authored-target "Adiabatic Process")
        (range (start 77 59) (end 77 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process") (range (start 68 5) (end 68 331)))
        )
      )
    )
    (query (range (start 90 15) (end 90 35)) (probe (position 90 15))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindSource) (ordinal 2) (authored-target "Static Temperature")
        (range (start 90 15) (end 90 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 68 48) (end 68 71)) (probe (position 68 48))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))
        (kind specialization) (ordinal 2) (authored-target "Thermodynamic Process")
        (range (start 68 48) (end 68 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process") (range (start 66 5) (end 66 40)))
        )
      )
    )
    (query (range (start 89 15) (end 89 38)) (probe (position 89 15))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindSource) (ordinal 1) (authored-target "Specific Gas Constant")
        (range (start 89 15) (end 89 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 90 38) (end 90 61)) (probe (position 90 38))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindTarget) (ordinal 2) (authored-target "Solve for Pressure1::T")
        (range (start 90 38) (end 90 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 91 35) (end 91 58)) (probe (position 91 35))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindTarget) (ordinal 3) (authored-target "Solve for Pressure1::p")
        (range (start 91 35) (end 91 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 27) (end 88 52)) (probe (position 88 27))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindTarget) (ordinal 0) (authored-target "Solve for Pressure1::rho")
        (range (start 88 27) (end 88 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 89 41) (end 89 68)) (probe (position 89 41))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind bindTarget) (ordinal 1) (authored-target "Solve for Pressure1::R_bar")
        (range (start 89 41) (end 89 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 47) (end 61 77)) (probe (position 61 47))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))
        (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Pressure")
        (range (start 61 47) (end 61 77))
        (outcome (status unresolved))
      )
    )
    (query (range (start 63 47) (end 63 77)) (probe (position 63 47))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))
        (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Enthalpy")
        (range (start 63 47) (end 63 77))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 47)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 1 16) (end 1 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 62 50) (end 62 83)) (probe (position 62 50))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))
        (kind redefinition) (ordinal 0) (authored-target "Ideal Gas Parcel::Temperature")
        (range (start 62 50) (end 62 83))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 56)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::DimensionOneValue")
        (range (start 2 16) (end 2 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 85 28) (end 85 81)) (probe (position 85 28))
      (reference
        (source (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
        (kind featureTyping) (ordinal 0) (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel")
        (range (start 85 28) (end 85 81))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
