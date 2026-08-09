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

    @SafetyFeature about vehicle::interior::seatBelt, vehicle::interior::driverAirBag, vehicle::bodyAssy::bumper;

    @SecurityFeature about vehicle::interior::alarm, vehicle::bodyAssy::keylessEntry;

    part vehicle {
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
(model
  (namespace
    (package 'Metadata Example-1'
      (metadata_def 'SafetyFeature')
      (metadata_def 'SecurityFeature'
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::PartDefinition'[unresolved])
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::PartUsage'[unresolved]))
      (metadata_usage :> 'Metadata Example-1::SafetyFeature'[metadata_def] annotated 'Metadata Example-1::vehicle::interior::seatBelt'[part_usage] annotated 'Metadata Example-1::vehicle::interior::driverAirBag'[part_usage] annotated 'Metadata Example-1::vehicle::bodyAssy::bumper'[part_usage])
      (metadata_usage :> 'Metadata Example-1::SecurityFeature'[metadata_def] annotated 'Metadata Example-1::vehicle::interior::alarm'[part_usage] annotated 'Metadata Example-1::vehicle::bodyAssy::keylessEntry'[part_usage])
      (part_usage 'vehicle'
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
          (part_usage composite 'keylessEntry'))))))
~~~
