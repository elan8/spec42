# META
~~~ini
description=SysML Validation (01-Parts Tree): 1a-Parts Tree
type=file
~~~
# SOURCE
~~~sysml
package '1a-Parts Tree' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass {
			doc
			/*
			 * The 'mass' attribute property is declared here to be a 
			 * specialization (subset) of the general 'mass' quantity 
			 * from the 'ISQ' (International System of Quantities) 
			 * library model.
			 */
			}
		}		
		part def AxleAssembly;		
		part def Axle { 
			attribute mass :> ISQ::mass;
		}	
		part def FrontAxle :> Axle { 
			attribute steeringAngle: ScalarValues::Real;
		}	
		part def Wheel;	
	}
	
	package Usages {
		private import Definitions::* {
			/*
			 * A "private" private import makes the imported names private to the
			 * imported package.
			 */
		}
	
		part vehicle1: Vehicle {
			/*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */
			 
			attribute mass redefines Vehicle::mass = 1750 [kg] {
				/*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
			}
			
			part frontAxleAssembly: AxleAssembly {
				/*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */
			
				part frontAxle: Axle;
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * 'frontWheel' is a nested part of type 'Wheel' with
					 * multiplicity "2". This means that this axle assembly
					 * must have exactly two wheels. However, there is still
					 * only one 'frontWheel' part. The part is "ordered",
					 * so that the first wheel can be distinguished from the
					 * second.
					 */
				}
			}
			
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}
			
		}
	
		part vehicle1_c1: Vehicle {
			/*
			 * 'vehicle1_c1' is a modified copy of 'vehicle1'. There is no
			 * connection between this copy and the original version in the
			 * model.
			 */			
			
			attribute mass redefines Vehicle::mass = 2000 [kg] {
				/*
				 * The mass attribute has been modified.
				 */
			}
	
			part frontAxleAssembly: AxleAssembly {
				
				part frontAxle: FrontAxle {
					/*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
				}
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * The parts 'frontWheel_1' and 'frontWheel_2' have been added
					 * as subsets of 'frontWheel'. These are separate parts from
					 * 'frontWheel', but essentially provide alternate names for
					 * each of the two wheels, as given by their defining expressions.
					 */
				}
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
			
			part rearAxleAssembly: AxleAssembly {
				/*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */
						
				part rearAxle: Axle;
				
				part rearWheel: Wheel[2] ordered;
				part rearWheel_1 subsets rearWheel = rearWheel#(1);
				part rearWheel_2 subsets rearWheel = rearWheel#(2);
			}
			
		}
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/1a_parts_tree.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 21) (end 5 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 21) (end 17 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 28) (end 20 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1d065cf8f178a7b346286e07981ccda27e7bc28a0a8c39dbb86d123417ba9106") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The 'mass' attribute property is declared here to be a \n\t\t\t * specialization (subset) of the general 'mass' quantity \n\t\t\t * from the 'ISQ' (International System of Quantities) \n\t\t\t * library model.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (name "Usages") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FrontAxle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::kg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind specialization) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (name "Usages") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (value (kind quantity) (magnitude (value (kind integer) (integer 1750))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (value (kind quantity) (magnitude (value (kind integer) (integer 2000))) (unit "kg")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 17 21) (end 17 30)) (probe (position 17 21))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 19 24) (end 19 28)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle"))) (kind specialization) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 20 28) (end 20 46)) (probe (position 20 28))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 26 17) (end 26 31)) (probe (position 26 17))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (path (name "1a-Parts Tree") (name "Usages") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 33 17) (end 33 24)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 45 27) (end 45 39)) (probe (position 45 27))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 53 20) (end 53 24)) (probe (position 53 20))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 55 21) (end 55 26)) (probe (position 55 21))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 38 28) (end 38 41)) (probe (position 38 28))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 67 26) (end 67 38)) (probe (position 67 26))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 68 19) (end 68 23)) (probe (position 68 19))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 69 20) (end 69 25)) (probe (position 69 20))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 74 20) (end 74 27)) (probe (position 74 20))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 87 27) (end 87 39)) (probe (position 87 27))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 89 20) (end 89 29)) (probe (position 89 20))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::FrontAxle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 95 21) (end 95 26)) (probe (position 95 21))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 103 30) (end 103 40)) (probe (position 103 30))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_1"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 104 30) (end 104 40)) (probe (position 104 30))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel_2"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::frontAxleAssembly::frontWheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 81 28) (end 81 41)) (probe (position 81 28))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 107 26) (end 107 38)) (probe (position 107 26))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 113 19) (end 113 23)) (probe (position 113 19))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 115 20) (end 115 25)) (probe (position 115 20))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 116 29) (end 116 38)) (probe (position 116 29))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_1"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
  )
  (query (document "memory://snapshot/1a_parts_tree.md") (range (start 117 29) (end 117 38)) (probe (position 117 29))
    (reference (id (source (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel_2"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1a_parts_tree.md") (qualified-name "1a-Parts Tree::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
  )
)
~~~
