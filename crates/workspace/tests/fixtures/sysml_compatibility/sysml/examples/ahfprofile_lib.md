# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFProfileLib
type=file
~~~
# SOURCE
~~~sysml
library package AHFProfileLib {
	// Systems and Services and their functionalities
	private import ScalarValues::*;
	
	// Design level
	port def SD{
		doc /* Service definition */
		
		attribute serviceDefinition:String;
		attribute serviceURL:String;
		attribute intrfce_protocol:String; // which may be "REST" or "MQTT" etc.		 
	}	
		
	part def SysLocalCloudsDesign {
		doc /* System of Systems Definition */	

		// System of Local Clouds 
		part locclouds:LocalCloudDesign[1..*];
	}
	
	part system_of_systems:SysLocalCloudsDD; // defining a top level usage
	
	part def LocalCloudDesign {
		doc /* Local Cloud definition */

		part systems:SysD[1..*];	
	}

	part def SysD {
		doc /* System definitions */	

		port services: SD[1..*];
		attribute systemname: String;
		attribute address: String;
		attribute portno: Integer;
	}	

	// Design Description level
	port def IDD :> SD{
		doc /* Interface Design Description of services */
		
		attribute encoding_kind:String;
	}
	
	port def SDDD :> SD{
		doc /* Service Definition Design Description */
		
		port idds:IDD[*]; // nested protocol-specific services
	}	

	part def SysLocalCloudsDD :> SysLocalCloudsDesign {
		doc /* System of Systems Detailed Description */	

		part :>> locclouds:LocalCloudDD[1..*]; // the descriptions
	}

	part def LocalCloudDD :> LocalCloudDesign {
		part :>> systems:SysDD[1..*];
	}

	part def SysDD :> SysD{
		doc /* System Detailed Description */

		port :>> services:SDDD;
		action ServiceMethod[1..*]; //means general behaviors
	}
}

library package AHFProfileMetadata{
	private import Metaobjects::SemanticMetadata;
	private import AHFProfileLib::*;

	port global_sd:SD;
	metadata def <service> SDMetadata :> SemanticMetadata{
		// :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
		// :>> baseType = SysD::services meta SysML::PortUsage;
		:>> baseType default global_sd meta SysML::PortUsage;
	}
	
	metadata def <sos> SysLocalCloudsMetadata :> SemanticMetadata{
		:>> baseType = system_of_systems meta SysML::PartUsage;
	}
	
	metadata def <cloud> LocalCloudsMetadata :> SemanticMetadata{
		:>> baseType default system_of_systems::locclouds meta SysML::PartUsage;
	}
	
	metadata def <system> SysDMetadata :> SemanticMetadata{
		:>> baseType default system_of_systems::locclouds::systems meta SysML::PartUsage;
		// :>> baseType default LocalCloudDesign::systems meta SysML::PartUsage;
	}

	metadata def <idd> IDDMetadata :> SDMetadata{
		// :>> baseType = system_of_systems.locclouds.systems.services.idd meta SysML::PortUsage;
		:>> baseType = SDDD::idds meta SysML::PortUsage;
		// :>> global_sddd.idd;
	}

	port global_sddd:SDDD;
	metadata def <servicedd> SDDDMetadata :> SDMetadata {
		// :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
		:>> baseType = global_sddd meta SysML::PortUsage;
	}
	
	metadata def <clouddd> LocalCloudsDDMetadata :> LocalCloudsMetadata{
		:>> baseType = system_of_systems::locclouds meta SysML::PartUsage;
	}
	
	part global_clouddd:LocalCloudDD;
	part global_systemsdd:SysDD;
	metadata def <systemdd> SysDDMetadata :> SysDMetadata{
		// :>> baseType = system_of_systems.locclouds.systems meta SysML::PartUsage;
		//:>> baseType = LocalCloudDD::systems meta SysML::PartUsage;
		:>> baseType = global_systemsdd meta SysML::PartUsage;
	}	
	
}
~~~
# TOKENS
~~~zig
KwLibrary,KwPackage,Ident,OpenCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwPort,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
LineComment,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,LineComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
LineComment,
KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwPort,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwPort,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAction,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,
KwLibrary,KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
ColonGtGt,Ident,KwDefault,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,KwDefault,Ident,ColonColon,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,KwDefault,Ident,ColonColon,Ident,ColonColon,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
LineComment,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
LineComment,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
LineComment,
CloseCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
LineComment,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (library_package_def 'AHFProfileLib'
    (line_comment)
    (import_decl private 'ScalarValues::*')
    (line_comment)
    (port_def 'SD'
      (documentation)
      (attribute_usage 'serviceDefinition' : 'String')
      (attribute_usage 'serviceURL' : 'String')
      (attribute_usage 'intrfce_protocol' : 'String')
      (line_comment))
    (part_def 'SysLocalCloudsDesign'
      (documentation)
      (line_comment)
      (part_usage 'locclouds' : 'LocalCloudDesign' multiplicity))
    (part_usage 'system_of_systems' : 'SysLocalCloudsDD')
    (line_comment)
    (part_def 'LocalCloudDesign'
      (documentation)
      (part_usage 'systems' : 'SysD' multiplicity))
    (part_def 'SysD'
      (documentation)
      (port_usage 'services' : 'SD' multiplicity)
      (attribute_usage 'systemname' : 'String')
      (attribute_usage 'address' : 'String')
      (attribute_usage 'portno' : 'Integer'))
    (line_comment)
    (port_def 'IDD' :> 'SD'
      (documentation)
      (attribute_usage 'encoding_kind' : 'String'))
    (port_def 'SDDD' :> 'SD'
      (documentation)
      (port_usage 'idds' : 'IDD' multiplicity)
      (line_comment))
    (part_def 'SysLocalCloudsDD' :> 'SysLocalCloudsDesign'
      (documentation)
      (part_usage :>> 'locclouds' : 'LocalCloudDD' multiplicity)
      (line_comment))
    (part_def 'LocalCloudDD' :> 'LocalCloudDesign'
      (part_usage :>> 'systems' : 'SysDD' multiplicity))
    (part_def 'SysDD' :> 'SysD'
      (documentation)
      (port_usage :>> 'services' : 'SDDD')
      (action_usage 'ServiceMethod' multiplicity)
      (line_comment)))
  (library_package_def 'AHFProfileMetadata'
    (import_decl private 'Metaobjects::SemanticMetadata')
    (import_decl private 'AHFProfileLib::*')
    (port_usage 'global_sd' : 'SD')
    (metadata_def 'SDMetadata' :> 'SemanticMetadata'
      (line_comment)
      (line_comment)
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'SysLocalCloudsMetadata' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'LocalCloudsMetadata' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'SysDMetadata' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value)
      (line_comment))
    (metadata_def 'IDDMetadata' :> 'SDMetadata'
      (line_comment)
      (default_ref_usage :>> 'baseType' value)
      (line_comment))
    (port_usage 'global_sddd' : 'SDDD')
    (metadata_def 'SDDDMetadata' :> 'SDMetadata'
      (line_comment)
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'LocalCloudsDDMetadata' :> 'LocalCloudsMetadata'
      (default_ref_usage :>> 'baseType' value))
    (part_usage 'global_clouddd' : 'LocalCloudDD')
    (part_usage 'global_systemsdd' : 'SysDD')
    (metadata_def 'SysDDMetadata' :> 'SysDMetadata'
      (line_comment)
      (line_comment)
      (default_ref_usage :>> 'baseType' value))))
