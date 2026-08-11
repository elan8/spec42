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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwIndividual,KwPart,Ident,Colon,Ident,OpenCurly,
KwSnapshot,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwSuccession,Colon,Ident,KwFirst,Ident,KwThen,Ident,Semicolon,
KwTimeslice,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwIndividual,KwPart,Ident,Colon,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSnapshot,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIndividual,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIndividual,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,ColonGt,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIndividual,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwIndividual,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleIndividuals'
    (import_decl private 'VehicleUsages::*')
    (import_decl private 'Time::DateTime')
    (import_decl private 'SI::kg')
    (package_def 'IndividualDefinitions'
      (part_def individual 'Vehicle1' :> 'Vehicle'
        (documentation)
        (attribute_usage :>> 'mass' value))
      (part_def individual 'Vehicle2' :> 'Vehicle'
        (documentation)
        (attribute_usage :>> 'mass' value))
      (part_def individual 'AxleAssembly1' :> 'AxleAssembly')
      (part_def individual 'Wheel1' :> 'Wheel')
      (part_def individual 'Wheel2' :> 'Wheel'))
    (package_def 'IndividualSnapshots'
      (import_decl public 'IndividualDefinitions::*')
      (import_decl private 'Occurrences::HappensJustBefore')
      (attribute_usage 't0' : 'DateTime')
      (attribute_usage 't1' : 'DateTime')
      (part_usage individual 'vehicle1' : 'Vehicle1'
        (portion_usage snapshot 'vehicle1_t0'
          (documentation)
          (attribute_usage :>> 'localClock.currentTime' value))
        (succession_as_usage 'HappensJustBefore'
          (connector_end)
          (connector_end))
        (portion_usage timeslice 'vehicle1_t0_t1'
          (documentation)
          (portion_usage snapshot :>> 'done'
            (attribute_usage :>> 'localClock.currentTime' value)))))
    (package_def 'IndividualConfigurations'
      (import_decl public 'IndividualSnapshots::*')
      (part_usage individual 'vehicle1_C2' : 'Vehicle1' :> 'vehicle_C2', 'vehicle1'
        (documentation)
        (portion_usage snapshot 'vehicle1_C2_t0' :> 'vehicle1_t0'
          (documentation)
          (individual_usage individual 'axleAssembly1_t0' : 'AxleAssembly1' :>> 'frontAxleAssembly'
            (documentation)
            (individual_usage individual 'leftFrontWheel_t0' : 'Wheel1' :>> 'leftFrontWheel'
              (documentation))))
        (portion_usage snapshot 'vehicle1_C2_t1' :> 'vehicle1_t0_t1.done'
          (documentation)
          (individual_usage individual 'axleAssembly1_t1' : 'AxleAssembly1' :>> 'frontAxleAssembly'
            (individual_usage individual 'rightFrontWheel_t1' : 'Wheel1' :>> 'rightFrontWheel'
              (documentation))))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'done'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'vehicle_C2'
semantic.unresolved_name 'frontAxleAssembly'
semantic.unresolved_name 'leftFrontWheel'
semantic.unresolved_name 'vehicle1_t0_t1::done'
semantic.unresolved_name 'frontAxleAssembly'
semantic.unresolved_name 'rightFrontWheel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'done'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'vehicle_C2'
semantic.unresolved_name 'frontAxleAssembly'
semantic.unresolved_name 'leftFrontWheel'
semantic.unresolved_name 'vehicle1_t0_t1::done'
semantic.unresolved_name 'frontAxleAssembly'
semantic.unresolved_name 'rightFrontWheel'
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
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals"))) (kind "package") (name "VehicleIndividuals") (declared-name "VehicleIndividuals") (range (start (line 0) (character 0)) (end (line 0) (character 2826))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 33))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleUsages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 29))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 30))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (kind "package") (name "IndividualConfigurations") (declared-name "IndividualConfigurations") (range (start (line 64) (character 1)) (end (line 64) (character 1394))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 65) (character 2)) (end (line 65) (character 39))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (authored (membership (kind Import) (visibility "public") (import (reference "IndividualSnapshots::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 65) (character 16)) (end (line 65) (character 35))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind "part") (name "vehicle1_C2") (declared-name "vehicle1_C2") (range (start (line 67) (character 2)) (end (line 67) (character 1313))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle1") (range (start (line 67) (character 31)) (end (line 67) (character 39)))) (subsetting (reference "vehicle_C2") (range (start (line 67) (character 43)) (end (line 67) (character 53)))) (subsetting (reference "vehicle1") (range (start (line 67) (character 55)) (end (line 67) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::_documentation"))) (kind "documentation") (name "") (range (start (line 67) (character 2)) (end (line 67) (character 1313))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind "occurrence") (name "vehicle1_C2_t0") (declared-name "vehicle1_C2_t0") (range (start (line 74) (character 15)) (end (line 74) (character 625))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_t0") (range (start (line 74) (character 33)) (end (line 74) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::_documentation"))) (kind "documentation") (name "") (range (start (line 74) (character 15)) (end (line 74) (character 625))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind "occurrence") (name "axleAssembly1_t0") (declared-name "axleAssembly1_t0") (range (start (line 80) (character 18)) (end (line 80) (character 449))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly1") (range none)) (redefinition (reference "frontAxleAssembly") (range (start (line 80) (character 54)) (end (line 80) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::_documentation"))) (kind "documentation") (name "") (range (start (line 80) (character 18)) (end (line 80) (character 449))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind "occurrence") (name "leftFrontWheel_t0") (declared-name "leftFrontWheel_t0") (range (start (line 86) (character 19)) (end (line 86) (character 265))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel1") (range none)) (redefinition (reference "leftFrontWheel") (range (start (line 86) (character 49)) (end (line 86) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0::_documentation"))) (kind "documentation") (name "") (range (start (line 86) (character 19)) (end (line 86) (character 265))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind "occurrence") (name "vehicle1_C2_t1") (declared-name "vehicle1_C2_t1") (range (start (line 96) (character 15)) (end (line 96) (character 459))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_t0_t1.done") (range (start (line 96) (character 33)) (end (line 96) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::_documentation"))) (kind "documentation") (name "") (range (start (line 96) (character 15)) (end (line 96) (character 459))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind "occurrence") (name "axleAssembly1_t1") (declared-name "axleAssembly1_t1") (range (start (line 102) (character 18)) (end (line 102) (character 276))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly1") (range none)) (redefinition (reference "frontAxleAssembly") (range (start (line 102) (character 54)) (end (line 102) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind "occurrence") (name "rightFrontWheel_t1") (declared-name "rightFrontWheel_t1") (range (start (line 103) (character 19)) (end (line 103) (character 193))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel1") (range none)) (redefinition (reference "rightFrontWheel") (range (start (line 103) (character 50)) (end (line 103) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1::_documentation"))) (kind "documentation") (name "") (range (start (line 103) (character 19)) (end (line 103) (character 193))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (kind "package") (name "IndividualDefinitions") (declared-name "IndividualDefinitions") (range (start (line 5) (character 1)) (end (line 5) (character 521))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind "part def") (name "AxleAssembly1") (declared-name "AxleAssembly1") (range (start (line 25) (character 2)) (end (line 25) (character 52))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AxleAssembly") (range (start (line 25) (character 39)) (end (line 25) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind "part def") (name "Vehicle1") (declared-name "Vehicle1") (range (start (line 7) (character 2)) (end (line 7) (character 172))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 7) (character 34)) (end (line 7) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::_documentation"))) (kind "documentation") (name "") (range (start (line 7) (character 2)) (end (line 7) (character 172))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 13) (character 3)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 13) (character 23)) (end (line 13) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind "part def") (name "Vehicle2") (declared-name "Vehicle2") (range (start (line 16) (character 2)) (end (line 16) (character 171))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 16) (character 34)) (end (line 16) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::_documentation"))) (kind "documentation") (name "") (range (start (line 16) (character 2)) (end (line 16) (character 171))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 22) (character 3)) (end (line 22) (character 40))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 22) (character 23)) (end (line 22) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind "part def") (name "Wheel1") (declared-name "Wheel1") (range (start (line 27) (character 2)) (end (line 27) (character 38))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel") (range (start (line 27) (character 32)) (end (line 27) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind "part def") (name "Wheel2") (declared-name "Wheel2") (range (start (line 28) (character 2)) (end (line 28) (character 38))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel") (range (start (line 28) (character 32)) (end (line 28) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (kind "package") (name "IndividualSnapshots") (declared-name "IndividualSnapshots") (range (start (line 31) (character 1)) (end (line 31) (character 782))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 32) (character 2)) (end (line 32) (character 41))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Import) (visibility "public") (import (reference "IndividualDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 32) (character 16)) (end (line 32) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))) (kind "import") (name "HappensJustBefore") (declared-name "HappensJustBefore") (range (start (line 33) (character 2)) (end (line 33) (character 48))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensJustBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 33) (character 17)) (end (line 33) (character 47))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind "attribute def") (name "t0") (declared-name "t0") (range (start (line 35) (character 2)) (end (line 35) (character 25))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Owning)) (relationships (typing (reference "DateTime") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind "attribute def") (name "t1") (declared-name "t1") (range (start (line 36) (character 2)) (end (line 36) (character 25))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Owning)) (relationships (typing (reference "DateTime") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 38) (character 2)) (end (line 38) (character 599))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle1") (range (start (line 38) (character 29)) (end (line 38) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (kind "occurrence") (name "vehicle1_t0") (declared-name "vehicle1_t0") (range (start (line 39) (character 15)) (end (line 39) (character 178))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::_documentation"))) (kind "documentation") (name "") (range (start (line 39) (character 15)) (end (line 39) (character 178))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (range (start (line 45) (character 7)) (end (line 45) (character 49))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime") (range (start (line 45) (character 21)) (end (line 45) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (kind "occurrence") (name "vehicle1_t0_t1") (declared-name "vehicle1_t0_t1") (range (start (line 50) (character 16)) (end (line 50) (character 286))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (kind "occurrence") (name "") (range (start (line 57) (character 16)) (end (line 57) (character 86))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done") (range (start (line 57) (character 20)) (end (line 57) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (range (start (line 58) (character 8)) (end (line 58) (character 50))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime") (range (start (line 58) (character 22)) (end (line 58) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 16)) (end (line 50) (character 286))) (parent (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))))
    (element (id (node (document "d0") (qualified-name "VehicleIndividuals::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "VehicleIndividuals"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleUsages::*") (range (start (line 1) (character 16)) (end (line 1) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (range (start (line 2) (character 16)) (end (line 2) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "IndividualSnapshots::*") (range (start (line 65) (character 16)) (end (line 65) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle1") (range (start (line 67) (character 31)) (end (line 67) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C2") (range (start (line 67) (character 43)) (end (line 67) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 1)) (authored-target "vehicle1") (range (start (line 67) (character 55)) (end (line 67) (character 63))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_t0") (range (start (line 74) (character 33)) (end (line 74) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (range (start (line 80) (character 54)) (end (line 80) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0)) (authored-target "leftFrontWheel") (range (start (line 86) (character 49)) (end (line 86) (character 63))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_t0_t1.done") (range (start (line 96) (character 33)) (end (line 96) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (range (start (line 102) (character 54)) (end (line 102) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0)) (authored-target "rightFrontWheel") (range (start (line 103) (character 50)) (end (line 103) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind specialization) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 25) (character 39)) (end (line 25) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 7) (character 34)) (end (line 7) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 13) (character 23)) (end (line 13) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 16) (character 34)) (end (line 16) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 22) (character 23)) (end (line 22) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (range (start (line 27) (character 32)) (end (line 27) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (range (start (line 28) (character 32)) (end (line 28) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "IndividualDefinitions::*") (range (start (line 32) (character 16)) (end (line 32) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensJustBefore") (range (start (line 33) (character 17)) (end (line 33) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleIndividuals::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle1") (range (start (line 38) (character 29)) (end (line 38) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (range (start (line 45) (character 21)) (end (line 45) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (kind redefinition) (ordinal 0)) (authored-target "done") (range (start (line 57) (character 20)) (end (line 57) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (range (start (line 58) (character 22)) (end (line 58) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleIndividuals::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 3) (character 16)) (end (line 3) (character 22))) (outcome (status unresolved)))
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
