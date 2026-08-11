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
  (document "13a_model_containment.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 41))
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
        (range (start 34 17) (end 34 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 17) (end 46 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 17) (end 47 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 51 17) (end 51 44))
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
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 57 17) (end 57 39))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "60a5fc5ca0614584ba2069c160369b70872ab66ff08fed7bbbf45881b522ed6f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13a-Model Containment"))) (kind "package") (name "13a-Model Containment") (declared-name "13a-Model Containment"))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13a-Model Containment"))) (authored (membership (kind Import) (visibility "private") (import (reference "2a-Parts Interconnection::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13a-Model Containment"))) (authored (membership (kind Import) (visibility "private") (import (reference "8-Requirements::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (kind "requirement") (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements") (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))) (kind "import") (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement") (parent (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "MassLimitationRequirement") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::PowerTrainRequirements"))) (kind "requirement") (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements") (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))) (kind "package") (name "Vehicle Model") (declared-name "Vehicle Model") (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (kind "package") (name "Body&Interior") (declared-name "Body&Interior") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))) (kind "import") (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (authored (membership (kind Import) (visibility "public") (import (reference "BodyAndInteriorRequirements") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (kind "package") (name "PowerTrain") (declared-name "PowerTrain") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))) (kind "import") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "Engine") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))) (kind "import") (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "PowerTrainRequirements") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))) (kind "import") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "Transmission") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (kind "package") (name "Vehicle Reference Model") (declared-name "Vehicle Reference Model") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))) (kind "import") (name "VehicleA") (declared-name "VehicleA") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleA") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))) (kind "import") (name "VehicleSubsystems") (declared-name "VehicleSubsystems") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleSubsystems") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (kind "package") (name "Vehicle1-Configuration") (declared-name "Vehicle1-Configuration") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (kind "alias") (name "Sport Sedan") (declared-name "Sport Sedan") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))) (kind "import") (name "vehicle1-c1 Specification") (declared-name "vehicle1-c1 Specification") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1 Specification Context::vehicle1-c1 Specification") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (kind "package") (name "VehicleSubsystems") (declared-name "VehicleSubsystems") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))) (kind "import") (name "Body&Interior") (declared-name "Body&Interior") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (authored (membership (kind Import) (visibility "public") (import (reference "Body&Interior") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))) (kind "import") (name "PowerTrain") (declared-name "PowerTrain") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (authored (membership (kind Import) (visibility "public") (import (reference "PowerTrain") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "2a-Parts Interconnection::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "8-Requirements::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))) (kind membershipImport) (ordinal 0)) (authored-target "MassLimitationRequirement") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))) (kind membershipImport) (ordinal 0)) (authored-target "BodyAndInteriorRequirements") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))) (kind membershipImport) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))) (kind membershipImport) (ordinal 0)) (authored-target "PowerTrainRequirements") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))) (kind membershipImport) (ordinal 0)) (authored-target "Transmission") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleA") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleSubsystems") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1 Specification Context::vehicle1-c1 Specification") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))) (kind membershipImport) (ordinal 0)) (authored-target "Body&Interior") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))) (kind membershipImport) (ordinal 0)) (authored-target "PowerTrain") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 55 17) (end 55 23)) (probe (position 55 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))
        (kind membershipImport) (ordinal 0) (authored-target "Engine")
        (range (start 55 17) (end 55 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 17) (end 33 25)) (probe (position 33 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))
        (kind membershipImport) (ordinal 0) (authored-target "VehicleA")
        (range (start 33 17) (end 33 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 17) (end 47 29)) (probe (position 47 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))
        (kind membershipImport) (ordinal 0) (authored-target "PowerTrain")
        (range (start 47 17) (end 47 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 17) (end 56 29)) (probe (position 56 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))
        (kind membershipImport) (ordinal 0) (authored-target "Transmission")
        (range (start 56 17) (end 56 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 17) (end 46 32)) (probe (position 46 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))
        (kind membershipImport) (ordinal 0) (authored-target "Body&Interior")
        (range (start 46 17) (end 46 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 32)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "8-Requirements::*")
        (range (start 2 16) (end 2 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 17) (end 34 34)) (probe (position 34 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))
        (kind membershipImport) (ordinal 0) (authored-target "VehicleSubsystems")
        (range (start 34 17) (end 34 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 17) (end 57 39)) (probe (position 57 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))
        (kind membershipImport) (ordinal 0) (authored-target "PowerTrainRequirements")
        (range (start 57 17) (end 57 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 16) (end 5 41)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))
        (kind membershipImport) (ordinal 0) (authored-target "MassLimitationRequirement")
        (range (start 5 16) (end 5 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 42)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "2a-Parts Interconnection::*")
        (range (start 1 16) (end 1 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 17) (end 51 44)) (probe (position 51 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))
        (kind membershipImport) (ordinal 0) (authored-target "BodyAndInteriorRequirements")
        (range (start 51 17) (end 51 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 17) (end 22 81)) (probe (position 22 17))
      (reference
        (source (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1 Specification Context::vehicle1-c1 Specification")
        (range (start 22 17) (end 22 81))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
