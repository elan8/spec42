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
# FORMAT
~~~sysml
package 'Turbojet Stage Analysis' {
    private import Quantities::ScalarQuantityValue;
    private import MeasurementReferences::DimensionOneValue;
    private import ISQ::*;

    package 'Thermodynamic Functions' {
        calc def 'Ideal Gas Law' {
            in rho;
            in R_bar;
            in T;
            return p = rho * R_bar * T;
        }

        calc def 'Reversible Adiabatic Compression Density' {
            in rho_1;
            in p_1;
            in p_2;
            in gamma;
            return rho_2 = rho_1 * (p_2 / p_1)^(1/gamma);
        }

        calc def 'Reversible Adiabatic Compression Temperature' {
            in T_1;
            in p_1;
            in p_2;
            in gamma;
            return T_2 = T_1 * (p_2 / p_1)**((gamma - 1) / gamma);
        }

        calc def 'Total Pressure' {
            in P_static;
            in rho;
            in V;
            = 1 / 2 * rho * V ^ 2 + P_static;
        }

        // Showing explicit parameter typing
        calc def 'Total Temperature' {
            in T_static : TemperatureValue;
            in Cp : DimensionOneValue;
            in V : VolumeValue;
            return : TemperatureValue = 1/(2 * Cp) * V^2 + T_static;
        }

        calc def 'Total Enthalpy' {
            in h_total;
            in h_static;
            in V;
            return H_total = 1/2 * V^2 + h_static;
        }
    }

    package 'Thermodynamics Structure' {
        part def 'Ideal Gas Parcel' {
            comment /*
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
            comment about 'Stagnation Pressure' /*
	            Stagnation pressure is the pressure of the parcel if the kinetic energy defined by its
	            velocity in a given coordinate frame is converted to gas internal energy through deceleration
	            to a velocity that matches the current frame.
	            */
            attribute 'Stagnation Pressure';
            attribute 'Stagnation Temperature';
            attribute 'Stagnation Enthalpy';

            comment about 'Static Pressure' /*
	            Static pressure is the pressure of the parcel as it moves
	            */
            attribute 'Static Pressure' redefines 'Ideal Gas Parcel'::'Pressure';
            attribute 'Static Temperature' redefines 'Ideal Gas Parcel'::'Temperature';
            attribute 'Static Enthalpy' redefines 'Ideal Gas Parcel'::'Enthalpy';
        }

        action def 'Thermodynamic Process';
        // need start and end shots to show beginning and end attributes

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
                constraint {
                    = 'Static Pressure' == 'Ideal Gas Law'('Density', 'Specific Gas Constant', 'Static Temperature');
                }
            }
        }
    }
}
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
# SMG
~~~
(model
  (namespace
    (package 'Turbojet Stage Analysis'
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::DimensionOneValue'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (package 'Thermodynamic Functions'
        (calculation_def 'Ideal Gas Law'
          (reference_usage in reference 'rho')
          (reference_usage in reference 'R_bar')
          (reference_usage in reference 'T')
          (return_parameter_membership
            (feature_def out 'p'
              (feature_value (=)))))
        (calculation_def 'Reversible Adiabatic Compression Density'
          (reference_usage in reference 'rho_1')
          (reference_usage in reference 'p_1')
          (reference_usage in reference 'p_2')
          (reference_usage in reference 'gamma')
          (return_parameter_membership
            (feature_def out 'rho_2'
              (feature_value (=)))))
        (calculation_def 'Reversible Adiabatic Compression Temperature'
          (reference_usage in reference 'T_1')
          (reference_usage in reference 'p_1')
          (reference_usage in reference 'p_2')
          (reference_usage in reference 'gamma')
          (return_parameter_membership
            (feature_def out 'T_2'
              (feature_value (=)))))
        (calculation_def 'Total Pressure'
          (reference_usage in reference 'P_static')
          (reference_usage in reference 'rho')
          (reference_usage in reference 'V')
          (result_expr_membership))
        (calculation_def 'Total Temperature'
          (reference_usage in reference 'T_static' : 'TemperatureValue'[unresolved])
          (reference_usage in reference 'Cp' : 'DimensionOneValue'[unresolved])
          (reference_usage in reference 'V' : 'VolumeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'TemperatureValue'[unresolved]
              (feature_value (=)))))
        (calculation_def 'Total Enthalpy'
          (reference_usage in reference 'h_total')
          (reference_usage in reference 'h_static')
          (reference_usage in reference 'V')
          (return_parameter_membership
            (feature_def out 'H_total'
              (feature_value (=))))))
      (package 'Thermodynamics Structure'
        (part_def 'Ideal Gas Parcel'
          (comment_annotating)
          (attribute_usage composite 'Molar Mass')
          (attribute_usage composite 'Density')
          (attribute_usage composite 'Pressure')
          (attribute_usage composite 'Temperature')
          (attribute_usage composite 'Enthalpy')
          (attribute_usage composite 'Specific Gas Constant'))
        (part_def 'Moving Ideal Gas Parcel' :> 'Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel'[part_def]
          (comment_annotating)
          (attribute_usage composite 'Stagnation Pressure')
          (attribute_usage composite 'Stagnation Temperature')
          (attribute_usage composite 'Stagnation Enthalpy')
          (comment_annotating)
          (attribute_usage composite 'Static Pressure' :>> 'Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Pressure'[attribute_usage])
          (attribute_usage composite 'Static Temperature' :>> 'Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Temperature'[attribute_usage])
          (attribute_usage composite 'Static Enthalpy' :>> 'Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Enthalpy'[attribute_usage]))
        (action_def 'Thermodynamic Process')
        (action_def 'Adiabatic Process' :> 'Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process'[action_def]
          (action_usage composite 'Stage 1' :>> 'start'[unresolved])
          (action_usage composite 'Stage 2' :>> 'done'[unresolved]))
        (action_def 'Reversible Adiabatic Process' :> 'Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process'[action_def]))
      (package 'Low-Pressure Compressor Analysis'
        (part_usage 'Analysis Context'
          (namespace_import private -> 'Turbojet Stage Analysis::Thermodynamic Functions'[package])
          (part_usage composite 'Inlet Gas' : 'Turbojet Stage Analysis::Thermodynamics Structure::Moving Ideal Gas Parcel'[part_def]
            (calculation_usage composite 'Solve for Pressure1' : 'Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law'[calculation_def])
            (binding_connector_def
              (connector_end ''Density'')
              (connector_end ''Solve for Pressure1'.rho'))
            (binding_connector_def
              (connector_end ''Specific Gas Constant'')
              (connector_end ''Solve for Pressure1'.R_bar'))
            (binding_connector_def
              (connector_end ''Static Temperature'')
              (connector_end ''Solve for Pressure1'.T'))
            (binding_connector_def
              (connector_end ''Static Pressure'')
              (connector_end ''Solve for Pressure1'.p'))
            (calculation_usage composite 'Solve for Pressure2' : 'Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law'[calculation_def]
              (reference_usage in reference 'rho'
                (feature_value (=)))
              (reference_usage in reference 'R_bar'
                (feature_value (=)))
              (reference_usage in reference 'T'
                (feature_value (=))))
            (attribute_usage composite :>> 'Turbojet Stage Analysis::Thermodynamics Structure::Moving Ideal Gas Parcel::Static Pressure'[attribute_usage]
              (feature_value (=)))
            (constraint_usage composite
              (result_expr_membership))))))))
~~~