~~~
# FORMAT
~~~sysml
library package AHFProfileLib {
    // Systems and Services and their functionalities
    private import ScalarValues::*;

    // Design level
    port def SD {
        doc /* Service definition */

        attribute serviceDefinition : String;
        attribute serviceURL : String;
        attribute intrfce_protocol : String;
        // which may be "REST" or "MQTT" etc.		 
    }

    part def SysLocalCloudsDesign {
        doc /* System of Systems Definition */

        // System of Local Clouds 
        part locclouds : LocalCloudDesign [1..*];
    }

    part system_of_systems : SysLocalCloudsDD;
    // defining a top level usage

    part def LocalCloudDesign {
        doc /* Local Cloud definition */

        part systems : SysD [1..*];
    }

    part def SysD {
        doc /* System definitions */

        port services : SD [1..*];
        attribute systemname : String;
        attribute address : String;
        attribute portno : Integer;
    }

    // Design Description level
    port def IDD :> SD {
        doc /* Interface Design Description of services */

        attribute encoding_kind : String;
    }

    port def SDDD :> SD {
        doc /* Service Definition Design Description */

        port idds : IDD [*];
        // nested protocol-specific services
    }

    part def SysLocalCloudsDD :> SysLocalCloudsDesign {
        doc /* System of Systems Detailed Description */

        part :>> locclouds : LocalCloudDD [1..*];
        // the descriptions
    }

    part def LocalCloudDD :> LocalCloudDesign {
        part :>> systems : SysDD [1..*];
    }

    part def SysDD :> SysD {
        doc /* System Detailed Description */

        port :>> services : SDDD;
        action ServiceMethod[1..*];
        //means general behaviors
    }
}

