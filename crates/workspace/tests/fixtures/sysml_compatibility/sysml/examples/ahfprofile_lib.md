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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AHFProfileLib"))) (name "AHFProfileLib") (declared-name "AHFProfileLib")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFProfileLib::*"))) (name "*") (declared-name "*"))
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (name "IDD") (declared-name "IDD")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::IDD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (name "encoding_kind") (declared-name "encoding_kind") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::IDD")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::~IDD"))) (name "~IDD") (declared-name "~IDD") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::IDD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (name "LocalCloudDD") (declared-name "LocalCloudDD") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (name "systems") (declared-name "systems") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (name "LocalCloudDesign") (declared-name "LocalCloudDesign") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (name "systems") (declared-name "systems") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (name "SD") (declared-name "SD")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (name "intrfce_protocol") (declared-name "intrfce_protocol") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (name "serviceDefinition") (declared-name "serviceDefinition") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (name "serviceURL") (declared-name "serviceURL") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFProfileLib::SD::~SD"))) (name "~SD") (declared-name "~SD") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (name "SDDD") (declared-name "SDDD")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (name "idds") (declared-name "idds") (declared (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::~SDDD"))) (name "~SDDD") (declared-name "~SDDD") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (name "SysD") (declared-name "SysD") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (name "address") (declared-name "address") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (name "portno") (declared-name "portno") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (name "services") (declared-name "services") (declared (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (name "systemname") (declared-name "systemname") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (name "SysDD") (declared-name "SysDD") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::ServiceMethod"))) (name "ServiceMethod") (declared-name "ServiceMethod") (declared (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (name "services") (declared-name "services") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (name "SysLocalCloudsDD") (declared-name "SysLocalCloudsDD") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (name "locclouds") (declared-name "locclouds") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (name "SysLocalCloudsDesign") (declared-name "SysLocalCloudsDesign") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (name "locclouds") (declared-name "locclouds") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (name "system_of_systems") (declared-name "system_of_systems") (declared (properties (ordered false))))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "AHFProfileMetadata"))) (name "AHFProfileMetadata") (declared-name "AHFProfileMetadata")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFProfileMetadata::*"))) (name "*") (declared-name "*"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (name "IDDMetadata") (declared-name "IDDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (name "LocalCloudsDDMetadata") (declared-name "LocalCloudsDDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (name "LocalCloudsMetadata") (declared-name "LocalCloudsMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (name "SDDDMetadata") (declared-name "SDDDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (name "SDMetadata") (declared-name "SDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (name "SysDDMetadata") (declared-name "SysDDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (name "SysDMetadata") (declared-name "SysDMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (name "SysLocalCloudsMetadata") (declared-name "SysLocalCloudsMetadata")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (name "global_clouddd") (declared-name "global_clouddd") (declared (properties (ordered false))))
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (name "global_sd") (declared-name "global_sd")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd::~global_sd"))) (name "~global_sd") (declared-name "~global_sd") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (name "global_sddd") (declared-name "global_sddd")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd::~global_sddd"))) (name "~global_sddd") (declared-name "~global_sddd") (effective (featuring-type (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (name "global_systemsdd") (declared-name "global_systemsdd") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::IDD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SDDD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysDD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::_documentation"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::IDD::~IDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SD::~SD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SDDD::~SDDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd::~global_sd"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd::~global_sddd"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysD"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (to (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (to (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (to (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (to (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (to (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (to (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
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
  (document "sysml/examples/ahfprofile_lib.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 2) (end 34 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 69 1) (end 69 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 73 1) (end 73 260))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 1) (end 79 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 1) (end 83 140))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 87 1) (end 87 218))
      )
    )
  )
)
~~~
