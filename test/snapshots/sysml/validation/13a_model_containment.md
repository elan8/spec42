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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPackage,UnrestrictedName,OpenCurly,
KwAlias,UnrestrictedName,KwFor,Ident,Semicolon,
KwPublic,KwImport,UnrestrictedName,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPublic,KwImport,Ident,Semicolon,
KwPublic,KwImport,Ident,Semicolon,
MultilineNote,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,UnrestrictedName,Semicolon,
KwPublic,KwImport,UnrestrictedName,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
KwPublic,KwImport,Ident,Semicolon,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''13a-Model Containment''
    (import_decl private ''2a-Parts Interconnection'::*')
    (import_decl private ''8-Requirements'::*')
    (requirement_usage 'BodyAndInteriorRequirements'
      (import_decl public 'MassLimitationRequirement'))
    (requirement_usage 'PowerTrainRequirements')
    (package_def ''Vehicle Model''
      (documentation)
      (package_def ''Vehicle1-Configuration''
        (alias_member ''Sport Sedan'' for 'vehicle1_c1')
        (import_decl public ''vehicle1_c1 Specification Context'::'vehicle1-c1 Specification''))
      (package_def ''Vehicle Reference Model''
        (documentation)
        (import_decl public 'VehicleA')
        (import_decl public 'VehicleSubsystems')
        (multiline_note))
      (package_def 'VehicleSubsystems'
        (import_decl public ''Body&Interior'')
        (import_decl public ''PowerTrain''))
      (package_def ''Body&Interior''
        (import_decl public 'BodyAndInteriorRequirements'))
      (package_def 'PowerTrain'
        (import_decl public 'Engine')
        (import_decl public 'Transmission')
        (import_decl public 'PowerTrainRequirements')))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "271ceb93d7f65d82bc356b164d769686aec56827b3274426bcf7e098e3810532") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13a-Model Containment"))) (kind "package") (name "13a-Model Containment") (declared-name "13a-Model Containment") (range (start (line 0) (character 0)) (end (line 0) (character 1428))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "13a-Model Containment"))) (authored (membership (kind Import) (visibility "private") (import (reference "2a-Parts Interconnection::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 42))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 36))) (parent (node (document "d0") (qualified-name "13a-Model Containment"))) (authored (membership (kind Import) (visibility "private") (import (reference "8-Requirements::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 32))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (kind "requirement") (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements") (range (start (line 4) (character 1)) (end (line 4) (character 89))) (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))) (kind "import") (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement") (range (start (line 5) (character 2)) (end (line 5) (character 42))) (parent (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "MassLimitationRequirement") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 41))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::PowerTrainRequirements"))) (kind "requirement") (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements") (range (start (line 8) (character 1)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))) (kind "package") (name "Vehicle Model") (declared-name "Vehicle Model") (range (start (line 10) (character 1)) (end (line 10) (character 1173))) (parent (node (document "d0") (qualified-name "13a-Model Containment"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (kind "package") (name "Body&Interior") (declared-name "Body&Interior") (range (start (line 50) (character 2)) (end (line 50) (character 80))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))) (kind "import") (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements") (range (start (line 51) (character 3)) (end (line 51) (character 45))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (authored (membership (kind Import) (visibility "public") (import (reference "BodyAndInteriorRequirements") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 51) (character 17)) (end (line 51) (character 44))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (kind "package") (name "PowerTrain") (declared-name "PowerTrain") (range (start (line 54) (character 2)) (end (line 54) (character 126))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))) (kind "import") (name "Engine") (declared-name "Engine") (range (start (line 55) (character 3)) (end (line 55) (character 24))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "Engine") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 55) (character 17)) (end (line 55) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))) (kind "import") (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements") (range (start (line 57) (character 3)) (end (line 57) (character 40))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "PowerTrainRequirements") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 57) (character 17)) (end (line 57) (character 39))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))) (kind "import") (name "Transmission") (declared-name "Transmission") (range (start (line 56) (character 3)) (end (line 56) (character 30))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (authored (membership (kind Import) (visibility "public") (import (reference "Transmission") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 56) (character 17)) (end (line 56) (character 29))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (kind "package") (name "Vehicle Reference Model") (declared-name "Vehicle Reference Model") (range (start (line 25) (character 2)) (end (line 25) (character 487))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))) (kind "import") (name "VehicleA") (declared-name "VehicleA") (range (start (line 33) (character 3)) (end (line 33) (character 26))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleA") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 33) (character 17)) (end (line 33) (character 25))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))) (kind "import") (name "VehicleSubsystems") (declared-name "VehicleSubsystems") (range (start (line 34) (character 3)) (end (line 34) (character 35))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleSubsystems") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 34) (character 17)) (end (line 34) (character 34))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::_documentation"))) (kind "documentation") (name "") (range (start (line 25) (character 2)) (end (line 25) (character 487))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (kind "package") (name "Vehicle1-Configuration") (declared-name "Vehicle1-Configuration") (range (start (line 19) (character 2)) (end (line 19) (character 172))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (kind "alias") (name "Sport Sedan") (declared-name "Sport Sedan") (range (start (line 20) (character 3)) (end (line 20) (character 39))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))) (kind "import") (name "vehicle1-c1 Specification") (declared-name "vehicle1-c1 Specification") (range (start (line 22) (character 3)) (end (line 22) (character 82))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1 Specification Context::vehicle1-c1 Specification") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 17)) (end (line 22) (character 81))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (kind "package") (name "VehicleSubsystems") (declared-name "VehicleSubsystems") (range (start (line 45) (character 2)) (end (line 45) (character 98))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))) (kind "import") (name "Body&Interior") (declared-name "Body&Interior") (range (start (line 46) (character 3)) (end (line 46) (character 33))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (authored (membership (kind Import) (visibility "public") (import (reference "Body&Interior") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 46) (character 17)) (end (line 46) (character 32))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))) (kind "import") (name "PowerTrain") (declared-name "PowerTrain") (range (start (line 47) (character 3)) (end (line 47) (character 30))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (authored (membership (kind Import) (visibility "public") (import (reference "PowerTrain") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 47) (character 17)) (end (line 47) (character 29))))))
    (element (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::_documentation"))) (kind "documentation") (name "") (range (start (line 10) (character 1)) (end (line 10) (character 1173))) (parent (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "2a-Parts Interconnection::*") (range (start (line 1) (character 16)) (end (line 1) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "8-Requirements::*") (range (start (line 2) (character 16)) (end (line 2) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))) (kind membershipImport) (ordinal 0)) (authored-target "MassLimitationRequirement") (range (start (line 5) (character 16)) (end (line 5) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))) (kind membershipImport) (ordinal 0)) (authored-target "BodyAndInteriorRequirements") (range (start (line 51) (character 17)) (end (line 51) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))) (kind membershipImport) (ordinal 0)) (authored-target "Engine") (range (start (line 55) (character 17)) (end (line 55) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))) (kind membershipImport) (ordinal 0)) (authored-target "PowerTrainRequirements") (range (start (line 57) (character 17)) (end (line 57) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))) (kind membershipImport) (ordinal 0)) (authored-target "Transmission") (range (start (line 56) (character 17)) (end (line 56) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleA") (range (start (line 33) (character 17)) (end (line 33) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleSubsystems") (range (start (line 34) (character 17)) (end (line 34) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1 Specification Context::vehicle1-c1 Specification") (range (start (line 22) (character 17)) (end (line 22) (character 81))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))) (kind membershipImport) (ordinal 0)) (authored-target "Body&Interior") (range (start (line 46) (character 17)) (end (line 46) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))) (kind membershipImport) (ordinal 0)) (authored-target "PowerTrain") (range (start (line 47) (character 17)) (end (line 47) (character 29))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
