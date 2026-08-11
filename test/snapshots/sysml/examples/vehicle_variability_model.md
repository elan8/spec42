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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_variability_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 64 17) (end 64 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 75 42) (end 75 48))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 37) (end 96 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 44) (end 101 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 6) (end 125 725))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 126 10) (end 126 150))
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 10) (end 131 153))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 153 25) (end 153 49))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "bb01e5e719ef04118c986f4fc7ddd9e2a384fb3565a547fdd95a8a9f887b4c55") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel"))) (kind "package") (name "VehicleVariabilityModel") (declared-name "VehicleVariabilityModel"))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (kind "package") (name "100% Model") (declared-name "100% Model") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (authored (membership (kind Import) (visibility "private") (import (reference "150% Model::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (kind "part") (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "PartsTree::vehicleFamily")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (kind "part") (name "engine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (kind "part") (name "sunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sunroof")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (kind "part") (name "transmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))) (kind "package") (name "150% Model") (declared-name "150% Model") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree"))) (kind "package") (name "ActionTree") (declared-name "ActionTree") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind "action") (name "providePowerFamily") (declared-name "providePowerFamily") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower")) (perform (reference "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque")) (perform (reference "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (kind "action") (name "amplifyTorque") (declared-name "amplifyTorque") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::DiameterChoices"))) (kind "kermlDecl") (name "DiameterChoices") (declared-name "DiameterChoices") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices::4cylEngine"))) (kind "variant") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (kind "part") (name "vehicleFamily") (declared-name "vehicleFamily") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineChoices")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (kind "part") (name "sunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sunroof")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withSunroof"))) (kind "part") (name "withSunroof") (declared-name "withSunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withoutSunroof"))) (kind "part") (name "withoutSunroof") (declared-name "withoutSunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::automaticTransmission"))) (kind "variant") (name "automaticTransmission") (declared-name "automaticTransmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::manualTransmission"))) (kind "variant") (name "manualTransmission") (declared-name "manualTransmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "PartDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "PartsTree::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "ActionDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "ActionTree::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))) (kind "package") (name "ActionDefinitions") (declared-name "ActionDefinitions") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))) (kind "action def") (name "AmplifyTorque") (declared-name "AmplifyTorque") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (kind "package") (name "ActionTree") (declared-name "ActionTree") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (kind "action") (name "amplifyTorqueAutomatic") (declared-name "amplifyTorqueAutomatic") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (kind "action") (name "amplifyTorqueManual") (declared-name "amplifyTorqueManual") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (kind "action") (name "generateTorque4Cyl") (declared-name "generateTorque4Cyl") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (kind "action") (name "generateTorque6Cyl") (declared-name "generateTorque6Cyl") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))) (kind "package") (name "PartDefinitions") (declared-name "PartDefinitions") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))) (kind "port def") (name "AutoPort") (declared-name "AutoPort") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort::~AutoPort"))) (kind "conjugated port definition") (name "~AutoPort") (declared-name "~AutoPort") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (authored (membership (kind Feature)) (relationships (typing (reference "Diameter")) (typing (reference "Diameter")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))) (kind "attribute def") (name "Diameter") (declared-name "Diameter") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof"))) (kind "part def") (name "Sunroof") (declared-name "Sunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (kind "part") (name "cylinder") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (kind "part") (name "cylinder") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (kind "port") (name "autoPort") (declared-name "autoPort") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutoPort")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (kind "part") (name "cylinder") (declared-name "cylinder") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (kind "part") (name "sunroof") (declared-name "sunroof") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sunroof")))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "150% Model::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "PartsTree::vehicleFamily") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (kind redefinition) (ordinal 0)) (authored-target "sunroof") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 0)) (authored-target "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 1)) (authored-target "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineChoices") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (kind redefinition) (ordinal 0)) (authored-target "sunroof") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartsTree::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ActionDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ActionTree::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "Diameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "Diameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (kind subsetting) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AutoPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (kind subsetting) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (kind featureTyping) (ordinal 0)) (authored-target "Sunroof") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 25 21) (end 25 27)) (probe (position 25 21))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 25 21) (end 25 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine") (range (start 16 6) (end 16 22)))
        )
      )
    )
    (query (range (start 30 20) (end 30 26)) (probe (position 30 20))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 30 20) (end 30 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine") (range (start 16 6) (end 16 22)))
        )
      )
    )
    (query (range (start 35 27) (end 35 33)) (probe (position 35 27))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))
        (kind subsetting) (ordinal 0) (authored-target "engine")
        (range (start 35 27) (end 35 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine") (range (start 30 6) (end 30 114)))
        )
      )
    )
    (query (range (start 39 27) (end 39 33)) (probe (position 39 27))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))
        (kind subsetting) (ordinal 0) (authored-target "engine")
        (range (start 39 27) (end 39 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine") (range (start 30 6) (end 30 114)))
        )
      )
    )
    (query (range (start 75 42) (end 75 48)) (probe (position 75 42))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 75 42) (end 75 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 154 18) (end 154 24)) (probe (position 154 18))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 154 18) (end 154 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine") (range (start 154 9) (end 154 48)))
        )
      )
    )
    (query (range (start 24 21) (end 24 28)) (probe (position 24 21))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 24 21) (end 24 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle") (range (start 9 3) (end 9 20)))
        )
      )
    )
    (query (range (start 27 22) (end 27 29)) (probe (position 27 22))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))
        (kind featureTyping) (ordinal 0) (authored-target "Sunroof")
        (range (start 27 22) (end 27 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof") (range (start 18 6) (end 18 23)))
        )
      )
    )
    (query (range (start 96 37) (end 96 44)) (probe (position 96 37))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 96 37) (end 96 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 29) (end 112 36)) (probe (position 112 29))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))
        (kind redefinition) (ordinal 0) (authored-target "sunroof")
        (range (start 112 29) (end 112 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof") (range (start 112 10) (end 112 133)))
        )
      )
    )
    (query (range (start 156 18) (end 156 25)) (probe (position 156 18))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))
        (kind redefinition) (ordinal 0) (authored-target "sunroof")
        (range (start 156 18) (end 156 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof") (range (start 156 9) (end 156 52)))
        )
      )
    )
    (query (range (start 13 31) (end 13 39)) (probe (position 13 31))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))
        (kind featureTyping) (ordinal 1) (authored-target "Diameter")
        (range (start 13 31) (end 13 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter") (range (start 11 6) (end 11 29)))
        )
      )
    )
    (query (range (start 32 26) (end 32 34)) (probe (position 32 26))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 32 26) (end 32 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder") (range (start 12 6) (end 12 77)))
        )
      )
    )
    (query (range (start 36 16) (end 36 24)) (probe (position 36 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))
        (kind redefinition) (ordinal 0) (authored-target "cylinder")
        (range (start 36 16) (end 36 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder") (range (start 36 7) (end 36 28)))
        )
      )
    )
    (query (range (start 40 16) (end 40 24)) (probe (position 40 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))
        (kind redefinition) (ordinal 0) (authored-target "cylinder")
        (range (start 40 16) (end 40 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder") (range (start 40 7) (end 40 28)))
        )
      )
    )
    (query (range (start 4 16) (end 4 25)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "PartsTree::*")
        (range (start 4 16) (end 4 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree") (range (start 23 5) (end 23 602)))
        )
      )
    )
    (query (range (start 6 16) (end 6 26)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ActionTree::*")
        (range (start 6 16) (end 6 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree") (range (start 54 5) (end 54 246)))
        )
      )
    )
    (query (range (start 64 17) (end 64 28)) (probe (position 64 17))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "DesignModel::*")
        (range (start 64 17) (end 64 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 27) (end 26 39)) (probe (position 26 27))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 26 27) (end 26 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission") (range (start 17 6) (end 17 28)))
        )
      )
    )
    (query (range (start 43 23) (end 43 35)) (probe (position 43 23))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 43 23) (end 43 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission") (range (start 17 6) (end 17 28)))
        )
      )
    )
    (query (range (start 44 33) (end 44 45)) (probe (position 44 33))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))
        (kind subsetting) (ordinal 0) (authored-target "transmission")
        (range (start 44 33) (end 44 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission") (range (start 43 3) (end 43 36)))
        )
      )
    )
    (query (range (start 45 36) (end 45 48)) (probe (position 45 36))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))
        (kind subsetting) (ordinal 0) (authored-target "transmission")
        (range (start 45 36) (end 45 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission") (range (start 43 3) (end 43 36)))
        )
      )
    )
    (query (range (start 101 44) (end 101 56)) (probe (position 101 44))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 101 44) (end 101 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 149 17) (end 149 29)) (probe (position 149 17))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "150% Model::*")
        (range (start 149 17) (end 149 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 155 18) (end 155 30)) (probe (position 155 18))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 155 18) (end 155 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission") (range (start 155 9) (end 155 66)))
        )
      )
    )
    (query (range (start 98 28) (end 98 41)) (probe (position 98 28))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "EngineChoices")
        (range (start 98 28) (end 98 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices") (range (start 75 6) (end 75 650)))
        )
      )
    )
    (query (range (start 3 16) (end 3 31)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "PartDefinitions::*")
        (range (start 3 16) (end 3 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions") (range (start 8 2) (end 8 274)))
        )
      )
    )
    (query (range (start 5 16) (end 5 33)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ActionDefinitions::*")
        (range (start 5 16) (end 5 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions") (range (start 48 2) (end 48 135)))
        )
      )
    )
    (query (range (start 153 25) (end 153 49)) (probe (position 153 25))
      (reference
        (source (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))
        (kind subsetting) (ordinal 0) (authored-target "PartsTree::vehicleFamily")
        (range (start 153 25) (end 153 49))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
