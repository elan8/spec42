# META
~~~ini
description=SysML Validation (01-Parts Tree): 1c-Parts Tree Redefinition
type=file
~~~
# SOURCE
~~~sysml
package '1c-Parts Tree Redefinition' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass;
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
		private import Definitions::*;
		
		part vehicle1: Vehicle {
			attribute mass redefines Vehicle::mass default = 1750 [kg] {
			doc
			/*
			 * The mass attribute is redefined to give it a default value.
			 */
			}
					
			part frontAxleAssembly: AxleAssembly {
				part frontAxle: Axle;			
				part frontWheel: Wheel[2] ordered;
			}		
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}		
		}
	
		part vehicle1_c1 :> vehicle1 {
			/*
			 * 'vehicle1_c1' is a specialization of 'vehicle1' (technically 
			 * a subset). It inherits all the parts of 'vehicle1' and
			 * only needs to specify additional or redefined parts.
			 */
		
			attribute mass redefines vehicle1::mass = 2000 [kg] {
				/*
				 * The mass is further redefined to override the default value
				 * with a bound value for 'vehicle_c1'.
				 */
			}
					
			part frontAxleAssembly_c1 redefines frontAxleAssembly {
				part frontAxle_c1: FrontAxle redefines frontAxle {
					/*
					 * 'frontAxle_c1' redefines 'frontAxleAssembly'::'frontAxle'
					 * to give it a new name and the specialized type
					 * 'FrontAxle'.
					 */
				}
				
				/*
				 * 'frontWheel' is inherited from 'vehicle1'::'frontAxleAssembly',
				 * allowing it to be used in the following part declarations.
				 */
				
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
				
			part rearAxleAssembly_c1 redefines rearAxleAssembly {
				part rearAxle_c1 redefines rearAxle {
					/*
					 * 'rearAxle_c1' redefines 'rearAxleAssembly'::'rearAxle'
					 * to give it a new name. It inherits the type 'Axle'
					 * from the redefined part.
					 */
				}
						
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
  (document "memory://snapshot/1c_parts_tree_redefinition.md"
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
        (range (start 9 21) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 28) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 39) (end 52 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 43) (end 53 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 30) (end 66 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 30) (end 67 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 38) (end 70 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 31) (end 71 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 29) (end 79 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 29) (end 80 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:11973dec81e9056ba9f34ecf61a7b72f25f4c8d866d054e6af20910a1ca20dc3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The mass attribute is redefined to give it a default value.\n\t\t\t "))) (feature-value (kind bind) (default true)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "frontAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FrontAxle")) (redefinition (reference "frontAxle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle1::mass"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxle"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::kg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind redefinition) (ordinal 0))
      (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind redefinition) (ordinal 0))
      (authored-target "frontAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle1::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass")))))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (value (kind quantity) (magnitude (value (kind integer) (integer 1750))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (value (kind quantity) (magnitude (value (kind integer) (integer 2000))) (unit "kg")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 9 21) (end 9 30)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 11 24) (end 11 28)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle"))) (kind specialization) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 12 28) (end 12 46)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle::steeringAngle"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 18 17) (end 18 31)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 20 17) (end 20 24)) (probe (position 20 17))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 28 27) (end 28 39)) (probe (position 28 27))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 29 20) (end 29 24)) (probe (position 29 20))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 30 21) (end 30 26)) (probe (position 30 21))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 21 28) (end 21 41)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 32 26) (end 32 38)) (probe (position 32 26))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 33 19) (end 33 23)) (probe (position 33 19))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 34 20) (end 34 25)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 38 22) (end 38 30)) (probe (position 38 22))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 52 39) (end 52 56)) (probe (position 52 39))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1"))) (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 53 23) (end 53 32)) (probe (position 53 23))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "FrontAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Definitions::FrontAxle")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 53 43) (end 53 52)) (probe (position 53 43))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontAxle_c1"))) (kind redefinition) (ordinal 0) (authored-target "frontAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 66 30) (end 66 40)) (probe (position 66 30))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_1"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 67 30) (end 67 40)) (probe (position 67 30))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::frontAxleAssembly_c1::frontWheel_2"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 45 28) (end 45 42)) (probe (position 45 28))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0) (authored-target "vehicle1::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1::mass")))))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 70 38) (end 70 54)) (probe (position 70 38))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1"))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 71 31) (end 71 39)) (probe (position 71 31))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearAxle_c1"))) (kind redefinition) (ordinal 0) (authored-target "rearAxle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 79 29) (end 79 38)) (probe (position 79 29))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_1"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/1c_parts_tree_redefinition.md") (range (start 80 29) (end 80 38)) (probe (position 80 29))
    (reference (id (source (node (document "memory://snapshot/1c_parts_tree_redefinition.md") (qualified-name "1c-Parts Tree Redefinition::Usages::vehicle1_c1::rearAxleAssembly_c1::rearWheel_2"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unresolved)))
  )
)
~~~
