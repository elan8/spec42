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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "57d523f94d366d881bc96fc390bacc5805a61ea440f0ed5d222a5d8726f62fb1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AHFProfileLib"))) (kind "package") (name "AHFProfileLib") (declared-name "AHFProfileLib"))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (kind "port def") (name "IDD") (declared-name "IDD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind "attribute") (name "encoding_kind") (declared-name "encoding_kind") (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::IDD::~IDD"))) (kind "conjugated port definition") (name "~IDD") (declared-name "~IDD") (parent (node (document "d0") (qualified-name "AHFProfileLib::IDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind "part def") (name "LocalCloudDD") (declared-name "LocalCloudDD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LocalCloudDesign")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (kind "part") (name "systems") (declared-name "systems") (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (kind "part def") (name "LocalCloudDesign") (declared-name "LocalCloudDesign") (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind "part") (name "systems") (declared-name "systems") (parent (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (kind "port def") (name "SD") (declared-name "SD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind "attribute") (name "intrfce_protocol") (declared-name "intrfce_protocol") (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind "attribute") (name "serviceDefinition") (declared-name "serviceDefinition") (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind "attribute") (name "serviceURL") (declared-name "serviceURL") (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SD::~SD"))) (kind "conjugated port definition") (name "~SD") (declared-name "~SD") (parent (node (document "d0") (qualified-name "AHFProfileLib::SD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (kind "port def") (name "SDDD") (declared-name "SDDD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind "port") (name "idds") (declared-name "idds") (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "IDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SDDD::~SDDD"))) (kind "conjugated port definition") (name "~SDDD") (declared-name "~SDDD") (parent (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (kind "part def") (name "SysD") (declared-name "SysD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind "attribute") (name "address") (declared-name "address") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind "attribute") (name "portno") (declared-name "portno") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (kind "port") (name "services") (declared-name "services") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind "attribute") (name "systemname") (declared-name "systemname") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysD"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (kind "part def") (name "SysDD") (declared-name "SysDD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::ServiceMethod"))) (kind "action") (name "ServiceMethod") (declared-name "ServiceMethod") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind "port") (name "services") (declared-name "services") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "SDDD")) (redefinition (reference "services")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind "part def") (name "SysLocalCloudsDD") (declared-name "SysLocalCloudsDD") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysLocalCloudsDesign")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (kind "part") (name "locclouds") (declared-name "locclouds") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (kind "part def") (name "SysLocalCloudsDesign") (declared-name "SysLocalCloudsDesign") (parent (node (document "d0") (qualified-name "AHFProfileLib"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind "part") (name "locclouds") (declared-name "locclouds") (parent (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDesign")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (kind "part") (name "system_of_systems") (declared-name "system_of_systems") (parent (node (document "d0") (qualified-name "AHFProfileLib"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysLocalCloudsDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata"))) (kind "package") (name "AHFProfileMetadata") (declared-name "AHFProfileMetadata"))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileLib::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind "metadata def") (name "IDDMetadata") (declared-name "IDDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind "metadata def") (name "LocalCloudsDDMetadata") (declared-name "LocalCloudsDDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LocalCloudsMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind "metadata def") (name "LocalCloudsMetadata") (declared-name "LocalCloudsMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind "metadata def") (name "SDDDMetadata") (declared-name "SDDDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind "metadata def") (name "SDMetadata") (declared-name "SDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind "metadata def") (name "SysDDMetadata") (declared-name "SysDDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SysDMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind "metadata def") (name "SysDMetadata") (declared-name "SysDMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind "metadata def") (name "SysLocalCloudsMetadata") (declared-name "SysLocalCloudsMetadata") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind "part") (name "global_clouddd") (declared-name "global_clouddd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "LocalCloudDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (kind "port def") (name "global_sd") (declared-name "global_sd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd::~global_sd"))) (kind "conjugated port definition") (name "~global_sd") (declared-name "~global_sd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind "port def") (name "global_sddd") (declared-name "global_sddd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SDDD")))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd::~global_sddd"))) (kind "conjugated port definition") (name "~global_sddd") (declared-name "~global_sddd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))))
    (element (id (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind "part") (name "global_systemsdd") (declared-name "global_systemsdd") (parent (node (document "d0") (qualified-name "AHFProfileMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "SysDD")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0)) (authored-target "SD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0)) (authored-target "LocalCloudDesign") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0)) (authored-target "SD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0)) (authored-target "IDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::IDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0)) (authored-target "SD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0)) (authored-target "SysD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind featureTyping) (ordinal 0)) (authored-target "SDDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))) (kind redefinition) (ordinal 0)) (authored-target "services") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD::services")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0)) (authored-target "SysLocalCloudsDesign") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDesign") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0)) (authored-target "SysLocalCloudsDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileLib::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SDMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "LocalCloudsMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SDMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SysDMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0)) (authored-target "LocalCloudDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))) (kind specialization) (ordinal 0)) (authored-target "SD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind specialization) (ordinal 0)) (authored-target "SDDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0)) (authored-target "SysDD") (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFProfileLib::SysDD")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 0) (end 0 2)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))
        (kind specialization) (ordinal 0) (authored-target "SD")
        (range (start 0 0) (end 0 2))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SD") (range (start 5 1) (end 5 197)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))
        (kind specialization) (ordinal 0) (authored-target "SDDD")
        (range (start 0 0) (end 0 4))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SDDD") (range (start 44 1) (end 44 134)))
        )
      )
    )
    (query (range (start 38 17) (end 38 19)) (probe (position 38 17))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::IDD"))
        (kind specialization) (ordinal 0) (authored-target "SD")
        (range (start 38 17) (end 38 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SD") (range (start 5 1) (end 5 197)))
        )
      )
    )
    (query (range (start 44 18) (end 44 20)) (probe (position 44 18))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SDDD"))
        (kind specialization) (ordinal 0) (authored-target "SD")
        (range (start 44 18) (end 44 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SD") (range (start 5 1) (end 5 197)))
        )
      )
    )
    (query (range (start 0 0) (end 0 4)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_sd"))
        (kind specialization) (ordinal 0) (authored-target "SD")
        (range (start 0 0) (end 0 2))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SD") (range (start 5 1) (end 5 197)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_sddd"))
        (kind specialization) (ordinal 0) (authored-target "SDDD")
        (range (start 0 0) (end 0 4))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SDDD") (range (start 44 1) (end 44 134)))
        )
      )
    )
    (query (range (start 25 15) (end 25 19)) (probe (position 25 15))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))
        (kind featureTyping) (ordinal 0) (authored-target "SysD")
        (range (start 25 15) (end 25 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysD") (range (start 28 1) (end 28 169)))
        )
      )
    )
    (query (range (start 60 19) (end 60 23)) (probe (position 60 19))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysDD"))
        (kind specialization) (ordinal 0) (authored-target "SysD")
        (range (start 60 19) (end 60 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysD") (range (start 28 1) (end 28 169)))
        )
      )
    )
    (query (range (start 57 19) (end 57 24)) (probe (position 57 19))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD::systems"))
        (kind featureTyping) (ordinal 0) (authored-target "SysDD")
        (range (start 57 19) (end 57 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysDD") (range (start 60 1) (end 60 150)))
        )
      )
    )
    (query (range (start 109 23) (end 109 28)) (probe (position 109 23))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_systemsdd"))
        (kind featureTyping) (ordinal 0) (authored-target "SysDD")
        (range (start 109 23) (end 109 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysDD") (range (start 60 1) (end 60 150)))
        )
      )
    )
    (query (range (start 8 30) (end 8 36)) (probe (position 8 30))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SD::serviceDefinition"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 8 30) (end 8 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 23) (end 9 29)) (probe (position 9 23))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SD::serviceURL"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 9 23) (end 9 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 29) (end 10 35)) (probe (position 10 29))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 10 29) (end 10 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 24) (end 32 30)) (probe (position 32 24))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysD::systemname"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 32 24) (end 32 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 21) (end 33 27)) (probe (position 33 21))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysD::address"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 33 21) (end 33 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 26) (end 41 32)) (probe (position 41 26))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::IDD::encoding_kind"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 41 26) (end 41 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 20) (end 34 27)) (probe (position 34 20))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysD::portno"))
        (kind featureTyping) (ordinal 1) (authored-target "Integer")
        (range (start 34 20) (end 34 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 63 11) (end 63 19)) (probe (position 63 11))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysDD::services"))
        (kind redefinition) (ordinal 0) (authored-target "services")
        (range (start 63 11) (end 63 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysDD::services") (range (start 63 2) (end 63 25)))
        )
      )
    )
    (query (range (start 92 35) (end 92 45)) (probe (position 92 35))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SDMetadata")
        (range (start 92 35) (end 92 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata") (range (start 73 1) (end 73 260)))
        )
      )
    )
    (query (range (start 99 42) (end 99 52)) (probe (position 99 42))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SDMetadata")
        (range (start 99 42) (end 99 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata") (range (start 73 1) (end 73 260)))
        )
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 21) (end 53 33)) (probe (position 53 21))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD::locclouds"))
        (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDD")
        (range (start 53 21) (end 53 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD") (range (start 56 1) (end 56 79)))
        )
      )
    )
    (query (range (start 76 2) (end 76 14)) (probe (position 76 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 76 2) (end 76 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata::baseType") (range (start 76 2) (end 76 55)))
        )
      )
    )
    (query (range (start 80 2) (end 80 14)) (probe (position 80 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 80 2) (end 80 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType") (range (start 80 2) (end 80 57)))
        )
      )
    )
    (query (range (start 84 2) (end 84 14)) (probe (position 84 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 84 2) (end 84 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType") (range (start 84 2) (end 84 74)))
        )
      )
    )
    (query (range (start 88 2) (end 88 14)) (probe (position 88 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 88 2) (end 88 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType") (range (start 88 2) (end 88 83)))
        )
      )
    )
    (query (range (start 94 2) (end 94 14)) (probe (position 94 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 94 2) (end 94 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType") (range (start 94 2) (end 94 50)))
        )
      )
    )
    (query (range (start 101 2) (end 101 14)) (probe (position 101 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 101 2) (end 101 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType") (range (start 101 2) (end 101 51)))
        )
      )
    )
    (query (range (start 105 2) (end 105 14)) (probe (position 105 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 105 2) (end 105 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType") (range (start 105 2) (end 105 68)))
        )
      )
    )
    (query (range (start 108 21) (end 108 33)) (probe (position 108 21))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::global_clouddd"))
        (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDD")
        (range (start 108 21) (end 108 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD") (range (start 56 1) (end 56 79)))
        )
      )
    )
    (query (range (start 110 42) (end 110 54)) (probe (position 110 42))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SysDMetadata")
        (range (start 110 42) (end 110 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata") (range (start 87 1) (end 87 218)))
        )
      )
    )
    (query (range (start 113 2) (end 113 14)) (probe (position 113 2))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 113 2) (end 113 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType") (range (start 113 2) (end 113 56)))
        )
      )
    )
    (query (range (start 70 16) (end 70 29)) (probe (position 70 16))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib::*")
        (range (start 70 16) (end 70 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib") (range (start 0 0) (end 0 1517)))
        )
      )
    )
    (query (range (start 17 17) (end 17 33)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))
        (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDesign")
        (range (start 17 17) (end 17 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign") (range (start 22 1) (end 22 95)))
        )
      )
    )
    (query (range (start 20 24) (end 20 40)) (probe (position 20 24))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::system_of_systems"))
        (kind featureTyping) (ordinal 0) (authored-target "SysLocalCloudsDD")
        (range (start 20 24) (end 20 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD") (range (start 50 1) (end 50 169)))
        )
      )
    )
    (query (range (start 56 26) (end 56 42)) (probe (position 56 26))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::LocalCloudDD"))
        (kind specialization) (ordinal 0) (authored-target "LocalCloudDesign")
        (range (start 56 26) (end 56 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::LocalCloudDesign") (range (start 22 1) (end 22 95)))
        )
      )
    )
    (query (range (start 73 38) (end 73 54)) (probe (position 73 38))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 73 38) (end 73 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata") (range (start 69 1) (end 69 46)))
        )
      )
    )
    (query (range (start 79 46) (end 79 62)) (probe (position 79 46))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 79 46) (end 79 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata") (range (start 69 1) (end 69 46)))
        )
      )
    )
    (query (range (start 83 45) (end 83 61)) (probe (position 83 45))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 83 45) (end 83 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata") (range (start 69 1) (end 69 46)))
        )
      )
    )
    (query (range (start 87 39) (end 87 55)) (probe (position 87 39))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SysDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 87 39) (end 87 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata") (range (start 69 1) (end 69 46)))
        )
      )
    )
    (query (range (start 104 49) (end 104 68)) (probe (position 104 49))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))
        (kind specialization) (ordinal 0) (authored-target "LocalCloudsMetadata")
        (range (start 104 49) (end 104 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata") (range (start 83 1) (end 83 140)))
        )
      )
    )
    (query (range (start 50 30) (end 50 50)) (probe (position 50 30))
      (reference
        (source (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))
        (kind specialization) (ordinal 0) (authored-target "SysLocalCloudsDesign")
        (range (start 50 30) (end 50 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AHFProfileLib::SysLocalCloudsDesign") (range (start 13 1) (end 13 148)))
        )
      )
    )
    (query (range (start 69 16) (end 69 45)) (probe (position 69 16))
      (reference
        (source (document "d0") (qualified-name "AHFProfileMetadata::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 69 16) (end 69 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
