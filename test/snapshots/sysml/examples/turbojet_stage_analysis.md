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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Star,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Star,OpenParen,Ident,Slash,Ident,CloseParen,Caret,OpenParen,DecimalValue,Slash,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Star,OpenParen,Ident,Slash,Ident,CloseParen,StarStar,OpenParen,OpenParen,Ident,Minus,DecimalValue,CloseParen,Slash,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
DecimalValue,Slash,DecimalValue,Star,Ident,Star,Ident,Caret,DecimalValue,Plus,Ident,
CloseCurly,
LineComment,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,DecimalValue,Slash,OpenParen,DecimalValue,Star,Ident,CloseParen,Star,Ident,Caret,DecimalValue,Plus,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,
KwReturn,Ident,Eq,DecimalValue,Slash,DecimalValue,Star,Ident,Caret,DecimalValue,Plus,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,UnrestrictedName,OpenCurly,
KwComment,
RegularComment,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,UnrestrictedName,KwSpecializes,UnrestrictedName,OpenCurly,
KwComment,KwAbout,UnrestrictedName,
RegularComment,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,Semicolon,
KwComment,KwAbout,UnrestrictedName,
RegularComment,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,ColonColon,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,ColonColon,UnrestrictedName,Semicolon,
KwAttribute,UnrestrictedName,KwRedefines,UnrestrictedName,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,
KwAction,KwDef,UnrestrictedName,Semicolon,LineComment,
KwAction,KwDef,UnrestrictedName,KwSpecializes,UnrestrictedName,OpenCurly,
RegularComment,
KwAction,UnrestrictedName,ColonGtGt,Ident,Semicolon,
KwAction,UnrestrictedName,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,UnrestrictedName,KwSpecializes,UnrestrictedName,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPart,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,UnrestrictedName,Colon,UnrestrictedName,ColonColon,UnrestrictedName,OpenCurly,
LineComment,
KwCalc,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwBind,UnrestrictedName,Eq,UnrestrictedName,Dot,Ident,Semicolon,
KwBind,UnrestrictedName,Eq,UnrestrictedName,Dot,Ident,Semicolon,
KwBind,UnrestrictedName,Eq,UnrestrictedName,Dot,Ident,Semicolon,
KwBind,UnrestrictedName,Eq,UnrestrictedName,Dot,Ident,Semicolon,
LineComment,
KwCalc,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,UnrestrictedName,Semicolon,
KwIn,Ident,Eq,UnrestrictedName,Semicolon,
KwIn,Ident,Eq,UnrestrictedName,Semicolon,
CloseCurly,
LineComment,
KwAttribute,ColonGtGt,UnrestrictedName,Eq,UnrestrictedName,OpenParen,UnrestrictedName,Comma,UnrestrictedName,Comma,UnrestrictedName,CloseParen,Semicolon,
LineComment,
KwConstraint,OpenCurly,UnrestrictedName,EqEq,UnrestrictedName,OpenParen,UnrestrictedName,Comma,UnrestrictedName,Comma,UnrestrictedName,CloseParen,CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Turbojet Stage Analysis''
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'MeasurementReferences::DimensionOneValue')
    (import_decl private 'ISQ::*')
    (package_def ''Thermodynamic Functions''
      (calc_def ''Ideal Gas Law''
        (default_ref_usage in 'rho')
        (default_ref_usage in 'R_bar')
        (default_ref_usage in 'T')
        (return_member))
      (calc_def ''Reversible Adiabatic Compression Density''
        (default_ref_usage in 'rho_1')
        (default_ref_usage in 'p_1')
        (default_ref_usage in 'p_2')
        (default_ref_usage in 'gamma')
        (return_member))
      (calc_def ''Reversible Adiabatic Compression Temperature''
        (default_ref_usage in 'T_1')
        (default_ref_usage in 'p_1')
        (default_ref_usage in 'p_2')
        (default_ref_usage in 'gamma')
        (return_member))
      (calc_def ''Total Pressure''
        (default_ref_usage in 'P_static')
        (default_ref_usage in 'rho')
        (default_ref_usage in 'V')
        (result_expr_member))
      (line_comment)
      (calc_def ''Total Temperature''
        (default_ref_usage in 'T_static' : 'TemperatureValue')
        (default_ref_usage in 'Cp' : 'DimensionOneValue')
        (default_ref_usage in 'V' : 'VolumeValue')
        (return_member))
      (calc_def ''Total Enthalpy''
        (default_ref_usage in 'h_total')
        (default_ref_usage in 'h_static')
        (default_ref_usage in 'V')
        (return_member)))
    (package_def ''Thermodynamics Structure''
      (part_def ''Ideal Gas Parcel''
        (comment_annotating)
        (attribute_usage ''Molar Mass'')
        (attribute_usage ''Density'')
        (attribute_usage ''Pressure'')
        (attribute_usage ''Temperature'')
        (attribute_usage ''Enthalpy'')
        (attribute_usage ''Specific Gas Constant''))
      (part_def ''Moving Ideal Gas Parcel'' :> ''Ideal Gas Parcel''
        (comment_annotating about ''Stagnation Pressure'')
        (attribute_usage ''Stagnation Pressure'')
        (attribute_usage ''Stagnation Temperature'')
        (attribute_usage ''Stagnation Enthalpy'')
        (comment_annotating about ''Static Pressure'')
        (attribute_usage ''Static Pressure'' :>> ''Ideal Gas Parcel'::'Pressure'')
        (attribute_usage ''Static Temperature'' :>> ''Ideal Gas Parcel'::'Temperature'')
        (attribute_usage ''Static Enthalpy'' :>> ''Ideal Gas Parcel'::'Enthalpy''))
      (action_def ''Thermodynamic Process'')
      (line_comment)
      (action_def ''Adiabatic Process'' :> ''Thermodynamic Process''
        (comment)
        (action_usage ''Stage 1'' :>> 'start')
        (action_usage ''Stage 2'' :>> 'done'))
      (action_def ''Reversible Adiabatic Process'' :> ''Adiabatic Process''))
    (package_def ''Low-Pressure Compressor Analysis''
      (part_usage ''Analysis Context''
        (import_decl private ''Thermodynamic Functions'::*')
        (part_usage ''Inlet Gas'' : ''Thermodynamics Structure'::'Moving Ideal Gas Parcel''
          (line_comment)
          (calc_usage ''Solve for Pressure1'' : ''Ideal Gas Law'')
          (binding_as_usage
            (connector_end)
            (connector_end))
          (binding_as_usage
            (connector_end)
            (connector_end))
          (binding_as_usage
            (connector_end)
            (connector_end))
          (binding_as_usage
            (connector_end)
            (connector_end))
          (line_comment)
          (calc_usage ''Solve for Pressure2'' : ''Ideal Gas Law''
            (default_ref_usage in 'rho' value)
            (default_ref_usage in 'R_bar' value)
            (default_ref_usage in 'T' value))
          (line_comment)
          (attribute_usage :>> ''Static Pressure'' value)
          (line_comment)
          (constraint_usage
            (result_expr_member)))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "fd9324ab4ba4577b38c23c8f5dbb963c0c8ffb5532916479a74aef2accc97e4f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (kind "package") (name "Turbojet Stage Analysis") (declared-name "Turbojet Stage Analysis") (range (start (line 0) (character 0)) (end (line 0) (character 4250))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (kind "import") (name "DimensionOneValue") (declared-name "DimensionOneValue") (range (start (line 2) (character 1)) (end (line 2) (character 57))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::DimensionOneValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 56))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))) (kind "package") (name "Low-Pressure Compressor Analysis") (declared-name "Low-Pressure Compressor Analysis") (range (start (line 80) (character 1)) (end (line 80) (character 1233))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (kind "part") (name "Analysis Context") (declared-name "Analysis Context") (range (start (line 82) (character 5)) (end (line 82) (character 1178))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind "part") (name "Inlet Gas") (declared-name "Inlet Gas") (range (start (line 85) (character 9)) (end (line 85) (character 1076))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thermodynamics Structure::Moving Ideal Gas Parcel") (range (start (line 85) (character 28)) (end (line 85) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (kind "attribute") (name "Static Pressure") (declared-name "Static Pressure") (range (start (line 101) (character 13)) (end (line 101) (character 121))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Static Pressure") (range (start (line 101) (character 27)) (end (line 101) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 1) (character 1)) (end (line 1) (character 48))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 47))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))) (kind "package") (name "Thermodynamic Functions") (declared-name "Thermodynamic Functions") (range (start (line 5) (character 1)) (end (line 5) (character 928))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (kind "calc def") (name "Ideal Gas Law") (declared-name "Ideal Gas Law") (range (start (line 6) (character 5)) (end (line 6) (character 96))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (kind "in out parameter") (name "R_bar") (declared-name "R_bar") (range (start (line 6) (character 40)) (end (line 6) (character 49))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (kind "in out parameter") (name "T") (declared-name "T") (range (start (line 6) (character 50)) (end (line 6) (character 55))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (kind "in out parameter") (name "rho") (declared-name "rho") (range (start (line 6) (character 32)) (end (line 6) (character 39))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (kind "calc def") (name "Reversible Adiabatic Compression Density") (declared-name "Reversible Adiabatic Compression Density") (range (start (line 10) (character 5)) (end (line 10) (character 153))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (kind "in out parameter") (name "gamma") (declared-name "gamma") (range (start (line 10) (character 85)) (end (line 10) (character 94))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (kind "in out parameter") (name "p_1") (declared-name "p_1") (range (start (line 10) (character 69)) (end (line 10) (character 76))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (kind "in out parameter") (name "p_2") (declared-name "p_2") (range (start (line 10) (character 77)) (end (line 10) (character 84))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (kind "in out parameter") (name "rho_1") (declared-name "rho_1") (range (start (line 10) (character 59)) (end (line 10) (character 68))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (kind "calc def") (name "Reversible Adiabatic Compression Temperature") (declared-name "Reversible Adiabatic Compression Temperature") (range (start (line 14) (character 5)) (end (line 14) (character 164))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (kind "in out parameter") (name "T_1") (declared-name "T_1") (range (start (line 14) (character 63)) (end (line 14) (character 70))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (kind "in out parameter") (name "gamma") (declared-name "gamma") (range (start (line 14) (character 87)) (end (line 14) (character 96))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (kind "in out parameter") (name "p_1") (declared-name "p_1") (range (start (line 14) (character 71)) (end (line 14) (character 78))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (kind "in out parameter") (name "p_2") (declared-name "p_2") (range (start (line 14) (character 79)) (end (line 14) (character 86))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (kind "calc def") (name "Total Enthalpy") (declared-name "Total Enthalpy") (range (start (line 27) (character 5)) (end (line 27) (character 115))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (kind "in out parameter") (name "V") (declared-name "V") (range (start (line 27) (character 58)) (end (line 27) (character 63))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (kind "in out parameter") (name "h_static") (declared-name "h_static") (range (start (line 27) (character 45)) (end (line 27) (character 57))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (kind "in out parameter") (name "h_total") (declared-name "h_total") (range (start (line 27) (character 33)) (end (line 27) (character 44))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (kind "calc def") (name "Total Pressure") (declared-name "Total Pressure") (range (start (line 18) (character 5)) (end (line 18) (character 99))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (kind "in out parameter") (name "P_static") (declared-name "P_static") (range (start (line 18) (character 33)) (end (line 18) (character 45))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (kind "in out parameter") (name "V") (declared-name "V") (range (start (line 18) (character 54)) (end (line 18) (character 59))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (kind "in out parameter") (name "rho") (declared-name "rho") (range (start (line 18) (character 46)) (end (line 18) (character 53))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (kind "calc def") (name "Total Temperature") (declared-name "Total Temperature") (range (start (line 23) (character 5)) (end (line 23) (character 184))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::"))) (kind "return parameter") (name "") (range (start (line 24) (character 6)) (end (line 24) (character 62))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind "in out parameter") (name "Cp") (declared-name "Cp") (range (start (line 23) (character 68)) (end (line 23) (character 94))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind "in out parameter") (name "T_static") (declared-name "T_static") (range (start (line 23) (character 36)) (end (line 23) (character 67))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind "in out parameter") (name "V") (declared-name "V") (range (start (line 23) (character 95)) (end (line 23) (character 114))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (authored (relationships (typing (reference "VolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (kind "package") (name "Thermodynamics Structure") (declared-name "Thermodynamics Structure") (range (start (line 32) (character 1)) (end (line 32) (character 1909))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind "action def") (name "Adiabatic Process") (declared-name "Adiabatic Process") (range (start (line 68) (character 5)) (end (line 68) (character 331))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Thermodynamic Process") (range none)) (specializes (reference "Thermodynamic Process") (range none)) (specializes (reference "Thermodynamic Process") (range (start (line 68) (character 48)) (end (line 68) (character 71)))) (perform (reference "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1") (range none)) (perform (reference "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2") (range none)))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind "action") (name "Stage 1") (declared-name "Stage 1") (range (start (line 73) (character 9)) (end (line 73) (character 36))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start") (range (start (line 73) (character 30)) (end (line 73) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind "action") (name "Stage 2") (declared-name "Stage 2") (range (start (line 74) (character 9)) (end (line 74) (character 35))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done") (range (start (line 74) (character 30)) (end (line 74) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (kind "part def") (name "Ideal Gas Parcel") (declared-name "Ideal Gas Parcel") (range (start (line 33) (character 5)) (end (line 33) (character 1332))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Enthalpy"))) (kind "attribute") (name "Stagnation Enthalpy") (declared-name "Stagnation Enthalpy") (range (start (line 55) (character 9)) (end (line 55) (character 41))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Pressure"))) (kind "attribute") (name "Stagnation Pressure") (declared-name "Stagnation Pressure") (range (start (line 53) (character 9)) (end (line 53) (character 41))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Temperature"))) (kind "attribute") (name "Stagnation Temperature") (declared-name "Stagnation Temperature") (range (start (line 54) (character 9)) (end (line 54) (character 44))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind "attribute") (name "Static Enthalpy") (declared-name "Static Enthalpy") (range (start (line 63) (character 9)) (end (line 63) (character 78))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Enthalpy") (range (start (line 63) (character 47)) (end (line 63) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind "attribute") (name "Static Pressure") (declared-name "Static Pressure") (range (start (line 61) (character 9)) (end (line 61) (character 78))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Pressure") (range (start (line 61) (character 47)) (end (line 61) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind "attribute") (name "Static Temperature") (declared-name "Static Temperature") (range (start (line 62) (character 9)) (end (line 62) (character 84))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Ideal Gas Parcel::Temperature") (range (start (line 62) (character 50)) (end (line 62) (character 83)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind "action def") (name "Reversible Adiabatic Process") (declared-name "Reversible Adiabatic Process") (range (start (line 77) (character 5)) (end (line 77) (character 79))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Adiabatic Process") (range none)) (specializes (reference "Adiabatic Process") (range none)) (specializes (reference "Adiabatic Process") (range (start (line 77) (character 59)) (end (line 77) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (kind "action def") (name "Thermodynamic Process") (declared-name "Thermodynamic Process") (range (start (line 66) (character 5)) (end (line 66) (character 40))) (parent (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 3) (character 16)) (end (line 3) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::DimensionOneValue") (range (start (line 2) (character 16)) (end (line 2) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind featureTyping) (ordinal 0)) (authored-target "Thermodynamics Structure::Moving Ideal Gas Parcel") (range (start (line 85) (character 28)) (end (line 85) (character 81))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 0)) (authored-target "Density") (range (start (line 88) (character 15)) (end (line 88) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 1)) (authored-target "Specific Gas Constant") (range (start (line 89) (character 15)) (end (line 89) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 2)) (authored-target "Static Temperature") (range (start (line 90) (character 15)) (end (line 90) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindSource) (ordinal 3)) (authored-target "Static Pressure") (range (start (line 91) (character 15)) (end (line 91) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 0)) (authored-target "Solve for Pressure1::rho") (range (start (line 88) (character 27)) (end (line 88) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 1)) (authored-target "Solve for Pressure1::R_bar") (range (start (line 89) (character 41)) (end (line 89) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 2)) (authored-target "Solve for Pressure1::T") (range (start (line 90) (character 38)) (end (line 90) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (kind bindTarget) (ordinal 3)) (authored-target "Solve for Pressure1::p") (range (start (line 91) (character 35)) (end (line 91) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (kind redefinition) (ordinal 0)) (authored-target "Static Pressure") (range (start (line 101) (character 27)) (end (line 101) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (range (start (line 1) (character 16)) (end (line 1) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 0)) (authored-target "Thermodynamic Process") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 1)) (authored-target "Thermodynamic Process") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind specialization) (ordinal 2)) (authored-target "Thermodynamic Process") (range (start (line 68) (character 48)) (end (line 68) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 0)) (authored-target "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (kind performSource) (ordinal 1)) (authored-target "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (kind redefinition) (ordinal 0)) (authored-target "start") (range (start (line 73) (character 30)) (end (line 73) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (kind redefinition) (ordinal 0)) (authored-target "done") (range (start (line 74) (character 30)) (end (line 74) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Enthalpy") (range (start (line 63) (character 47)) (end (line 63) (character 77))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Pressure") (range (start (line 61) (character 47)) (end (line 61) (character 77))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (kind redefinition) (ordinal 0)) (authored-target "Ideal Gas Parcel::Temperature") (range (start (line 62) (character 50)) (end (line 62) (character 83))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 0)) (authored-target "Adiabatic Process") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 1)) (authored-target "Adiabatic Process") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (kind specialization) (ordinal 2)) (authored-target "Adiabatic Process") (range (start (line 77) (character 59)) (end (line 77) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
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
