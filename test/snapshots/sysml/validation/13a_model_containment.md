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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment"))) (name "13a-Model Containment") (declared-name "13a-Model Containment")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::*#import"))) (name "*") (declared-name "*"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements::MassLimitationRequirement"))) (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement"))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "13a-Model Containment::PowerTrainRequirements"))) (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements"))
        (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))) (name "Vehicle Model") (declared-name "Vehicle Model")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior"))) (name "Body&Interior") (declared-name "Body&Interior")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Body&Interior::BodyAndInteriorRequirements"))) (name "BodyAndInteriorRequirements") (declared-name "BodyAndInteriorRequirements"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain"))) (name "PowerTrain") (declared-name "PowerTrain")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Engine"))) (name "Engine") (declared-name "Engine"))
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::PowerTrainRequirements"))) (name "PowerTrainRequirements") (declared-name "PowerTrainRequirements"))
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::PowerTrain::Transmission"))) (name "Transmission") (declared-name "Transmission"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (name "Vehicle Reference Model") (declared-name "Vehicle Reference Model")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleA"))) (name "VehicleA") (declared-name "VehicleA"))
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::VehicleSubsystems"))) (name "VehicleSubsystems") (declared-name "VehicleSubsystems"))
                (element (kind "documentation") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::_documentation"))) (name ""))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration"))) (name "Vehicle1-Configuration") (declared-name "Vehicle1-Configuration")
              (contains
                (element (kind "alias") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::Sport Sedan"))) (name "Sport Sedan") (declared-name "Sport Sedan"))
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle1-Configuration::vehicle1-c1 Specification"))) (name "vehicle1-c1 Specification") (declared-name "vehicle1-c1 Specification"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems"))) (name "VehicleSubsystems") (declared-name "VehicleSubsystems")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::Body&Interior"))) (name "Body&Interior") (declared-name "Body&Interior"))
                (element (kind "import") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::VehicleSubsystems::PowerTrain"))) (name "PowerTrain") (declared-name "PowerTrain"))
              )
            )
            (element (kind "documentation") (id (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::_documentation"))) (name ""))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model::_documentation"))) (to (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::Vehicle Reference Model"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model::_documentation"))) (to (node (document "d0") (qualified-name "13a-Model Containment::Vehicle Model"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "13a-Model Containment::BodyAndInteriorRequirements"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "13a-Model Containment::PowerTrainRequirements"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/13a_model_containment.md"
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
