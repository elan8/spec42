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
        doc /*
		 * This package is used to represent a top-level "model".
		 * There is no specific syntax for identifying a package
		 * used in this way.
		 */

        package 'Vehicle1-Configuration' {
            alias 'Sport Sedan' for vehicle1_c1;

            public import 'vehicle1_c1 Specification Context'::'vehicle1-c1 Specification';
        }

        package 'Vehicle Reference Model' {
            doc /*
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
(model
  (namespace
    (package '13a-Model Containment'
      (namespace_import private -> '2a-Parts Interconnection'[unresolved])
      (namespace_import private -> '8-Requirements'[unresolved])
      (requirement_usage 'BodyAndInteriorRequirements'
        (membership_import public -> 'MassLimitationRequirement'[unresolved]))
      (requirement_usage 'PowerTrainRequirements')
      (package 'Vehicle Model'
        (documentation)
        (package 'Vehicle1-Configuration'
          (alias_member 'Sport Sedan' -> 'vehicle1_c1'[unresolved])
          (membership_import public -> 'vehicle1_c1 Specification Context::vehicle1-c1 Specification'[unresolved]))
        (package 'Vehicle Reference Model'
          (documentation)
          (membership_import public -> 'VehicleA'[unresolved])
          (membership_import public -> '13a-Model Containment::Vehicle Model::VehicleSubsystems'[package]))
        (package 'VehicleSubsystems'
          (membership_import public -> '13a-Model Containment::Vehicle Model::Body&Interior'[package])
          (membership_import public -> '13a-Model Containment::Vehicle Model::PowerTrain'[package]))
        (package 'Body&Interior'
          (membership_import public -> '13a-Model Containment::BodyAndInteriorRequirements'[requirement_usage]))
        (package 'PowerTrain'
          (membership_import public -> 'Engine'[unresolved])
          (membership_import public -> 'Transmission'[unresolved])
          (membership_import public -> '13a-Model Containment::PowerTrainRequirements'[requirement_usage]))))))
~~~
