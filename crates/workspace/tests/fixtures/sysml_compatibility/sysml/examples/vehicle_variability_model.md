# META
~~~ini
description=SysML Example (Variability): VehicleVariabilityModel
type=file
~~~
# SOURCE
~~~sysml
package VehicleVariabilityModel {

	package DesignModel {
		public import PartDefinitions::*;
		public import PartsTree::*;
		public import ActionDefinitions::*;
		public import ActionTree::*;
	
		package PartDefinitions {
			part def Vehicle;
			
		    attribute def Diameter;
		    part def Cylinder {
		        attribute diameter : Diameter[1];
		    }
		
		    part def Engine;
		    part def Transmission;
		    part def Sunroof;
		
		    port def AutoPort;
	    }
	    
	    package PartsTree {
	    	part vehicle : Vehicle {
	    		part engine : Engine[1];
	    		part transmission : Transmission[1];
	    		part sunroof : Sunroof[0..1];
	    	}
	    	
		    part engine : Engine {
		        port autoPort : AutoPort;
		        part cylinder : Cylinder[2..*];
		    }
		    
		    part '4cylEngine' :> engine {
		    	part :>> cylinder[4];
		    }
		    
		    part '6cylEngine' :> engine {
		    	part :>> cylinder[6];
		    }
		
			part transmission : Transmission;
		    part manualTransmission :> transmission;
		    part automaticTransmission :> transmission;
	    }
	
		package ActionDefinitions {   
		    action def GenerateTorque;
		    action def AmplifyTorque;
		    action def ProvidePower;
	    }
	    
	    package ActionTree {    
		    action generateTorque4Cyl : GenerateTorque;
		    action generateTorque6Cyl : GenerateTorque;
		    
		    action amplifyTorqueManual : AmplifyTorque;
		    action amplifyTorqueAutomatic : AmplifyTorque;
	    }	
	}
		
	package '150% Model' {
		private import DesignModel::*;
	
		package PartsTree {
		
		    // Variation point definitions
		
		    variation attribute def DiameterChoices :> Diameter {
		    	variant attribute diameterSmall;
		    	variant attribute diameterLarge;
		    }
		
		    variation part def EngineChoices :> Engine {
		        variant '4cylEngine';
		        variant '6cylEngine' {
		        	variation port :>> autoPort {
		        		variant port autoPort1;
		        		variant port autoPort2;
		        	}
		        	
		        	part :>> cylinder {
		        		attribute :>> diameter : DiameterChoices;
		        	}
		        	
		          	assert constraint {
		            	(autoPort == autoPort::autoPort1 and cylinder.diameter == cylinder::diameter::diameterSmall) xor
		             	(autoPort == autoPort::autoPort2 and cylinder.diameter == cylinder::diameter::diameterLarge)
		        	}
		        }
		    }
		
		    // Part superset model
		    
		    abstract part vehicleFamily :> vehicle {
		    	// Variation point usage
		        part :>> engine : EngineChoices[1];
		        
		        // Variation point with embedded variant definitions
		        variation part :>> transmission : Transmission[1] {
		        	variant manualTransmission;
		        	variant automaticTransmission;
		        }
		        
		        assert constraint {
		            (engine == engine::'4cylEngine' and transmission == transmission::manualTransmission) xor
		            (engine == engine::'6cylEngine' and transmission == transmission::automaticTransmission)
		        }
		        
		        // Variation point on variant multiplicity (inherited multiplicity is [0..1]) 
		        variation part :>> sunroof {
		        	variant part withSunroof[1];
		        	variant part withoutSunroof[0];
		        }
		        
		        perform ActionTree::providePowerFamily;
		    }
		}
		
		package ActionTree {
		
		    // Action superset Model
		    
		    action providePowerFamily : ProvidePower {
		        variation action generateTorque : GenerateTorque {
		        	variant generateTorque4Cyl;
		        	variant generateTorque6Cyl;
		        }
		        
		        variation action amplifyTorque : AmplifyTorque {
		        	variant amplifyTorqueManual;
		        	variant amplifyTorqueAutomatic;
		        }
		        
			    assert constraint {
			        (generateTorque == generateTorque::generateTorque4Cyl and 
			        	amplifyTorque == amplifyTorque::amplifyTorqueManual
			        ) xor
			        (generateTorque == generateTorque::generateTorque6Cyl and 
			        	amplifyTorque == amplifyTorque::amplifyTorqueAutomatic
			        )
			    }		   
		    }		    
		}
	}
	
	package '100% Model' {
		private import '150% Model'::*;
		
		// Vehicle instance model
		
	    part vehicle4Cyl :> PartsTree::vehicleFamily {
	        part :>> engine = engine::'4cylEngine';
	        part :>> transmission = transmission::manualTransmission;
	        part :>> sunroof = sunroof::withoutSunroof;
	        
	        perform action :>> providePowerFamily {
	            action :>> generateTorque = generateTorque::generateTorque4Cyl;
	            action :>> amplifyTorque = amplifyTorque::amplifyTorqueManual;
	        }
	    }
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
LineComment,
KwVariation,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwAttribute,Ident,Semicolon,
KwVariant,KwAttribute,Ident,Semicolon,
CloseCurly,
KwVariation,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,UnrestrictedName,Semicolon,
KwVariant,UnrestrictedName,OpenCurly,
KwVariation,KwPort,ColonGtGt,Ident,OpenCurly,
KwVariant,KwPort,Ident,Semicolon,
KwVariant,KwPort,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,Ident,Dot,Ident,EqEq,Ident,ColonColon,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,Ident,Dot,Ident,EqEq,Ident,ColonColon,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
KwAbstract,KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
KwPart,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwVariation,KwPart,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
LineComment,
KwVariation,KwPart,ColonGtGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwVariant,KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPerform,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwAction,Ident,Colon,Ident,OpenCurly,
KwVariation,KwAction,Ident,Colon,Ident,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwVariation,KwAction,Ident,Colon,Ident,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,
CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,Ident,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,
CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
LineComment,
KwPart,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPerform,KwAction,ColonGtGt,Ident,OpenCurly,
KwAction,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAction,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleVariabilityModel'
    (package_def 'DesignModel'
      (import_decl public 'PartDefinitions::*')
      (import_decl public 'PartsTree::*')
      (import_decl public 'ActionDefinitions::*')
      (import_decl public 'ActionTree::*')
      (package_def 'PartDefinitions'
        (part_def 'Vehicle')
        (attribute_def 'Diameter')
        (part_def 'Cylinder'
          (attribute_usage 'diameter' : 'Diameter' multiplicity))
        (part_def 'Engine')
        (part_def 'Transmission')
        (part_def 'Sunroof')
        (port_def 'AutoPort'))
      (package_def 'PartsTree'
        (part_usage 'vehicle' : 'Vehicle'
          (part_usage 'engine' : 'Engine' multiplicity)
          (part_usage 'transmission' : 'Transmission' multiplicity)
          (part_usage 'sunroof' : 'Sunroof' multiplicity))
        (part_usage 'engine' : 'Engine'
          (port_usage 'autoPort' : 'AutoPort')
          (part_usage 'cylinder' : 'Cylinder' multiplicity))
        (part_usage ''4cylEngine'' :> 'engine'
          (part_usage :>> 'cylinder' multiplicity))
        (part_usage ''6cylEngine'' :> 'engine'
          (part_usage :>> 'cylinder' multiplicity))
        (part_usage 'transmission' : 'Transmission')
        (part_usage 'manualTransmission' :> 'transmission')
        (part_usage 'automaticTransmission' :> 'transmission'))
      (package_def 'ActionDefinitions'
        (action_def 'GenerateTorque')
        (action_def 'AmplifyTorque')
        (action_def 'ProvidePower'))
      (package_def 'ActionTree'
        (action_usage 'generateTorque4Cyl' : 'GenerateTorque')
        (action_usage 'generateTorque6Cyl' : 'GenerateTorque')
        (action_usage 'amplifyTorqueManual' : 'AmplifyTorque')
        (action_usage 'amplifyTorqueAutomatic' : 'AmplifyTorque')))
    (package_def ''150% Model''
      (import_decl private 'DesignModel::*')
      (package_def 'PartsTree'
        (line_comment)
        (attribute_def variation 'DiameterChoices' :> 'Diameter'
          (variant_usage
            (attribute_usage 'diameterSmall'))
          (variant_usage
            (attribute_usage 'diameterLarge')))
        (part_def variation 'EngineChoices' :> 'Engine'
          (variant_usage
            (default_ref_usage ''4cylEngine''))
          (variant_usage
            (default_ref_usage ''6cylEngine''
              (port_usage variation :>> 'autoPort'
                (variant_usage
                  (port_usage 'autoPort1'))
                (variant_usage
                  (port_usage 'autoPort2')))
              (part_usage :>> 'cylinder'
                (attribute_usage :>> 'diameter' : 'DiameterChoices'))
              (sysml_decl
                (result_expr_member)))))
        (line_comment)
        (part_usage abstract 'vehicleFamily' :> 'vehicle'
          (line_comment)
          (part_usage :>> 'engine' : 'EngineChoices' multiplicity)
          (line_comment)
          (part_usage variation :>> 'transmission' : 'Transmission' multiplicity
            (variant_usage
              (default_ref_usage 'manualTransmission'))
            (variant_usage
              (default_ref_usage 'automaticTransmission')))
          (sysml_decl
            (result_expr_member))
          (line_comment)
          (part_usage variation :>> 'sunroof'
            (variant_usage
              (part_usage 'withSunroof' multiplicity))
            (variant_usage
              (part_usage 'withoutSunroof' multiplicity)))
          (perform_action :>> 'ActionTree::providePowerFamily')))
      (package_def 'ActionTree'
        (line_comment)
        (action_usage 'providePowerFamily' : 'ProvidePower'
          (action_usage variation 'generateTorque' : 'GenerateTorque'
            (variant_usage
              (default_ref_usage 'generateTorque4Cyl'))
            (variant_usage
              (default_ref_usage 'generateTorque6Cyl')))
          (action_usage variation 'amplifyTorque' : 'AmplifyTorque'
            (variant_usage
              (default_ref_usage 'amplifyTorqueManual'))
            (variant_usage
              (default_ref_usage 'amplifyTorqueAutomatic')))
          (sysml_decl
            (result_expr_member)))))
    (package_def ''100% Model''
      (import_decl private ''150% Model'::*')
      (line_comment)
      (part_usage 'vehicle4Cyl' :> 'PartsTree::vehicleFamily'
        (part_usage :>> 'engine' value)
        (part_usage :>> 'transmission' value)
        (part_usage :>> 'sunroof' value)
        (perform_action :>> 'providePowerFamily'
          (action_usage :>> 'generateTorque' value)
          (action_usage :>> 'amplifyTorque' value))))))
