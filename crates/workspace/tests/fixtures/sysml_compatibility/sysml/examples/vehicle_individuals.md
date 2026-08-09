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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleIndividuals"))) (name "VehicleIndividuals") (declared-name "VehicleIndividuals")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::DateTime"))) (name "DateTime") (declared-name "DateTime"))
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (name "IndividualConfigurations") (declared-name "IndividualConfigurations")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (name "vehicle1_C2") (declared-name "vehicle1_C2") (declared (properties (individual true) (ordered false)))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (name "vehicle1_C2_t0") (declared-name "vehicle1_C2_t0") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (name "axleAssembly1_t0") (declared-name "axleAssembly1_t0") (declared (properties (individual true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                      (contains
                        (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (name "leftFrontWheel_t0") (declared-name "leftFrontWheel_t0") (declared (properties (individual true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))))
                          (contains
                            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (name "vehicle1_C2_t1") (declared-name "vehicle1_C2_t1") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (name "axleAssembly1_t1") (declared-name "axleAssembly1_t1") (declared (properties (individual true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                      (contains
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (name "rightFrontWheel_t1") (declared-name "rightFrontWheel_t1") (declared (properties (individual true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))))
                          (contains
                            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
                          )
                        )
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (name "IndividualDefinitions") (declared-name "IndividualDefinitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (name "AxleAssembly1") (declared-name "AxleAssembly1") (declared (properties (individual true))))
            (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (name "Vehicle1") (declared-name "Vehicle1") (declared (properties (individual true)))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 1800))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (name "Vehicle2") (declared-name "Vehicle2") (declared (properties (individual true)))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 1700))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (name "Wheel1") (declared-name "Wheel1") (declared (properties (individual true))))
            (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (name "Wheel2") (declared-name "Wheel2") (declared (properties (individual true))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (name "IndividualSnapshots") (declared-name "IndividualSnapshots")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::HappensJustBefore"))) (name "HappensJustBefore") (declared-name "HappensJustBefore"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (name "t0") (declared-name "t0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (name "t1") (declared-name "t1") (declared (properties (ordered false) (unique true))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (individual true) (ordered false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (name "vehicle1_t0") (declared-name "vehicle1_t0") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (name "currentTime") (declared-name "currentTime") (declared (feature-value (kind bound) (expression (kind "featureReference") (reference "t0")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (role feature-value))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (name "vehicle1_t0_t1") (declared-name "vehicle1_t0_t1") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (name "currentTime") (declared-name "currentTime") (declared (feature-value (kind bound) (expression (kind "featureReference") (reference "t1")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (role feature-value))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                      )
                    )
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleIndividuals::kg"))) (name "kg") (declared-name "kg"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::_documentation"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (to (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0::currentTime"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1::::currentTime"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_individuals.md"
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
        (range (start 7 2) (end 7 172))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 13 3) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 13 3) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 2) (end 16 171))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 22 3) (end 22 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 22 3) (end 22 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 25 2) (end 25 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 27 2) (end 27 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 2) (end 28 38))
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
        (range (start 35 2) (end 35 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 45 7) (end 45 49))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 48 6) (end 48 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 57 16) (end 57 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 58 8) (end 58 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 65 16) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 80 18) (end 80 449))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 86 19) (end 86 265))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 102 18) (end 102 276))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 103 19) (end 103 193))
      )
    )
  )
)
~~~
