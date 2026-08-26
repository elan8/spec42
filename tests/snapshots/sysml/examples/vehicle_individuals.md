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
  (document "memory://snapshot/vehicle_individuals.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 32))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 23) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 34) (end 16 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 23) (end 22 27))
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
        (range (start 33 17) (end 33 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 16) (end 35 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 16) (end 36 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 21) (end 45 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 19) (end 48 36))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 43) (end 67 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 33) (end 74 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 54) (end 80 71))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 54) (end 102 71))
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5ebd1bfdcf66e6d42fb3be35cd2157e4d253d627734000a925e36c30f97e69a6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleUsages") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualConfigurations")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "IndividualSnapshots") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n\t\t\t * This asserts that for some portion of its lifetime, Vehicle1 conforms\n\t\t\t * to the configuration vehicle_C2;\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle1")) (subsetting (reference "vehicle_C2")) (subsetting (reference "vehicle1")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (documentation (doc (text "\n    \t\t\t * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.\n    \t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1_t0")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n    \t\t\t\t * frontAxleAssembly is a feature of vehicle1_C2.\n    \t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly1")) (redefinition (reference "frontAxleAssembly")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n    \t\t\t\t\t * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0\n    \t\t\t\t\t * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).\n    \t\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel1")) (redefinition (reference "leftFrontWheel")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (documentation (doc (text "\n    \t\t\t * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.\n    \t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1_t0_t1::done")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly1")) (redefinition (reference "frontAxleAssembly")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n    \t\t\t\t\t * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.\n    \t\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel1")) (redefinition (reference "rightFrontWheel")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AxleAssembly")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n\t\t\t * This is an individual Vehicle with a mass of 1800 kg.\n\t\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n\t\t\t * This is an individual Vehicle with a mass of 1700 kg.\n\t\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "IndividualDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensJustBefore") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle1")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensJustBefore")) (succession (reference "vehicle1_t0")) (succession (reference "vehicle1_t0_t1")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (documentation (doc (text "\n    \t\t\t * This is a snapshot of Vehicle1 at time t0;\n    \t\t\t "))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock::currentTime")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "t0")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)) (documentation (doc (text "\n    \t\t\t * This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 \n    \t\t\t * (time t0) and ending at time t1.\n    \t\t\t "))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "done")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock::currentTime")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "t1")))))
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleUsages")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::kg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualConfigurations")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "IndividualSnapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_C2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 1))
      (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_t0")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind redefinition) (ordinal 0))
      (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0))
      (authored-target "leftFrontWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_t0_t1::done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind redefinition) (ordinal 0))
      (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0))
      (authored-target "rightFrontWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind specialization) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "IndividualDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "vehicle1_t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "vehicle1_t0_t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1800))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1700))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (unit (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 13 36) (end 13 38) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 22 36) (end 22 38) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (source direct))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (source inherited) (from (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (scopes any))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0")))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1")))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))
      (type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1")))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 1 16) (end 1 32)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleUsages")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 65 16) (end 65 38)) (probe (position 65 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualConfigurations")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "IndividualSnapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 67 31) (end 67 39)) (probe (position 67 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 67 43) (end 67 53)) (probe (position 67 43))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_C2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 67 55) (end 67 63)) (probe (position 67 55))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2"))) (kind subsetting) (ordinal 1) (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 74 33) (end 74 44)) (probe (position 74 33))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_t0")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 80 36) (end 80 49)) (probe (position 80 36))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 80 54) (end 80 71)) (probe (position 80 54))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0"))) (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 86 38) (end 86 44)) (probe (position 86 38))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 86 49) (end 86 63)) (probe (position 86 49))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t0::axleAssembly1_t0::leftFrontWheel_t0"))) (kind redefinition) (ordinal 0) (authored-target "leftFrontWheel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 96 33) (end 96 52)) (probe (position 96 33))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_t0_t1::done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 102 36) (end 102 49)) (probe (position 102 36))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 102 54) (end 102 71)) (probe (position 102 54))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1"))) (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 103 39) (end 103 45)) (probe (position 103 39))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 103 50) (end 103 65)) (probe (position 103 50))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualConfigurations::vehicle1_C2::vehicle1_C2_t1::axleAssembly1_t1::rightFrontWheel_t1"))) (kind redefinition) (ordinal 0) (authored-target "rightFrontWheel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 25 39) (end 25 51)) (probe (position 25 39))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::AxleAssembly1"))) (kind specialization) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 7 34) (end 7 41)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 13 23) (end 13 27)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 16 34) (end 16 41)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle2"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 22 23) (end 22 27)) (probe (position 22 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualDefinitions")) (named (kind part-def) (name "Vehicle2")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 27 32) (end 27 37)) (probe (position 27 32))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel1"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 28 32) (end 28 37)) (probe (position 28 32))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Wheel2"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 32 16) (end 32 40)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "IndividualDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 33 17) (end 33 47)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensJustBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 35 16) (end 35 24)) (probe (position 35 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 36 16) (end 36 24)) (probe (position 36 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 38 29) (end 38 37)) (probe (position 38 29))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualDefinitions::Vehicle1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 48 19) (end 48 36)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 48 43) (end 48 54)) (probe (position 48 43))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "vehicle1_t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 48 60) (end 48 74)) (probe (position 48 60))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "vehicle1_t0_t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::vehicle1::vehicle1_t0_t1")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 45 21) (end 45 43)) (probe (position 45 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock::currentTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 45 46) (end 45 48)) (probe (position 45 46))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t0")))))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 57 20) (end 57 24)) (probe (position 57 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 58 22) (end 58 44)) (probe (position 58 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock::currentTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_individuals.md") (range (start 58 47) (end 58 49)) (probe (position 58 47))
    (reference (id (source (node (document "memory://snapshot/vehicle_individuals.md") (path (named (kind package) (name "VehicleIndividuals")) (named (kind package) (name "IndividualSnapshots")) (named (kind part) (name "vehicle1")) (named (kind occurrence) (name "vehicle1_t0_t1")) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_individuals.md") (qualified-name "VehicleIndividuals::IndividualSnapshots::t1")))))
    )
  )
)
~~~
