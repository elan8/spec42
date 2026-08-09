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
# FORMAT
~~~sysml
package VehicleIndividuals {
    private import VehicleUsages::*;
    private import Time::DateTime;
    private import SI::kg;

    package IndividualDefinitions {
        individual part def Vehicle1 :> Vehicle {
            doc /*
			 * This is an individual Vehicle with a mass of 1800 kg.
			 */

            attribute redefines mass = 1800 [kg];
        }

        individual part def Vehicle2 :> Vehicle {
            doc /*
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

        attribute t0 : DateTime;
        attribute t1 : DateTime;

        individual part vehicle1 : Vehicle1 {
            snapshot vehicle1_t0 {
                doc /*
    			 * This is a snapshot of Vehicle1 at time t0;
    			 */

                attribute :>> localClock.currentTime = t0;
            }

            succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;

            timeslice vehicle1_t0_t1 {
                doc /*
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

        individual part vehicle1_C2 : Vehicle1 :> vehicle_C2, vehicle1 {
            doc /*
			 * This asserts that for some portion of its lifetime, Vehicle1 conforms
			 * to the configuration vehicle_C2;
			 */

            snapshot vehicle1_C2_t0 :> vehicle1_t0 {
                doc /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.
    			 */

                individual axleAssembly1_t0 : AxleAssembly1 :>> frontAxleAssembly {
                    doc /*
    				 * frontAxleAssembly is a feature of vehicle1_C2.
    				 */

                    individual leftFrontWheel_t0 : Wheel1 :>> leftFrontWheel {
                        doc /*
    					 * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0
    					 * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).
    					 */
                    }
                }
            }

            snapshot vehicle1_C2_t1 :> vehicle1_t0_t1.done {
                doc /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.
    			 */

                individual axleAssembly1_t1 : AxleAssembly1 :>> frontAxleAssembly {
                    individual rightFrontWheel_t1 : Wheel1 :>> rightFrontWheel {
                        doc /*
    					 * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.
    					 */
                    }
                }
            }
        }
    }
}
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
# SMG
~~~
(model
  (namespace
    (package 'VehicleIndividuals'
      (namespace_import private -> 'VehicleUsages'[unresolved])
      (membership_import private -> 'Time::DateTime'[unresolved])
      (membership_import private -> 'SI::kg'[unresolved])
      (package 'IndividualDefinitions'
        (part_def individual 'Vehicle1' :> 'Vehicle'[unresolved]
          (documentation)
          (attribute_usage composite :>> 'mass'[unresolved]
            (feature_value (=))))
        (part_def individual 'Vehicle2' :> 'Vehicle'[unresolved]
          (documentation)
          (attribute_usage composite :>> 'mass'[unresolved]
            (feature_value (=))))
        (part_def individual 'AxleAssembly1' :> 'AxleAssembly'[unresolved])
        (part_def individual 'Wheel1' :> 'Wheel'[unresolved])
        (part_def individual 'Wheel2' :> 'Wheel'[unresolved]))
      (package 'IndividualSnapshots'
        (namespace_import public -> 'VehicleIndividuals::IndividualDefinitions'[package])
        (membership_import private -> 'Occurrences::HappensJustBefore'[unresolved])
        (attribute_usage 't0' : 'DateTime'[unresolved])
        (attribute_usage 't1' : 'DateTime'[unresolved])
        (part_usage individual 'vehicle1' : 'VehicleIndividuals::IndividualDefinitions::Vehicle1'[part_def]
          (occurrence_usage composite 'vehicle1_t0'
            (documentation)
            (attribute_usage composite :>> 'localClock::currentTime'[unresolved]
              (feature_value (=))))
          (succession_def : 'HappensJustBefore'[unresolved]
            (connector_end 'vehicle1_t0')
            (connector_end 'vehicle1_t0_t1'))
          (occurrence_usage composite 'vehicle1_t0_t1'
            (documentation)
            (occurrence_usage composite :>> 'done'[unresolved]
              (attribute_usage composite :>> 'localClock::currentTime'[unresolved]
                (feature_value (=)))))))
      (package 'IndividualConfigurations'
        (namespace_import public -> 'VehicleIndividuals::IndividualSnapshots'[package])
        (part_usage individual 'vehicle1_C2' : 'VehicleIndividuals::IndividualDefinitions::Vehicle1'[part_def] :> 'vehicle_C2'[unresolved] :> 'VehicleIndividuals::IndividualSnapshots::vehicle1'[part_usage]
          (documentation)
          (occurrence_usage composite 'vehicle1_C2_t0' :> 'VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0'[occurrence_usage]
            (documentation)
            (occurrence_usage individual composite 'axleAssembly1_t0' : 'VehicleIndividuals::IndividualDefinitions::AxleAssembly1'[part_def] :>> 'frontAxleAssembly'[unresolved]
              (documentation)
              (occurrence_usage individual composite 'leftFrontWheel_t0' : 'VehicleIndividuals::IndividualDefinitions::Wheel1'[part_def] :>> 'leftFrontWheel'[unresolved]
                (documentation))))
          (occurrence_usage composite 'vehicle1_C2_t1' :> 'vehicle1_t0_t1::done'[unresolved]
            (documentation)
            (occurrence_usage individual composite 'axleAssembly1_t1' : 'VehicleIndividuals::IndividualDefinitions::AxleAssembly1'[part_def] :>> 'frontAxleAssembly'[unresolved]
              (occurrence_usage individual composite 'rightFrontWheel_t1' : 'VehicleIndividuals::IndividualDefinitions::Wheel1'[part_def] :>> 'rightFrontWheel'[unresolved]
                (documentation)))))))))
~~~
