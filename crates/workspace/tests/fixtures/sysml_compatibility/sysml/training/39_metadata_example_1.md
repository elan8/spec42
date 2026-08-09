# META
~~~ini
description=SysML Training 39 (Metadata): Metadata Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def SecurityFeature {
		:> annotatedElement : SysML::PartDefinition;
		:> annotatedElement : SysML::PartUsage;
	}
	
	metadata SafetyFeature about 
		vehicle::interior::seatBelt,
		vehicle::interior::driverAirBag,
		vehicle::bodyAssy::bumper;
	
	metadata SecurityFeature about
		vehicle::interior::alarm,
		vehicle::bodyAssy::keylessEntry;
		
	part vehicle {
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
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwMetadata,KwDef,Ident,Semicolon,
KwMetadata,KwDef,Ident,OpenCurly,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,Ident,KwAbout,
Ident,ColonColon,Ident,ColonColon,Ident,Comma,
Ident,ColonColon,Ident,ColonColon,Ident,Comma,
Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwMetadata,Ident,KwAbout,
Ident,ColonColon,Ident,ColonColon,Ident,Comma,
Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Metadata Example-1''
    (metadata_def 'SafetyFeature')
    (metadata_def 'SecurityFeature'
      (default_ref_usage :> 'annotatedElement' : 'SysML::PartDefinition')
      (default_ref_usage :> 'annotatedElement' : 'SysML::PartUsage'))
    (metadata_feature typed 'SafetyFeature' about 'vehicle::interior::seatBelt', 'vehicle::interior::driverAirBag', 'vehicle::bodyAssy::bumper')
    (metadata_feature typed 'SecurityFeature' about 'vehicle::interior::alarm', 'vehicle::bodyAssy::keylessEntry')
    (part_usage 'vehicle'
      (part_usage 'interior'
        (part_usage 'alarm')
        (part_usage 'seatBelt' multiplicity)
        (part_usage 'frontSeat' multiplicity)
        (part_usage 'driverAirBag'))
      (part_usage 'bodyAssy'
        (part_usage 'body')
        (part_usage 'bumper')
        (part_usage 'keylessEntry')))))
~~~
# FORMAT
~~~sysml
package 'Metadata Example-1' {

    metadata def SafetyFeature;
    metadata def SecurityFeature {
        :> annotatedElement : SysML::PartDefinition;
        :> annotatedElement : SysML::PartUsage;
    }

    metadata SafetyFeature about
    vehicle::interior::seatBelt,
    vehicle::interior::driverAirBag,
    vehicle::bodyAssy::bumper;

    metadata SecurityFeature about
    vehicle::interior::alarm,
    vehicle::bodyAssy::keylessEntry;

    part vehicle {
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

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartUsage'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartUsage'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Metadata Example-1"))) (name "Metadata Example-1") (declared-name "Metadata Example-1")
      (contains
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Metadata Example-1::SafetyFeature"))) (name "SafetyFeature") (declared-name "SafetyFeature"))
        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "Metadata Example-1::SafetyFeature#metadata_usage"))) (name "SafetyFeature") (declared-name "SafetyFeature"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature"))) (name "SecurityFeature") (declared-name "SecurityFeature")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature")))))
          )
        )
        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature#metadata_usage"))) (name "SecurityFeature") (declared-name "SecurityFeature"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (name "bodyAssy") (declared-name "bodyAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::body"))) (name "body") (declared-name "body") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::bumper"))) (name "bumper") (declared-name "bumper") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))) (name "interior") (declared-name "interior") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::alarm"))) (name "alarm") (declared-name "alarm") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::frontSeat"))) (name "frontSeat") (declared-name "frontSeat") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::seatBelt"))) (name "seatBelt") (declared-name "seatBelt") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))) (to (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))) (to (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))))
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
  (document "sysml/training/39_metadata_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_about_unresolved")
        (source "semantic")
        (range (start 8 1) (end 8 125))
      )
      (diagnostic
        (severity warning)
        (code "metadata_about_unresolved")
        (source "semantic")
        (range (start 8 1) (end 8 125))
      )
      (diagnostic
        (severity warning)
        (code "metadata_about_unresolved")
        (source "semantic")
        (range (start 8 1) (end 8 125))
      )
      (diagnostic
        (severity warning)
        (code "metadata_about_unresolved")
        (source "semantic")
        (range (start 13 1) (end 13 94))
      )
      (diagnostic
        (severity warning)
        (code "metadata_about_unresolved")
        (source "semantic")
        (range (start 13 1) (end 13 94))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 19 3) (end 19 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 20 3) (end 20 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 21 3) (end 21 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 22 3) (end 22 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 25 3) (end 25 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 3) (end 26 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 27 3) (end 27 21))
      )
    )
  )
)
~~~
