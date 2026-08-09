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
                attribute diameter : Diameter [1];
            }

            part def Engine;
            part def Transmission;
            part def Sunroof;

            port def AutoPort;
        }

        package PartsTree {
            part vehicle : Vehicle {
                part engine : Engine [1];
                part transmission : Transmission [1];
                part sunroof : Sunroof [0..1];
            }

            part engine : Engine {
                port autoPort : AutoPort;
                part cylinder : Cylinder [2..*];
            }

            part '4cylEngine' :> engine {
                part :>> cylinder [4];
            }

            part '6cylEngine' :> engine {
                part :>> cylinder [6];
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
                part :>> engine : EngineChoices [1];

                // Variation point with embedded variant definitions
                variation part :>> transmission : Transmission [1] {
                    variant manualTransmission;
                    variant automaticTransmission;
                }

                assert constraint {
                    = (engine == engine::'4cylEngine' and transmission == transmission::manualTransmission) xor (engine == engine::'6cylEngine' and transmission == transmission::automaticTransmission);
                }

                // Variation point on variant multiplicity (inherited multiplicity is [0..1]) 
                variation part :>> sunroof {
                    variant part withSunroof[1];
                    variant part withoutSunroof[0];
                }

                perform :>> ActionTree::providePowerFamily;
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
                    = (generateTorque == generateTorque::generateTorque4Cyl and amplifyTorque == amplifyTorque::amplifyTorqueManual) xor (generateTorque == generateTorque::generateTorque6Cyl and amplifyTorque == amplifyTorque::amplifyTorqueAutomatic);
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
(model
  (namespace
    (package 'VehicleVariabilityModel'
      (package 'DesignModel'
        (namespace_import public -> 'VehicleVariabilityModel::DesignModel::PartDefinitions'[package])
        (namespace_import public -> 'VehicleVariabilityModel::DesignModel::PartsTree'[package])
        (namespace_import public -> 'VehicleVariabilityModel::DesignModel::ActionDefinitions'[package])
        (namespace_import public -> 'VehicleVariabilityModel::DesignModel::ActionTree'[package])
        (package 'PartDefinitions'
          (part_def 'Vehicle')
          (attribute_def 'Diameter')
          (part_def 'Cylinder'
            (attribute_usage composite 'diameter' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter'[attribute_def]
              (multiplicity_range [1])))
          (part_def 'Engine')
          (part_def 'Transmission')
          (part_def 'Sunroof')
          (port_def 'AutoPort'))
        (package 'PartsTree'
          (part_usage 'vehicle' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle'[part_def]
            (part_usage composite 'engine' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Engine'[part_def]
              (multiplicity_range [1]))
            (part_usage composite 'transmission' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission'[part_def]
              (multiplicity_range [1]))
            (part_usage composite 'sunroof' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof'[part_def]
              (multiplicity_range [0..1])))
          (part_usage 'engine' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Engine'[part_def]
            (port_usage composite 'autoPort' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort'[port_def])
            (part_usage composite 'cylinder' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder'[part_def]
              (multiplicity_range [2..*])))
          (part_usage '4cylEngine' :> 'VehicleVariabilityModel::DesignModel::PartsTree::engine'[part_usage]
            (part_usage composite :>> 'VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder'[part_usage]
              (multiplicity_range [4])))
          (part_usage '6cylEngine' :> 'VehicleVariabilityModel::DesignModel::PartsTree::engine'[part_usage]
            (part_usage composite :>> 'VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder'[part_usage]
              (multiplicity_range [6])))
          (part_usage 'transmission' : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission'[part_def])
          (part_usage 'manualTransmission' :> 'VehicleVariabilityModel::DesignModel::PartsTree::transmission'[part_usage])
          (part_usage 'automaticTransmission' :> 'VehicleVariabilityModel::DesignModel::PartsTree::transmission'[part_usage]))
        (package 'ActionDefinitions'
          (action_def 'GenerateTorque')
          (action_def 'AmplifyTorque')
          (action_def 'ProvidePower'))
        (package 'ActionTree'
          (action_usage 'generateTorque4Cyl' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque'[action_def])
          (action_usage 'generateTorque6Cyl' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque'[action_def])
          (action_usage 'amplifyTorqueManual' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque'[action_def])
          (action_usage 'amplifyTorqueAutomatic' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque'[action_def])))
      (package '150% Model'
        (namespace_import private -> 'VehicleVariabilityModel::DesignModel'[package])
        (package 'PartsTree'
          (attribute_def variation 'DiameterChoices' :> 'VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter'[attribute_def]
            (variant_usage
              (attribute_usage composite 'diameterSmall'))
            (variant_usage
              (attribute_usage composite 'diameterLarge')))
          (part_def variation 'EngineChoices' :> 'VehicleVariabilityModel::DesignModel::PartDefinitions::Engine'[part_def]
            (variant_usage
              (reference_usage reference '4cylEngine'))
            (variant_usage
              (reference_usage reference '6cylEngine'
                (port_usage variation composite :>> 'autoPort'[unresolved]
                  (variant_usage
                    (port_usage composite 'autoPort1'))
                  (variant_usage
                    (port_usage composite 'autoPort2')))
                (part_usage composite :>> 'cylinder'[unresolved]
                  (attribute_usage composite :>> 'diameter'[unresolved] : 'VehicleVariabilityModel::150% Model::PartsTree::DiameterChoices'[attribute_def]))
                (assert_constraint_usage
                  (result_expr_membership)))))
          (part_usage abstract 'vehicleFamily' :> 'VehicleVariabilityModel::DesignModel::PartsTree::vehicle'[part_usage]
            (part_usage composite :>> 'VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine'[part_usage] : 'VehicleVariabilityModel::150% Model::PartsTree::EngineChoices'[part_def]
              (multiplicity_range [1]))
            (part_usage variation composite :>> 'VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission'[part_usage] : 'VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission'[part_def]
              (multiplicity_range [1])
              (variant_usage
                (reference_usage reference 'manualTransmission'))
              (variant_usage
                (reference_usage reference 'automaticTransmission')))
            (assert_constraint_usage
              (result_expr_membership))
            (part_usage variation composite :>> 'VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof'[part_usage]
              (variant_usage
                (part_usage composite 'withSunroof'
                  (multiplicity_range [1])))
              (variant_usage
                (part_usage composite 'withoutSunroof'
                  (multiplicity_range [0]))))
            (perform_action_usage :>> 'VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily'[action_usage])))
        (package 'ActionTree'
          (action_usage 'providePowerFamily' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower'[action_def]
            (action_usage variation composite 'generateTorque' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque'[action_def]
              (variant_usage
                (reference_usage reference 'generateTorque4Cyl'))
              (variant_usage
                (reference_usage reference 'generateTorque6Cyl')))
            (action_usage variation composite 'amplifyTorque' : 'VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque'[action_def]
              (variant_usage
                (reference_usage reference 'amplifyTorqueManual'))
              (variant_usage
                (reference_usage reference 'amplifyTorqueAutomatic')))
            (assert_constraint_usage
              (result_expr_membership)))))
      (package '100% Model'
        (namespace_import private -> 'VehicleVariabilityModel::150% Model'[package])
        (part_usage 'vehicle4Cyl' :> 'VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily'[part_usage]
          (part_usage composite :>> ''[part_usage]
            (feature_value (=)))
          (part_usage composite :>> ''[part_usage]
            (feature_value (=)))
          (part_usage composite :>> ''[part_usage]
            (feature_value (=)))
          (perform_action_usage :>> ''[perform_action_usage]
            (action_usage :>> 'VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque'[action_usage]
              (feature_value (=)))
            (action_usage :>> 'VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque'[action_usage]
              (feature_value (=)))))))))
~~~
