# META
~~~ini
description=SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group
type=file
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group' {
	
	part vehicle1_c1 {
		part interior {
			part alarm;
			part seatBelt[2];
			part frontSeat[2];
			part driverAirBag;
		}
		part bodyAssy {
			part body;
			part bumper;
			part keylessEntry;
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */
		
		public import vehicle1_c1::interior::seatBelt;
		public import vehicle1_c1::interior::driverAirBag;
		public import vehicle1_c1::bodyAssy::bumper;		
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */
		
		public import vehicle1_c1::interior::alarm;
		public import vehicle1_c1::bodyAssy::keylessEntry;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety AND
		 * parts that contribute to security.
		 */
		 
		public import 'Safety Features'::*;
		public import 'Security Features'::*;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''13b-Safety and Security Features Element Group''
    (part_usage 'vehicle1_c1'
      (part_usage 'interior'
        (part_usage 'alarm')
        (part_usage 'seatBelt' multiplicity)
        (part_usage 'frontSeat' multiplicity)
        (part_usage 'driverAirBag'))
      (part_usage 'bodyAssy'
        (part_usage 'body')
        (part_usage 'bumper')
        (part_usage 'keylessEntry')))
    (package_def ''Safety Features''
      (comment)
      (import_decl public 'vehicle1_c1::interior::seatBelt')
      (import_decl public 'vehicle1_c1::interior::driverAirBag')
      (import_decl public 'vehicle1_c1::bodyAssy::bumper'))
    (package_def ''Security Features''
      (comment)
      (import_decl public 'vehicle1_c1::interior::alarm')
      (import_decl public 'vehicle1_c1::bodyAssy::keylessEntry'))
    (package_def ''Safety & Security Features''
      (comment)
      (import_decl public ''Safety Features'::*')
      (import_decl public ''Security Features'::*'))))
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group' {
    part vehicle1_c1 {
        part interior {
            part alarm;
            part seatBelt [2];
            part frontSeat [2];
            part driverAirBag;
        }
        part bodyAssy {
            part body;
            part bumper;
            part keylessEntry;
        }
    }

    package 'Safety Features' {
        /* Parts that contribute to safety. */

        public import vehicle1_c1::interior::seatBelt;
        public import vehicle1_c1::interior::driverAirBag;
        public import vehicle1_c1::bodyAssy::bumper;
    }

    package 'Security Features' {
        /* Parts that contribute to security. */

        public import vehicle1_c1::interior::alarm;
        public import vehicle1_c1::bodyAssy::keylessEntry;
    }

    package 'Safety & Security Features' {
        /* Parts that contribute to safety AND
		 * parts that contribute to security.
		 */

        public import 'Safety Features'::*;
        public import 'Security Features'::*;
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
    (package '13b-Safety and Security Features Element Group'
      (part_usage 'vehicle1_c1'
        (part_usage composite 'interior'
          (part_usage composite 'alarm')
          (part_usage composite 'seatBelt'
            (multiplicity_range [2]))
          (part_usage composite 'frontSeat'
            (multiplicity_range [2]))
          (part_usage composite 'driverAirBag'))
        (part_usage composite 'bodyAssy'
          (part_usage composite 'body')
          (part_usage composite 'bumper')
          (part_usage composite 'keylessEntry')))
      (package 'Safety Features'
        (membership_import public -> '13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt'[part_usage])
        (membership_import public -> '13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag'[part_usage])
        (membership_import public -> '13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper'[part_usage]))
      (package 'Security Features'
        (membership_import public -> '13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm'[part_usage])
        (membership_import public -> '13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry'[part_usage]))
      (package 'Safety & Security Features'
        (namespace_import public -> '13b-Safety and Security Features Element Group::Safety Features'[package])
        (namespace_import public -> '13b-Safety and Security Features Element Group::Security Features'[package])))))
~~~