library package AHFProfileMetadata {
    private import Metaobjects::SemanticMetadata;
    private import AHFProfileLib::*;

    port global_sd : SD;
    metadata def <service> SDMetadata :> SemanticMetadata {
        // :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
        // :>> baseType = SysD::services meta SysML::PortUsage;
        :>> baseType default = global_sd meta SysML::PortUsage;
    }

    metadata def <sos> SysLocalCloudsMetadata :> SemanticMetadata {
        :>> baseType = system_of_systems meta SysML::PartUsage;
    }

    metadata def <cloud> LocalCloudsMetadata :> SemanticMetadata {
        :>> baseType default = system_of_systems::locclouds meta SysML::PartUsage;
    }

    metadata def <system> SysDMetadata :> SemanticMetadata {
        :>> baseType default = system_of_systems::locclouds::systems meta SysML::PartUsage;
        // :>> baseType default LocalCloudDesign::systems meta SysML::PartUsage;
    }

    metadata def <idd> IDDMetadata :> SDMetadata {
        // :>> baseType = system_of_systems.locclouds.systems.services.idd meta SysML::PortUsage;
        :>> baseType = SDDD::idds meta SysML::PortUsage;
        // :>> global_sddd.idd;
    }

    port global_sddd : SDDD;
    metadata def <servicedd> SDDDMetadata :> SDMetadata {
        // :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
        :>> baseType = global_sddd meta SysML::PortUsage;
    }

    metadata def <clouddd> LocalCloudsDDMetadata :> LocalCloudsMetadata {
        :>> baseType = system_of_systems::locclouds meta SysML::PartUsage;
    }

    part global_clouddd : LocalCloudDD;
    part global_systemsdd : SysDD;
    metadata def <systemdd> SysDDMetadata :> SysDMetadata {
        // :>> baseType = system_of_systems.locclouds.systems meta SysML::PartUsage;
        //:>> baseType = LocalCloudDD::systems meta SysML::PartUsage;
        :>> baseType = global_systemsdd meta SysML::PartUsage;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'String'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'AHFProfileLib'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (port_def 'SD'
        (documentation)
        (attribute_usage composite 'serviceDefinition' : 'String'[unresolved])
        (attribute_usage composite 'serviceURL' : 'String'[unresolved])
        (attribute_usage composite 'intrfce_protocol' : 'String'[unresolved]))
      (part_def 'SysLocalCloudsDesign'
        (documentation)
        (part_usage composite 'locclouds' : 'AHFProfileLib::LocalCloudDesign'[part_def]
          (multiplicity_range [1..*])))
      (part_usage 'system_of_systems' : 'AHFProfileLib::SysLocalCloudsDD'[part_def])
      (part_def 'LocalCloudDesign'
        (documentation)
        (part_usage composite 'systems' : 'AHFProfileLib::SysD'[part_def]
          (multiplicity_range [1..*])))
      (part_def 'SysD'
        (documentation)
        (port_usage composite 'services' : 'AHFProfileLib::SD'[port_def]
          (multiplicity_range [1..*]))
        (attribute_usage composite 'systemname' : 'String'[unresolved])
        (attribute_usage composite 'address' : 'String'[unresolved])
        (attribute_usage composite 'portno' : 'Integer'[unresolved]))
      (port_def 'IDD' :> 'AHFProfileLib::SD'[port_def]
        (documentation)
        (attribute_usage composite 'encoding_kind' : 'String'[unresolved]))
      (port_def 'SDDD' :> 'AHFProfileLib::SD'[port_def]
        (documentation)
        (port_usage composite 'idds' : 'AHFProfileLib::IDD'[port_def]
          (multiplicity_range [*])))
      (part_def 'SysLocalCloudsDD' :> 'AHFProfileLib::SysLocalCloudsDesign'[part_def]
        (documentation)
        (part_usage composite :>> 'AHFProfileLib::SysLocalCloudsDesign::locclouds'[part_usage] : 'AHFProfileLib::LocalCloudDD'[part_def]
          (multiplicity_range [1..*])))
      (part_def 'LocalCloudDD' :> 'AHFProfileLib::LocalCloudDesign'[part_def]
        (part_usage composite :>> 'AHFProfileLib::LocalCloudDesign::systems'[part_usage] : 'AHFProfileLib::SysDD'[part_def]
          (multiplicity_range [1..*])))
      (part_def 'SysDD' :> 'AHFProfileLib::SysD'[part_def]
        (documentation)
        (port_usage composite :>> 'AHFProfileLib::SysD::services'[port_usage] : 'AHFProfileLib::SDDD'[port_def])
        (action_usage composite 'ServiceMethod'
          (multiplicity_range [1..*]))))
    (library_package 'AHFProfileMetadata'
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (namespace_import private -> 'AHFProfileLib'[library_package])
      (port_usage 'global_sd' : 'AHFProfileLib::SD'[port_def])
      (metadata_def 'SDMetadata' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (default =))))
      (metadata_def 'SysLocalCloudsMetadata' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'LocalCloudsMetadata' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (default =))))
      (metadata_def 'SysDMetadata' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (default =))))
      (metadata_def 'IDDMetadata' :> 'AHFProfileMetadata::SDMetadata'[metadata_def]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (port_usage 'global_sddd' : 'AHFProfileLib::SDDD'[port_def])
      (metadata_def 'SDDDMetadata' :> 'AHFProfileMetadata::SDMetadata'[metadata_def]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'LocalCloudsDDMetadata' :> 'AHFProfileMetadata::LocalCloudsMetadata'[metadata_def]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (part_usage 'global_clouddd' : 'AHFProfileLib::LocalCloudDD'[part_def])
      (part_usage 'global_systemsdd' : 'AHFProfileLib::SysDD'[part_def])
      (metadata_def 'SysDDMetadata' :> 'AHFProfileMetadata::SysDMetadata'[metadata_def]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=)))))))
~~~
