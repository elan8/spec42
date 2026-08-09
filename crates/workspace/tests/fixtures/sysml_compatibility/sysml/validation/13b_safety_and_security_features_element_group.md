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
    (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))) (name "13b-Safety and Security Features Element Group") (declared-name "13b-Safety and Security Features Element Group")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (name "Safety & Security Features") (declared-name "Safety & Security Features")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))) (name "*") (declared-name "*"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (name "Safety Features") (declared-name "Safety Features")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))) (name "bumper") (declared-name "bumper"))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag"))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))) (name "seatBelt") (declared-name "seatBelt"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (name "Security Features") (declared-name "Security Features")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))) (name "alarm") (declared-name "alarm"))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry"))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (name "bodyAssy") (declared-name "bodyAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body"))) (name "body") (declared-name "body") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper"))) (name "bumper") (declared-name "bumper") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (name "interior") (declared-name "interior") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm"))) (name "alarm") (declared-name "alarm") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat"))) (name "frontSeat") (declared-name "frontSeat") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))))
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt"))) (name "seatBelt") (declared-name "seatBelt") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/13b_safety_and_security_features_element_group.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 3) (end 4 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 3) (end 5 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 3) (end 6 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 7 3) (end 7 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 3) (end 10 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 3) (end 11 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 12 3) (end 12 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 2) (end 19 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 2) (end 20 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 2) (end 21 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 2) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 2) (end 28 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 36 2) (end 36 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 37 2) (end 37 39))
      )
    )
  )
)
~~~
