# META
~~~ini
description=SysML Validation (13-Model Containment): 13a-Model Containment
type=file
~~~
# SOURCE
~~~sysml
package '13a-Model Containment' {
	private import '2a-Parts Interconnection'::*;
	private import '8-Requirements'::*;
	
	requirement BodyAndInteriorRequirements {
		public import MassLimitationRequirement; 
	}
	
	requirement PowerTrainRequirements;
	
	package 'Vehicle Model' {
		doc
		/*
		 * This package is used to represent a top-level "model".
		 * There is no specific syntax for identifying a package
		 * used in this way.
		 */
	
		
		package 'Vehicle1-Configuration' {			
			alias 'Sport Sedan' for vehicle1_c1;
			
			public import 'vehicle1_c1 Specification Context'::'vehicle1-c1 Specification';		
		}
		
		package 'Vehicle Reference Model' {
			doc
			/*
			 * This package is used to represent a "model library".
			 * There is no specific syntax for identifying a package
			 * used in this way.
			 */
		
			public import VehicleA;			
			public import VehicleSubsystems;
			
			//*
			// The following would transitively import all the
			// members of the VehicleSubsystems package, rather
			// then importing the package itself.
			 
			   public import VehicleSubsystems::*;
			*/
		}
		
		package VehicleSubsystems {
			public import 'Body&Interior';
			public import 'PowerTrain';
		}
		
		package 'Body&Interior' {
			public import BodyAndInteriorRequirements;			
		}
		
		package PowerTrain {
			public import Engine;
			public import Transmission;
			public import PowerTrainRequirements;			
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13a_model_containment.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 27) (end 20 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 17) (end 22 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 17) (end 33 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 55 17) (end 55 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 56 17) (end 56 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:51b9f2b846f0a5f90279a22aef7c274f6a3e850233ecc25eeb44e4f918280f1d") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "2a-Parts Interconnection") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "8-Requirements") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind requirement) (name "BodyAndInteriorRequirements")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "MassLimitationRequirement") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::PowerTrainRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * This package is used to represent a top-level \"model\".\n\t\t * There is no specific syntax for identifying a package\n\t\t * used in this way.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Body&Interior")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "BodyAndInteriorRequirements") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Engine") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Transmission") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "PowerTrainRequirements") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t\t * This package is used to represent a \"model library\".\n\t\t\t * There is no specific syntax for identifying a package\n\t\t\t * used in this way.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleA") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "VehicleSubsystems") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle1-Configuration")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1 Specification Context::vehicle1-c1 Specification") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "vehicle1_c1")))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Body&Interior") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "PowerTrain") (import (shape membership) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "8-Requirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind requirement) (name "BodyAndInteriorRequirements")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "MassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Body&Interior")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "BodyAndInteriorRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements")))))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "PowerTrainRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::PowerTrainRequirements")))))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleSubsystems")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems")))))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle1-Configuration")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1 Specification Context::vehicle1-c1 Specification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (kind aliasBinding) (ordinal 0))
      (authored-target "vehicle1_c1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Body&Interior")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior")))))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "2a-Parts Interconnection")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "8-Requirements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 5 16) (end 5 41)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind requirement) (name "BodyAndInteriorRequirements")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "MassLimitationRequirement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 51 17) (end 51 44)) (probe (position 51 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Body&Interior")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "BodyAndInteriorRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements")))))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 55 17) (end 55 23)) (probe (position 55 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 56 17) (end 56 29)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Transmission")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 57 17) (end 57 39)) (probe (position 57 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "PowerTrain")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "PowerTrainRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::PowerTrainRequirements")))))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 33 17) (end 33 25)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleA")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 34 17) (end 34 34)) (probe (position 34 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle Reference Model")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleSubsystems")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems")))))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 22 17) (end 22 81)) (probe (position 22 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "Vehicle1-Configuration")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1 Specification Context::vehicle1-c1 Specification")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 20 27) (end 20 38)) (probe (position 20 27))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (kind aliasBinding) (ordinal 0) (authored-target "vehicle1_c1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 46 17) (end 46 32)) (probe (position 46 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Body&Interior")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior")))))
    )
  )
  (query (document "memory://snapshot/13a_model_containment.md") (range (start 47 17) (end 47 29)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/13a_model_containment.md") (path (named (kind package) (name "13a-Model Containment")) (named (kind package) (name "Vehicle Model")) (named (kind package) (name "VehicleSubsystems")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/13a_model_containment.md") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain")))))
    )
  )
)
~~~
