# META
~~~ini
description=SysML Example (Vehicle): VehicleIndividuals
type=file
~~~
# SOURCE
~~~sysml
package VehicleIndividuals {
	private import VehicleUsages::*;
	private import Time::DateTime;
	private import SI::kg;
	
	package IndividualDefinitions {

		individual part def Vehicle1 :> Vehicle {
			doc
			/*
			 * This is an individual Vehicle with a mass of 1800 kg.
			 */
			
			attribute redefines mass = 1800 [kg];
		}
		
		individual part def Vehicle2 :> Vehicle {
			doc
			/*
			 * This is an individual Vehicle with a mass of 1700 kg.
			 */
		
			attribute redefines mass = 1700 [kg];
		}
		
		individual part def AxleAssembly1 :> AxleAssembly;
		
		individual part def Wheel1 :> Wheel;
		individual part def Wheel2 :> Wheel;
	}
	
	package IndividualSnapshots {
		public import IndividualDefinitions::*;
		private import Occurrences::HappensJustBefore;
	
		attribute t0: DateTime;
		attribute t1: DateTime;
		
		individual part vehicle1 : Vehicle1 {
    		snapshot vehicle1_t0 {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 at time t0;
    			 */
    		
    			attribute :>> localClock.currentTime = t0;
    		}
    		
    		succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;
    		
    		timeslice vehicle1_t0_t1 {
    			doc
    			/*
    			 * This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 
    			 * (time t0) and ending at time t1.
    			 */
    		
    			snapshot :>> done {
    				attribute :>> localClock.currentTime = t1;
    			}
    		}
		}	
	}
	
	package IndividualConfigurations {
		public import IndividualSnapshots::*;
	
		individual part vehicle1_C2: Vehicle1 :> vehicle_C2, vehicle1 {
			doc
			/*
			 * This asserts that for some portion of its lifetime, Vehicle1 conforms
			 * to the configuration vehicle_C2;
			 */
			
    		snapshot vehicle1_C2_t0 :> vehicle1_t0 {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.
    			 */
    		
    			individual axleAssembly1_t0: AxleAssembly1 :>> frontAxleAssembly {
    				doc
    				/*
    				 * frontAxleAssembly is a feature of vehicle1_C2.
    				 */
    			
    				individual leftFrontWheel_t0: Wheel1 :>> leftFrontWheel {
    					doc
    					/*
    					 * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0
    					 * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).
    					 */
    				}
    			}
    		}
		
    		snapshot vehicle1_C2_t1 :> vehicle1_t0_t1.done {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.
    			 */
    		
    			individual axleAssembly1_t1: AxleAssembly1 :>> frontAxleAssembly {
    				individual rightFrontWheel_t1: Wheel1 :>> rightFrontWheel {
    					doc
    					/*
    					 * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.
    					 */
    				}
    			}
    		}	
	       
        }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_individuals.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 34) (end 7 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 34) (end 16 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 25 39) (end 25 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 27 32) (end 27 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 32) (end 28 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 16) (end 32 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 17) (end 33 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 29) (end 38 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 21) (end 45 43))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 48 6) (end 48 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 20) (end 57 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 22) (end 58 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 65 16) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 31) (end 67 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 43) (end 67 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 55) (end 67 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 33) (end 74 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 18) (end 80 449))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 54) (end 80 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 19) (end 86 265))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 86 49) (end 86 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 33) (end 96 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 18) (end 102 276))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 54) (end 102 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 19) (end 103 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 103 50) (end 103 65))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleIndividuals {
    private import VehicleUsages::*;
    private import Time::DateTime;
    private import SI::kg;

    package IndividualDefinitions {

        individual part def Vehicle1 :> Vehicle {
            doc
            /*
			 * This is an individual Vehicle with a mass of 1800 kg.
			 */

            attribute redefines mass = 1800 [kg];
        }

        individual part def Vehicle2 :> Vehicle {
            doc
            /*
			 * This is an individual Vehicle with a mass of 1700 kg.
			 */

            attribute redefines mass = 1700 [kg];
        }

        individual part def AxleAssembly1 :> AxleAssembly;

        individual part def Wheel1 :> Wheel;
        individual part def Wheel2 :> Wheel;
    }

    package IndividualSnapshots {
        public import IndividualDefinitions::*;
        private import Occurrences::HappensJustBefore;

        attribute t0: DateTime;
        attribute t1: DateTime;

        individual part vehicle1 : Vehicle1 {
            snapshot vehicle1_t0 {
                doc
                /*
    			 * This is a snapshot of Vehicle1 at time t0;
    			 */

                attribute :>> localClock.currentTime = t0;
            }

            succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;

            timeslice vehicle1_t0_t1 {
                doc
                /*
    			 * This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 
    			 * (time t0) and ending at time t1.
    			 */

                snapshot :>> done {
                    attribute :>> localClock.currentTime = t1;
                }
            }
        }
    }

    package IndividualConfigurations {
        public import IndividualSnapshots::*;

        individual part vehicle1_C2: Vehicle1 :> vehicle_C2, vehicle1 {
            doc
            /*
			 * This asserts that for some portion of its lifetime, Vehicle1 conforms
			 * to the configuration vehicle_C2;
			 */

            snapshot vehicle1_C2_t0 :> vehicle1_t0 {
                doc
                /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.
    			 */

                individual axleAssembly1_t0: AxleAssembly1 :>> frontAxleAssembly {
                    doc
                    /*
    				 * frontAxleAssembly is a feature of vehicle1_C2.
    				 */

                    individual leftFrontWheel_t0: Wheel1 :>> leftFrontWheel {
                        doc
                        /*
    					 * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0
    					 * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).
    					 */
                    }
                }
            }

            snapshot vehicle1_C2_t1 :> vehicle1_t0_t1.done {
                doc
                /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.
    			 */

                individual axleAssembly1_t1: AxleAssembly1 :>> frontAxleAssembly {
                    individual rightFrontWheel_t1: Wheel1 :>> rightFrontWheel {
                        doc
                        /*
    					 * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.
    					 */
                    }
                }
            }

        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "57a160d1c92ae1f064624b7fc52d15ae95accb75035152408636d88ac59a8154") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals"))) (kind "package") (name "VehicleIndividuals") (declared-name "VehicleIndividuals"))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleUsages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (kind "package") (name "IndividualConfigurations") (declared-name "IndividualConfigurations") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (authored (membership (kind Import) (visibility "public") (import (reference "IndividualSnapshots::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind "part") (name "vehicle1_C2") (declared-name "vehicle1_C2") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle1")) (subsetting (reference "vehicle_C2")) (subsetting (reference "vehicle1")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind "occurrence") (name "vehicle1_C2_t0") (declared-name "vehicle1_C2_t0") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_t0")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind "occurrence") (name "axleAssembly1_t0") (declared-name "axleAssembly1_t0") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly1")) (redefinition (reference "frontAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind "occurrence") (name "leftFrontWheel_t0") (declared-name "leftFrontWheel_t0") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel1")) (redefinition (reference "leftFrontWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind "occurrence") (name "vehicle1_C2_t1") (declared-name "vehicle1_C2_t1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_t0_t1.done")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind "occurrence") (name "axleAssembly1_t1") (declared-name "axleAssembly1_t1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly1")) (redefinition (reference "frontAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind "occurrence") (name "rightFrontWheel_t1") (declared-name "rightFrontWheel_t1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel1")) (redefinition (reference "rightFrontWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (kind "package") (name "IndividualDefinitions") (declared-name "IndividualDefinitions") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind "part def") (name "AxleAssembly1") (declared-name "AxleAssembly1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind "part def") (name "Vehicle1") (declared-name "Vehicle1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind "part def") (name "Vehicle2") (declared-name "Vehicle2") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind "part def") (name "Wheel1") (declared-name "Wheel1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind "part def") (name "Wheel2") (declared-name "Wheel2") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (kind "package") (name "IndividualSnapshots") (declared-name "IndividualSnapshots") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Import) (visibility "public") (import (reference "IndividualDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))) (kind "import") (name "HappensJustBefore") (declared-name "HappensJustBefore") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensJustBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind "attribute def") (name "t0") (declared-name "t0") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Owning)) (relationships (typing (reference "DateTime")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind "attribute def") (name "t1") (declared-name "t1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Owning)) (relationships (typing (reference "DateTime")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle1")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (kind "occurrence") (name "vehicle1_t0") (declared-name "vehicle1_t0") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (kind "occurrence") (name "vehicle1_t0_t1") (declared-name "vehicle1_t0_t1") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime")))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::kg"))) (kind "import") (name "kg") (declared-name "kg") (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleUsages::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "IndividualSnapshots::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C2") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 1)) (authored-target "vehicle1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_t0") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0)) (authored-target "leftFrontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_t0_t1.done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0)) (authored-target "rightFrontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind specialization) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "IndividualDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensJustBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (kind redefinition) (ordinal 0)) (authored-target "done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 13 23) (end 13 27)) (probe (position 13 23))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 13 23) (end 13 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass") (range (start 13 3) (end 13 40)))
        )
      )
    )
    (query (range (start 22 23) (end 22 27)) (probe (position 22 23))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 22 23) (end 22 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass") (range (start 22 3) (end 22 40)))
        )
      )
    )
    (query (range (start 57 20) (end 57 24)) (probe (position 57 20))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))
        (kind redefinition) (ordinal 0) (authored-target "done")
        (range (start 57 20) (end 57 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 32) (end 27 37)) (probe (position 27 32))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))
        (kind specialization) (ordinal 0) (authored-target "Wheel")
        (range (start 27 32) (end 27 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 32) (end 28 37)) (probe (position 28 32))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))
        (kind specialization) (ordinal 0) (authored-target "Wheel")
        (range (start 28 32) (end 28 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 22)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::kg"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
        (range (start 3 16) (end 3 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 34) (end 7 41)) (probe (position 7 34))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 7 34) (end 7 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 34) (end 16 41)) (probe (position 16 34))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 16 34) (end 16 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 29) (end 38 37)) (probe (position 38 29))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle1")
        (range (start 38 29) (end 38 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 31) (end 67 39)) (probe (position 67 31))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle1")
        (range (start 67 31) (end 67 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 55) (end 67 63)) (probe (position 67 55))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))
        (kind subsetting) (ordinal 1) (authored-target "vehicle1")
        (range (start 67 55) (end 67 63))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 43) (end 67 53)) (probe (position 67 43))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle_C2")
        (range (start 67 43) (end 67 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 33) (end 74 44)) (probe (position 74 33))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_t0")
        (range (start 74 33) (end 74 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 39) (end 25 51)) (probe (position 25 39))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))
        (kind specialization) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 25 39) (end 25 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 29)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleUsages::*")
        (range (start 1 16) (end 1 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 30)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::DateTime"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
        (range (start 2 16) (end 2 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 86 49) (end 86 63)) (probe (position 86 49))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))
        (kind redefinition) (ordinal 0) (authored-target "leftFrontWheel")
        (range (start 86 49) (end 86 63))
        (outcome (status unresolved))
      )
    )
    (query (range (start 103 50) (end 103 65)) (probe (position 103 50))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))
        (kind redefinition) (ordinal 0) (authored-target "rightFrontWheel")
        (range (start 103 50) (end 103 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 80 54) (end 80 71)) (probe (position 80 54))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))
        (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
        (range (start 80 54) (end 80 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 102 54) (end 102 71)) (probe (position 102 54))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))
        (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
        (range (start 102 54) (end 102 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 16) (end 65 35)) (probe (position 65 16))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "IndividualSnapshots::*")
        (range (start 65 16) (end 65 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 96 33) (end 96 52)) (probe (position 96 33))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_t0_t1.done")
        (range (start 96 33) (end 96 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 16) (end 32 37)) (probe (position 32 16))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "IndividualDefinitions::*")
        (range (start 32 16) (end 32 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 21) (end 45 43)) (probe (position 45 21))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))
        (kind redefinition) (ordinal 0) (authored-target "localClock.currentTime")
        (range (start 45 21) (end 45 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 22) (end 58 44)) (probe (position 58 22))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))
        (kind redefinition) (ordinal 0) (authored-target "localClock.currentTime")
        (range (start 58 22) (end 58 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 17) (end 33 47)) (probe (position 33 17))
      (reference
        (source (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensJustBefore")
        (range (start 33 17) (end 33 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
