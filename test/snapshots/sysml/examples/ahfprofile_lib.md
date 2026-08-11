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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfprofile_lib.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 30) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 23) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 29) (end 10 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 2) (end 32 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 24) (end 32 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 2) (end 33 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 21) (end 33 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 2) (end 34 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 20) (end 34 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 2) (end 41 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 26) (end 41 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 69 16) (end 69 45))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "831b6d553f85636950862b5ce74e63e70a82617d66593f903d09d87d74e88b16") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AHFProfileLib"))) (kind "package") (name "AHFProfileLib") (declared-name "AHFProfileLib") (range (start (line 0) (character 0)) (end (line 0) (character 1517))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (kind "port def") (name "IDD") (declared-name "IDD") (range (start (line 38) (character 1)) (end (line 38) (character 113))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD") (range (start (line 38) (character 17)) (end (line 38) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::_documentation"))) (kind "documentation") (name "") (range (start (line 38) (character 1)) (end (line 38) (character 113))) (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind "attribute") (name "encoding_kind") (declared-name "encoding_kind") (range (start (line 41) (character 2)) (end (line 41) (character 33))) (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 41) (character 26)) (end (line 41) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::~IDD"))) (kind "conjugated port definition") (name "~IDD") (declared-name "~IDD") (range (start (line 38) (character 1)) (end (line 38) (character 113))) (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind "part def") (name "LocalCloudDD") (declared-name "LocalCloudDD") (range (start (line 56) (character 1)) (end (line 56) (character 79))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LocalCloudDesign") (range (start (line 56) (character 26)) (end (line 56) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (kind "part") (name "systems") (declared-name "systems") (range (start (line 57) (character 2)) (end (line 57) (character 31))) (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysDD") (range (start (line 57) (character 19)) (end (line 57) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (kind "part def") (name "LocalCloudDesign") (declared-name "LocalCloudDesign") (range (start (line 22) (character 1)) (end (line 22) (character 95))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::_documentation"))) (kind "documentation") (name "") (range (start (line 22) (character 1)) (end (line 22) (character 95))) (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind "part") (name "systems") (declared-name "systems") (range (start (line 25) (character 2)) (end (line 25) (character 26))) (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysD") (range (start (line 25) (character 15)) (end (line 25) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (kind "port def") (name "SD") (declared-name "SD") (range (start (line 5) (character 1)) (end (line 5) (character 197))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::_documentation"))) (kind "documentation") (name "") (range (start (line 5) (character 1)) (end (line 5) (character 197))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind "attribute") (name "intrfce_protocol") (declared-name "intrfce_protocol") (range (start (line 10) (character 2)) (end (line 10) (character 36))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 10) (character 29)) (end (line 10) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind "attribute") (name "serviceDefinition") (declared-name "serviceDefinition") (range (start (line 8) (character 2)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 8) (character 30)) (end (line 8) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind "attribute") (name "serviceURL") (declared-name "serviceURL") (range (start (line 9) (character 2)) (end (line 9) (character 30))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 9) (character 23)) (end (line 9) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::~SD"))) (kind "conjugated port definition") (name "~SD") (declared-name "~SD") (range (start (line 5) (character 1)) (end (line 5) (character 197))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (kind "port def") (name "SDDD") (declared-name "SDDD") (range (start (line 44) (character 1)) (end (line 44) (character 134))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD") (range (start (line 44) (character 18)) (end (line 44) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::_documentation"))) (kind "documentation") (name "") (range (start (line 44) (character 1)) (end (line 44) (character 134))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind "port") (name "idds") (declared-name "idds") (range (start (line 47) (character 2)) (end (line 47) (character 19))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "IDD") (range none)))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::~SDDD"))) (kind "conjugated port definition") (name "~SDDD") (declared-name "~SDDD") (range (start (line 44) (character 1)) (end (line 44) (character 134))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (kind "part def") (name "SysD") (declared-name "SysD") (range (start (line 28) (character 1)) (end (line 28) (character 169))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::_documentation"))) (kind "documentation") (name "") (range (start (line 28) (character 1)) (end (line 28) (character 169))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind "attribute") (name "address") (declared-name "address") (range (start (line 33) (character 2)) (end (line 33) (character 28))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 33) (character 21)) (end (line 33) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind "attribute") (name "portno") (declared-name "portno") (range (start (line 34) (character 2)) (end (line 34) (character 28))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "Integer") (range (start (line 34) (character 20)) (end (line 34) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (kind "port") (name "services") (declared-name "services") (range (start (line 31) (character 2)) (end (line 31) (character 26))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SD") (range none)))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind "attribute") (name "systemname") (declared-name "systemname") (range (start (line 32) (character 2)) (end (line 32) (character 31))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 32) (character 24)) (end (line 32) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (kind "part def") (name "SysDD") (declared-name "SysDD") (range (start (line 60) (character 1)) (end (line 60) (character 150))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysD") (range (start (line 60) (character 19)) (end (line 60) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::ServiceMethod"))) (kind "action") (name "ServiceMethod") (declared-name "ServiceMethod") (range (start (line 64) (character 2)) (end (line 64) (character 29))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::_documentation"))) (kind "documentation") (name "") (range (start (line 60) (character 1)) (end (line 60) (character 150))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind "port") (name "services") (declared-name "services") (range (start (line 63) (character 2)) (end (line 63) (character 25))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SDDD") (range none)) (redefinition (reference "services") (range (start (line 63) (character 11)) (end (line 63) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind "part def") (name "SysLocalCloudsDD") (declared-name "SysLocalCloudsDD") (range (start (line 50) (character 1)) (end (line 50) (character 169))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysLocalCloudsDesign") (range (start (line 50) (character 30)) (end (line 50) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 1)) (end (line 50) (character 169))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (kind "part") (name "locclouds") (declared-name "locclouds") (range (start (line 53) (character 2)) (end (line 53) (character 40))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDD") (range (start (line 53) (character 21)) (end (line 53) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (kind "part def") (name "SysLocalCloudsDesign") (declared-name "SysLocalCloudsDesign") (range (start (line 13) (character 1)) (end (line 13) (character 148))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::_documentation"))) (kind "documentation") (name "") (range (start (line 13) (character 1)) (end (line 13) (character 148))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind "part") (name "locclouds") (declared-name "locclouds") (range (start (line 17) (character 2)) (end (line 17) (character 40))) (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDesign") (range (start (line 17) (character 17)) (end (line 17) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (kind "part") (name "system_of_systems") (declared-name "system_of_systems") (range (start (line 20) (character 1)) (end (line 20) (character 41))) (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysLocalCloudsDD") (range (start (line 20) (character 24)) (end (line 20) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata"))) (kind "package") (name "AHFProfileMetadata") (declared-name "AHFProfileMetadata") (range (start (line 68) (character 0)) (end (line 68) (character 1807))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 70) (character 1)) (end (line 70) (character 33))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileLib::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 70) (character 16)) (end (line 70) (character 29))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind "metadata def") (name "IDDMetadata") (declared-name "IDDMetadata") (range (start (line 92) (character 1)) (end (line 92) (character 218))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDMetadata") (range (start (line 92) (character 35)) (end (line 92) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 94) (character 2)) (end (line 94) (character 50))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 94) (character 2)) (end (line 94) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind "metadata def") (name "LocalCloudsDDMetadata") (declared-name "LocalCloudsDDMetadata") (range (start (line 104) (character 1)) (end (line 104) (character 141))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LocalCloudsMetadata") (range (start (line 104) (character 49)) (end (line 104) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 105) (character 2)) (end (line 105) (character 68))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 105) (character 2)) (end (line 105) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind "metadata def") (name "LocalCloudsMetadata") (declared-name "LocalCloudsMetadata") (range (start (line 83) (character 1)) (end (line 83) (character 140))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 83) (character 45)) (end (line 83) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 84) (character 2)) (end (line 84) (character 74))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 84) (character 2)) (end (line 84) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind "metadata def") (name "SDDDMetadata") (declared-name "SDDDMetadata") (range (start (line 99) (character 1)) (end (line 99) (character 197))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDMetadata") (range (start (line 99) (character 42)) (end (line 99) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 101) (character 2)) (end (line 101) (character 51))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 101) (character 2)) (end (line 101) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind "metadata def") (name "SDMetadata") (declared-name "SDMetadata") (range (start (line 73) (character 1)) (end (line 73) (character 260))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 73) (character 38)) (end (line 73) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 76) (character 2)) (end (line 76) (character 55))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 76) (character 2)) (end (line 76) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 69) (character 1)) (end (line 69) (character 46))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 69) (character 16)) (end (line 69) (character 45))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind "metadata def") (name "SysDDMetadata") (declared-name "SysDDMetadata") (range (start (line 110) (character 1)) (end (line 110) (character 258))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysDMetadata") (range (start (line 110) (character 42)) (end (line 110) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 113) (character 2)) (end (line 113) (character 56))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 113) (character 2)) (end (line 113) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind "metadata def") (name "SysDMetadata") (declared-name "SysDMetadata") (range (start (line 87) (character 1)) (end (line 87) (character 218))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 87) (character 39)) (end (line 87) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 88) (character 2)) (end (line 88) (character 83))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 88) (character 2)) (end (line 88) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind "metadata def") (name "SysLocalCloudsMetadata") (declared-name "SysLocalCloudsMetadata") (range (start (line 79) (character 1)) (end (line 79) (character 124))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 79) (character 46)) (end (line 79) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 80) (character 2)) (end (line 80) (character 57))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 80) (character 2)) (end (line 80) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind "part") (name "global_clouddd") (declared-name "global_clouddd") (range (start (line 108) (character 1)) (end (line 108) (character 34))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDD") (range (start (line 108) (character 21)) (end (line 108) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (kind "port def") (name "global_sd") (declared-name "global_sd") (range (start (line 72) (character 1)) (end (line 72) (character 19))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD") (range (start (line 0) (character 0)) (end (line 0) (character 2)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd::~global_sd"))) (kind "conjugated port definition") (name "~global_sd") (declared-name "~global_sd") (range (start (line 72) (character 1)) (end (line 72) (character 19))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind "port def") (name "global_sddd") (declared-name "global_sddd") (range (start (line 98) (character 1)) (end (line 98) (character 23))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDDD") (range (start (line 0) (character 0)) (end (line 0) (character 4)))))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd::~global_sddd"))) (kind "conjugated port definition") (name "~global_sddd") (declared-name "~global_sddd") (range (start (line 98) (character 1)) (end (line 98) (character 23))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind "part") (name "global_systemsdd") (declared-name "global_systemsdd") (range (start (line 109) (character 1)) (end (line 109) (character 29))) (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysDD") (range (start (line 109) (character 23)) (end (line 109) (character 28)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0)) (authored-target "SD") (range (start (line 38) (character 17)) (end (line 38) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 41) (character 26)) (end (line 41) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0)) (authored-target "LocalCloudDesign") (range (start (line 56) (character 26)) (end (line 56) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysDD") (range (start (line 57) (character 19)) (end (line 57) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysD") (range (start (line 25) (character 15)) (end (line 25) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 10) (character 29)) (end (line 10) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 8) (character 30)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 9) (character 23)) (end (line 9) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0)) (authored-target "SD") (range (start (line 44) (character 18)) (end (line 44) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0)) (authored-target "IDD") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::IDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 33) (character 21)) (end (line 33) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (range (start (line 34) (character 20)) (end (line 34) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0)) (authored-target "SD") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 32) (character 24)) (end (line 32) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0)) (authored-target "SysD") (range (start (line 60) (character 19)) (end (line 60) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind featureTyping) (ordinal 0)) (authored-target "SDDD") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind redefinition) (ordinal 0)) (authored-target "services") (range (start (line 63) (character 11)) (end (line 63) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0)) (authored-target "SysLocalCloudsDesign") (range (start (line 50) (character 30)) (end (line 50) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDD") (range (start (line 53) (character 21)) (end (line 53) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDesign") (range (start (line 17) (character 17)) (end (line 17) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysLocalCloudsDD") (range (start (line 20) (character 24)) (end (line 20) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileLib::*") (range (start (line 70) (character 16)) (end (line 70) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SDMetadata") (range (start (line 92) (character 35)) (end (line 92) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 94) (character 2)) (end (line 94) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "LocalCloudsMetadata") (range (start (line 104) (character 49)) (end (line 104) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 105) (character 2)) (end (line 105) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 83) (character 45)) (end (line 83) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 84) (character 2)) (end (line 84) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SDMetadata") (range (start (line 99) (character 42)) (end (line 99) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 101) (character 2)) (end (line 101) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 73) (character 38)) (end (line 73) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 76) (character 2)) (end (line 76) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 69) (character 16)) (end (line 69) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SysDMetadata") (range (start (line 110) (character 42)) (end (line 110) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 113) (character 2)) (end (line 113) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 87) (character 39)) (end (line 87) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 88) (character 2)) (end (line 88) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 79) (character 46)) (end (line 79) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 80) (character 2)) (end (line 80) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDD") (range (start (line 108) (character 21)) (end (line 108) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (kind specialization) (ordinal 0)) (authored-target "SD") (range (start (line 0) (character 0)) (end (line 0) (character 2))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind specialization) (ordinal 0)) (authored-target "SDDD") (range (start (line 0) (character 0)) (end (line 0) (character 4))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0)) (authored-target "SysDD") (range (start (line 109) (character 23)) (end (line 109) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (target (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
