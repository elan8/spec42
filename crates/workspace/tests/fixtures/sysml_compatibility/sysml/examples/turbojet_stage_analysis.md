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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis"))) (name "Turbojet Stage Analysis") (declared-name "Turbojet Stage Analysis")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::DimensionOneValue"))) (name "DimensionOneValue") (declared-name "DimensionOneValue"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis"))) (name "Low-Pressure Compressor Analysis") (declared-name "Low-Pressure Compressor Analysis")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context"))) (name "Analysis Context") (declared-name "Analysis Context") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))) (name "Inlet Gas") (declared-name "Inlet Gas") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (name "Static Pressure") (declared-name "Static Pressure") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "Ideal Gas Law"))) (arguments (argument (expression (kind "featureReference") (reference "Density"))) (argument (expression (kind "featureReference") (reference "Specific Gas Constant"))) (argument (expression (kind "featureReference") (reference "Static Temperature"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas::Static Pressure"))) (role feature-value))))
                  )
                )
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions"))) (name "Thermodynamic Functions") (declared-name "Thermodynamic Functions")
          (contains
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law"))) (name "Ideal Gas Law") (declared-name "Ideal Gas Law")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::R_bar"))) (name "R_bar") (declared-name "R_bar") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::T"))) (name "T") (declared-name "T") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law::rho"))) (name "rho") (declared-name "rho") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Ideal Gas Law")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density"))) (name "Reversible Adiabatic Compression Density") (declared-name "Reversible Adiabatic Compression Density")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::gamma"))) (name "gamma") (declared-name "gamma") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_1"))) (name "p_1") (declared-name "p_1") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::p_2"))) (name "p_2") (declared-name "p_2") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density::rho_1"))) (name "rho_1") (declared-name "rho_1") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Density")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature"))) (name "Reversible Adiabatic Compression Temperature") (declared-name "Reversible Adiabatic Compression Temperature")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::T_1"))) (name "T_1") (declared-name "T_1") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::gamma"))) (name "gamma") (declared-name "gamma") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_1"))) (name "p_1") (declared-name "p_1") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature::p_2"))) (name "p_2") (declared-name "p_2") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Reversible Adiabatic Compression Temperature")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy"))) (name "Total Enthalpy") (declared-name "Total Enthalpy")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::V"))) (name "V") (declared-name "V") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_static"))) (name "h_static") (declared-name "h_static") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy::h_total"))) (name "h_total") (declared-name "h_total") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Enthalpy")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure"))) (name "Total Pressure") (declared-name "Total Pressure")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::P_static"))) (name "P_static") (declared-name "P_static") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::V"))) (name "V") (declared-name "V") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure::rho"))) (name "rho") (declared-name "rho") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Pressure")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature"))) (name "Total Temperature") (declared-name "Total Temperature")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::Cp"))) (name "Cp") (declared-name "Cp") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::T_static"))) (name "T_static") (declared-name "T_static") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature::V"))) (name "V") (declared-name "V") (effective (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamic Functions::Total Temperature")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure"))) (name "Thermodynamics Structure") (declared-name "Thermodynamics Structure")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (name "Adiabatic Process") (declared-name "Adiabatic Process")
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))) (name "Stage 1") (declared-name "Stage 1") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))) (name "Stage 2") (declared-name "Stage 2") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel"))) (name "Ideal Gas Parcel") (declared-name "Ideal Gas Parcel") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Enthalpy"))) (name "Stagnation Enthalpy") (declared-name "Stagnation Enthalpy") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Pressure"))) (name "Stagnation Pressure") (declared-name "Stagnation Pressure") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Stagnation Temperature"))) (name "Stagnation Temperature") (declared-name "Stagnation Temperature") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Enthalpy"))) (name "Static Enthalpy") (declared-name "Static Enthalpy") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Pressure"))) (name "Static Pressure") (declared-name "Static Pressure") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel::Static Temperature"))) (name "Static Temperature") (declared-name "Static Temperature") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Ideal Gas Parcel")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (name "Reversible Adiabatic Process") (declared-name "Reversible Adiabatic Process"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))) (name "Thermodynamic Process") (declared-name "Thermodynamic Process"))
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (to (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 1"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (to (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process::Stage 2"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))) (to (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Thermodynamic Process"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Reversible Adiabatic Process"))) (to (node (document "d0") (qualified-name "Turbojet Stage Analysis::Thermodynamics Structure::Adiabatic Process"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "Density") (target-expression "Solve for Pressure1::rho") (container-prefix "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
    (bind (status pending-expression) (document "d0") (source-expression "Specific Gas Constant") (target-expression "Solve for Pressure1::R_bar") (container-prefix "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
    (bind (status pending-expression) (document "d0") (source-expression "Static Pressure") (target-expression "Solve for Pressure1::p") (container-prefix "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
    (bind (status pending-expression) (document "d0") (source-expression "Static Temperature") (target-expression "Solve for Pressure1::T") (container-prefix "Turbojet Stage Analysis::Low-Pressure Compressor Analysis::Analysis Context::Inlet Gas"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/turbojet_stage_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 23))
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
        (range (start 23 68) (end 23 94))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 61 9) (end 61 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 62 9) (end 62 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 63 9) (end 63 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 73 9) (end 73 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 74 9) (end 74 35))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
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
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 88 15) (end 88 24))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 89 15) (end 89 38))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 90 15) (end 90 35))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 91 15) (end 91 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 101 13) (end 101 121))
      )
    )
  )
)
~~~
