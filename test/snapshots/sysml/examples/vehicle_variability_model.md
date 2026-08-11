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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "bb01e5e719ef04118c986f4fc7ddd9e2a384fb3565a547fdd95a8a9f887b4c55") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel"))) (kind "package") (name "VehicleVariabilityModel") (declared-name "VehicleVariabilityModel") (range (start (line 0) (character 0)) (end (line 0) (character 4718))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (kind "package") (name "100% Model") (declared-name "100% Model") (range (start (line 148) (character 1)) (end (line 148) (character 545))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 149) (character 2)) (end (line 149) (character 33))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (authored (membership (kind Import) (visibility "private") (import (reference "150% Model::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 149) (character 17)) (end (line 149) (character 29))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (kind "part") (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (range (start (line 153) (character 5)) (end (line 153) (character 450))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "PartsTree::vehicleFamily") (range (start (line 153) (character 25)) (end (line 153) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (kind "part") (name "engine") (range (start (line 154) (character 9)) (end (line 154) (character 48))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 154) (character 18)) (end (line 154) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (kind "part") (name "sunroof") (range (start (line 156) (character 9)) (end (line 156) (character 52))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sunroof") (range (start (line 156) (character 18)) (end (line 156) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (kind "part") (name "transmission") (range (start (line 155) (character 9)) (end (line 155) (character 66))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 155) (character 18)) (end (line 155) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))) (kind "package") (name "150% Model") (declared-name "150% Model") (range (start (line 63) (character 1)) (end (line 63) (character 2691))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 64) (character 2)) (end (line 64) (character 32))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 64) (character 17)) (end (line 64) (character 28))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree"))) (kind "package") (name "ActionTree") (declared-name "ActionTree") (range (start (line 121) (character 2)) (end (line 121) (character 799))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind "action") (name "providePowerFamily") (declared-name "providePowerFamily") (range (start (line 125) (character 6)) (end (line 125) (character 725))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower") (range none)) (perform (reference "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque") (range none)) (perform (reference "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (kind "action") (name "amplifyTorque") (declared-name "amplifyTorque") (range (start (line 131) (character 10)) (end (line 131) (character 153))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 126) (character 10)) (end (line 126) (character 150))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (range (start (line 66) (character 2)) (end (line 66) (character 1826))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::DiameterChoices"))) (kind "kermlDecl") (name "DiameterChoices") (declared-name "DiameterChoices") (range (start (line 70) (character 6)) (end (line 70) (character 147))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices") (range (start (line 75) (character 6)) (end (line 75) (character 650))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 75) (character 42)) (end (line 75) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices::4cylEngine"))) (kind "variant") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 76) (character 10)) (end (line 76) (character 31))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (kind "part") (name "vehicleFamily") (declared-name "vehicleFamily") (range (start (line 96) (character 6)) (end (line 96) (character 916))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 96) (character 37)) (end (line 96) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 98) (character 10)) (end (line 98) (character 45))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineChoices") (range (start (line 98) (character 28)) (end (line 98) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (kind "part") (name "sunroof") (range (start (line 112) (character 10)) (end (line 112) (character 133))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sunroof") (range (start (line 112) (character 29)) (end (line 112) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withSunroof"))) (kind "part") (name "withSunroof") (declared-name "withSunroof") (range (start (line 113) (character 19)) (end (line 113) (character 39))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof::withoutSunroof"))) (kind "part") (name "withoutSunroof") (declared-name "withoutSunroof") (range (start (line 114) (character 19)) (end (line 114) (character 42))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 101) (character 10)) (end (line 101) (character 154))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 101) (character 44)) (end (line 101) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::automaticTransmission"))) (kind "variant") (name "automaticTransmission") (declared-name "automaticTransmission") (range (start (line 103) (character 11)) (end (line 103) (character 41))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission::manualTransmission"))) (kind "variant") (name "manualTransmission") (declared-name "manualTransmission") (range (start (line 102) (character 11)) (end (line 102) (character 38))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (range (start (line 2) (character 1)) (end (line 2) (character 1438))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 2)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "PartDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 31))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 2)) (end (line 4) (character 29))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "PartsTree::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 25))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 2)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "ActionDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 33))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 2)) (end (line 6) (character 30))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "ActionTree::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 26))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))) (kind "package") (name "ActionDefinitions") (declared-name "ActionDefinitions") (range (start (line 48) (character 2)) (end (line 48) (character 135))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque"))) (kind "action def") (name "AmplifyTorque") (declared-name "AmplifyTorque") (range (start (line 50) (character 6)) (end (line 50) (character 31))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (range (start (line 49) (character 6)) (end (line 49) (character 32))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (range (start (line 51) (character 6)) (end (line 51) (character 30))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (kind "package") (name "ActionTree") (declared-name "ActionTree") (range (start (line 54) (character 5)) (end (line 54) (character 246))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (kind "action") (name "amplifyTorqueAutomatic") (declared-name "amplifyTorqueAutomatic") (range (start (line 59) (character 6)) (end (line 59) (character 52))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (kind "action") (name "amplifyTorqueManual") (declared-name "amplifyTorqueManual") (range (start (line 58) (character 6)) (end (line 58) (character 49))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmplifyTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (kind "action") (name "generateTorque4Cyl") (declared-name "generateTorque4Cyl") (range (start (line 55) (character 6)) (end (line 55) (character 49))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (kind "action") (name "generateTorque6Cyl") (declared-name "generateTorque6Cyl") (range (start (line 56) (character 6)) (end (line 56) (character 49))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))) (kind "package") (name "PartDefinitions") (declared-name "PartDefinitions") (range (start (line 8) (character 2)) (end (line 8) (character 274))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))) (kind "port def") (name "AutoPort") (declared-name "AutoPort") (range (start (line 20) (character 6)) (end (line 20) (character 24))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort::~AutoPort"))) (kind "conjugated port definition") (name "~AutoPort") (declared-name "~AutoPort") (range (start (line 20) (character 6)) (end (line 20) (character 24))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 12) (character 6)) (end (line 12) (character 77))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (range (start (line 13) (character 10)) (end (line 13) (character 43))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder"))) (authored (membership (kind Feature)) (relationships (typing (reference "Diameter") (range none)) (typing (reference "Diameter") (range (start (line 13) (character 31)) (end (line 13) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter"))) (kind "attribute def") (name "Diameter") (declared-name "Diameter") (range (start (line 11) (character 6)) (end (line 11) (character 29))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 16) (character 6)) (end (line 16) (character 22))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof"))) (kind "part def") (name "Sunroof") (declared-name "Sunroof") (range (start (line 18) (character 6)) (end (line 18) (character 23))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 17) (character 6)) (end (line 17) (character 28))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 9) (character 3)) (end (line 9) (character 20))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (range (start (line 23) (character 5)) (end (line 23) (character 602))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 35) (character 6)) (end (line 35) (character 72))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine") (range (start (line 35) (character 27)) (end (line 35) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (kind "part") (name "cylinder") (range (start (line 36) (character 7)) (end (line 36) (character 28))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder") (range (start (line 36) (character 16)) (end (line 36) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (range (start (line 39) (character 6)) (end (line 39) (character 72))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine") (range (start (line 39) (character 27)) (end (line 39) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (kind "part") (name "cylinder") (range (start (line 40) (character 7)) (end (line 40) (character 28))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cylinder") (range (start (line 40) (character 16)) (end (line 40) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (range (start (line 45) (character 6)) (end (line 45) (character 49))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "transmission") (range (start (line 45) (character 36)) (end (line 45) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 30) (character 6)) (end (line 30) (character 114))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 30) (character 20)) (end (line 30) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (kind "port") (name "autoPort") (declared-name "autoPort") (range (start (line 31) (character 10)) (end (line 31) (character 35))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutoPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (kind "part") (name "cylinder") (declared-name "cylinder") (range (start (line 32) (character 10)) (end (line 32) (character 41))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 32) (character 26)) (end (line 32) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (range (start (line 44) (character 6)) (end (line 44) (character 46))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "transmission") (range (start (line 44) (character 33)) (end (line 44) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 43) (character 3)) (end (line 43) (character 36))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 43) (character 23)) (end (line 43) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 24) (character 6)) (end (line 24) (character 151))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 24) (character 21)) (end (line 24) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 25) (character 7)) (end (line 25) (character 31))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 25) (character 21)) (end (line 25) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (kind "part") (name "sunroof") (declared-name "sunroof") (range (start (line 27) (character 7)) (end (line 27) (character 36))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sunroof") (range (start (line 27) (character 22)) (end (line 27) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 26) (character 7)) (end (line 26) (character 43))) (parent (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 26) (character 27)) (end (line 26) (character 39)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "150% Model::*") (range (start (line 149) (character 17)) (end (line 149) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl"))) (kind subsetting) (ordinal 0)) (authored-target "PartsTree::vehicleFamily") (range (start (line 153) (character 25)) (end (line 153) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 154) (character 18)) (end (line 154) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof"))) (kind redefinition) (ordinal 0)) (authored-target "sunroof") (range (start (line 156) (character 18)) (end (line 156) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 155) (character 18)) (end (line 155) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::100% Model::vehicle4Cyl::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (range (start (line 64) (character 17)) (end (line 64) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 0)) (authored-target "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily"))) (kind performSource) (ordinal 1)) (authored-target "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::amplifyTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::ActionTree::providePowerFamily::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 75) (character 42)) (end (line 75) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 96) (character 37)) (end (line 96) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineChoices") (range (start (line 98) (character 28)) (end (line 98) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::EngineChoices")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof"))) (kind redefinition) (ordinal 0)) (authored-target "sunroof") (range (start (line 112) (character 29)) (end (line 112) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::150% Model::PartsTree::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 101) (character 44)) (end (line 101) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartDefinitions::*") (range (start (line 3) (character 16)) (end (line 3) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartsTree::*") (range (start (line 4) (character 16)) (end (line 4) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ActionDefinitions::*") (range (start (line 5) (character 16)) (end (line 5) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ActionTree::*") (range (start (line 6) (character 16)) (end (line 6) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueAutomatic"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::amplifyTorqueManual"))) (kind featureTyping) (ordinal 0)) (authored-target "AmplifyTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::AmplifyTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque4Cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionTree::generateTorque6Cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::ActionDefinitions::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "Diameter") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "Diameter") (range (start (line 13) (character 31)) (end (line 13) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (range (start (line 35) (character 27)) (end (line 35) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (range (start (line 36) (character 16)) (end (line 36) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::4cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (range (start (line 39) (character 27)) (end (line 39) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder"))) (kind redefinition) (ordinal 0)) (authored-target "cylinder") (range (start (line 40) (character 16)) (end (line 40) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::6cylEngine::cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::automaticTransmission"))) (kind subsetting) (ordinal 0)) (authored-target "transmission") (range (start (line 45) (character 36)) (end (line 45) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 30) (character 20)) (end (line 30) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::autoPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AutoPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::AutoPort")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::engine::cylinder"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 32) (character 26)) (end (line 32) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::manualTransmission"))) (kind subsetting) (ordinal 0)) (authored-target "transmission") (range (start (line 44) (character 33)) (end (line 44) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 43) (character 23)) (end (line 43) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 24) (character 21)) (end (line 24) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 25) (character 21)) (end (line 25) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::sunroof"))) (kind featureTyping) (ordinal 0)) (authored-target "Sunroof") (range (start (line 27) (character 22)) (end (line 27) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Sunroof")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartsTree::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 26) (character 27)) (end (line 26) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleVariabilityModel::DesignModel::PartDefinitions::Transmission")))))
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