~~~
# FORMAT
~~~sysml
package VehicleVariabilityModel {

	package DesignModel {
		public import PartDefinitions::*;
		public import PartsTree::*;
		public import ActionDefinitions::*;
		public import ActionTree::*;
	
		package PartDefinitions {
			part def Vehicle;
			
		    attribute def Diameter;
		    part def Cylinder {
		        attribute diameter : Diameter[1];
		    }
		
		    part def Engine;
		    part def Transmission;
		    part def Sunroof;
		
		    port def AutoPort;
	    }
	    
	    package PartsTree {
	    	part vehicle : Vehicle {
	    		part engine : Engine[1];
	    		part transmission : Transmission[1];
	    		part sunroof : Sunroof[0..1];
	    	}
	    	
		    part engine : Engine {
		        port autoPort : AutoPort;
		        part cylinder : Cylinder[2..*];
		    }
		    
		    part '4cylEngine' :> engine {
		    	part :>> cylinder[4];
		    }
		    
		    part '6cylEngine' :> engine {
		    	part :>> cylinder[6];
		    }
		
			part transmission : Transmission;
		    part manualTransmission :> transmission;
		    part automaticTransmission :> transmission;
	    }
	
		package ActionDefinitions {   
		    action def GenerateTorque;
		    action def AmplifyTorque;
		    action def ProvidePower;
	    }
	    
	    package ActionTree {    
		    action generateTorque4Cyl : GenerateTorque;
		    action generateTorque6Cyl : GenerateTorque;
		    
		    action amplifyTorqueManual : AmplifyTorque;
		    action amplifyTorqueAutomatic : AmplifyTorque;
	    }	
	}
		
	package '150% Model' {
		private import DesignModel::*;
	
		package PartsTree {
		
		    // Variation point definitions
		
		    variation attribute def DiameterChoices :> Diameter {
		    	variant attribute diameterSmall;
		    	variant attribute diameterLarge;
		    }
		
		    variation part def EngineChoices :> Engine {
		        variant '4cylEngine';
		        variant '6cylEngine' {
		        	variation port :>> autoPort {
		        		variant port autoPort1;
		        		variant port autoPort2;
		        	}
		        	
		        	part :>> cylinder {
		        		attribute :>> diameter : DiameterChoices;
		        	}
		        	
		          	assert constraint {
		            	(autoPort == autoPort::autoPort1 and cylinder.diameter == cylinder::diameter::diameterSmall) xor
		             	(autoPort == autoPort::autoPort2 and cylinder.diameter == cylinder::diameter::diameterLarge)
		        	}
		        }
		    }
		
		    // Part superset model
		    
		    abstract part vehicleFamily :> vehicle {
		    	// Variation point usage
		        part :>> engine : EngineChoices[1];
		        
		        // Variation point with embedded variant definitions
		        variation part :>> transmission : Transmission[1] {
		        	variant manualTransmission;
		        	variant automaticTransmission;
		        }
		        
		        assert constraint {
		            (engine == engine::'4cylEngine' and transmission == transmission::manualTransmission) xor
		            (engine == engine::'6cylEngine' and transmission == transmission::automaticTransmission)
		        }
		        
		        // Variation point on variant multiplicity (inherited multiplicity is [0..1]) 
		        variation part :>> sunroof {
		        	variant part withSunroof[1];
		        	variant part withoutSunroof[0];
		        }
		        
		        perform ActionTree::providePowerFamily;
		    }
		}
		
		package ActionTree {
		
		    // Action superset Model
		    
		    action providePowerFamily : ProvidePower {
		        variation action generateTorque : GenerateTorque {
		        	variant generateTorque4Cyl;
		        	variant generateTorque6Cyl;
		        }
		        
		        variation action amplifyTorque : AmplifyTorque {
		        	variant amplifyTorqueManual;
		        	variant amplifyTorqueAutomatic;
		        }
		        
			    assert constraint {
			        (generateTorque == generateTorque::generateTorque4Cyl and 
			        	amplifyTorque == amplifyTorque::amplifyTorqueManual
			        ) xor
			        (generateTorque == generateTorque::generateTorque6Cyl and 
			        	amplifyTorque == amplifyTorque::amplifyTorqueAutomatic
			        )
			    }		   
		    }		    
		}
	}
	
	package '100% Model' {
		private import '150% Model'::*;
		
		// Vehicle instance model
		
	    part vehicle4Cyl :> PartsTree::vehicleFamily {
	        part :>> engine = engine::'4cylEngine';
	        part :>> transmission = transmission::manualTransmission;
	        part :>> sunroof = sunroof::withoutSunroof;
	        
	        perform action :>> providePowerFamily {
	            action :>> generateTorque = generateTorque::generateTorque4Cyl;
	            action :>> amplifyTorque = amplifyTorque::amplifyTorqueManual;
	        }
	    }
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'autoPort'
semantic.unresolved_name 'cylinder'
semantic.unresolved_name 'diameter'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'autoPort'
semantic.unresolved_name 'cylinder'
semantic.unresolved_name 'diameter'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel"))) (name "VehicleVariabilityModel") (declared-name "VehicleVariabilityModel")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (name "100% Model") (declared-name "100% Model")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (name "engine") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "engine::4cylEngine")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (role feature-value))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (name "sunroof") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "sunroof::withoutSunroof")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (role feature-value))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (name "transmission") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "transmission::manualTransmission")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (role feature-value))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))) (name "150% Model") (declared-name "150% Model")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree"))) (name "ActionTree") (declared-name "ActionTree")
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (name "providePowerFamily") (declared-name "providePowerFamily") (declared)
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (name "amplifyTorque") (declared-name "amplifyTorque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
              (contains
                (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::DiameterChoices"))) (name "DiameterChoices") (declared-name "DiameterChoices"))
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (name "EngineChoices") (declared-name "EngineChoices") (declared (properties (variation true)))
                  (contains
                    (element (kind "variant") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (effective (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (name "vehicleFamily") (declared-name "vehicleFamily") (declared (properties (abstract true) (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (name "sunroof") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withSunroof"))) (name "withSunroof") (declared-name "withSunroof") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withoutSunroof"))) (name "withoutSunroof") (declared-name "withoutSunroof") (declared (properties (ordered false)) (multiplicity (lower 0) (upper 0) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (variation true) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "variant") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::automaticTransmission"))) (name "automaticTransmission") (declared-name "automaticTransmission") (effective (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
                        (element (kind "variant") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::manualTransmission"))) (name "manualTransmission") (declared-name "manualTransmission") (effective (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (name "DesignModel") (declared-name "DesignModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))) (name "ActionDefinitions") (declared-name "ActionDefinitions")
              (contains
                (element (kind "action def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))) (name "AmplifyTorque") (declared-name "AmplifyTorque"))
                (element (kind "action def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque"))
                (element (kind "action def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower"))) (name "ProvidePower") (declared-name "ProvidePower"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (name "ActionTree") (declared-name "ActionTree")
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (name "amplifyTorqueAutomatic") (declared-name "amplifyTorqueAutomatic") (declared))
                (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (name "amplifyTorqueManual") (declared-name "amplifyTorqueManual") (declared))
                (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (name "generateTorque4Cyl") (declared-name "generateTorque4Cyl") (declared))
                (element (kind "action") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (name "generateTorque6Cyl") (declared-name "generateTorque6Cyl") (declared))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))) (name "PartDefinitions") (declared-name "PartDefinitions")
              (contains
                (element (kind "port def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))) (name "AutoPort") (declared-name "AutoPort")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort::~AutoPort"))) (name "~AutoPort") (declared-name "~AutoPort") (effective (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared)
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder")))))
                  )
                )
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))) (name "Diameter") (declared-name "Diameter") (declared (properties (ordered false) (unique true))))
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))) (name "Engine") (declared-name "Engine") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof"))) (name "Sunroof") (declared-name "Sunroof") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (declared (properties (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (name "6cylEngine") (declared-name "6cylEngine") (declared (properties (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (name "automaticTransmission") (declared-name "automaticTransmission") (declared (properties (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (name "autoPort") (declared-name "autoPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (name "cylinder") (declared-name "cylinder") (declared (properties (ordered false)) (multiplicity (lower 2) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (name "manualTransmission") (declared-name "manualTransmission") (declared (properties (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (name "sunroof") (declared-name "sunroof") (declared (properties (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort::~AutoPort"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (to (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_variability_model.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 31 10) (end 31 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 7) (end 36 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 40 7) (end 40 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 64 17) (end 64 28))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 76 10) (end 76 31))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 77 10) (end 77 566))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 77 10) (end 77 566))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 102 11) (end 102 38))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 103 11) (end 103 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 112 10) (end 112 133))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 127 11) (end 127 50))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 128 11) (end 128 49))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 132 11) (end 132 51))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 133 11) (end 133 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 149 17) (end 149 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 154 9) (end 154 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 155 9) (end 155 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 156 9) (end 156 52))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 158 9) (end 158 218))
      )
    )
  )
)
~~~
