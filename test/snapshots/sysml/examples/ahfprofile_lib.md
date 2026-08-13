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
  (document "memory://snapshot/ahfprofile_lib.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
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
        (range (start 9 23) (end 9 29))
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
        (range (start 32 24) (end 32 30))
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
        (range (start 34 20) (end 34 27))
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 73 38) (end 73 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 46) (end 79 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 45) (end 83 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 87 39) (end 87 55))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8c97661a610c2aa79a737de5c335ef2bd9885bc2ea527f1866a4d469d33e28eb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LocalCloudDesign"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysDD")) (redefinition (reference "systems"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IDD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::address"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::portno"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SysD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SDDD")) (redefinition (reference "services"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD::ServiceMethod"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SysLocalCloudsDesign"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LocalCloudDD")) (redefinition (reference "locclouds"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LocalCloudDesign"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::system_of_systems"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysLocalCloudsDD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileLib") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SDMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LocalCloudsMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SDMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SysDMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LocalCloudDD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sd"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SDDD"))))
    (declaration (id (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysDD"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0))
      (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0))
      (authored-target "LocalCloudDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "systems")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0))
      (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0))
      (authored-target "IDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0))
      (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0))
      (authored-target "SysD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SDDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "services")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0))
      (authored-target "SysLocalCloudsDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "LocalCloudDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "locclouds")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0))
      (authored-target "LocalCloudDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysLocalCloudsDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileLib")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "LocalCloudsMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SysDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0))
      (authored-target "LocalCloudDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sd"))) (kind featureTyping) (ordinal 0))
      (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind featureTyping) (ordinal 0))
      (authored-target "SDDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD")))))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD::idds"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::system_of_systems"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_clouddd"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sd"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sddd"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 38 17) (end 38 19)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD"))) (kind specialization) (ordinal 0) (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 41 26) (end 41 32)) (probe (position 41 26))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD::encoding_kind"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 56 26) (end 56 42)) (probe (position 56 26))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD"))) (kind specialization) (ordinal 0) (authored-target "LocalCloudDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 57 19) (end 57 24)) (probe (position 57 19))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 57 11) (end 57 18)) (probe (position 57 11))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "systems")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 25 15) (end 25 19)) (probe (position 25 15))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign::systems"))) (kind featureTyping) (ordinal 0) (authored-target "SysD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 10 29) (end 10 35)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::intrfce_protocol"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 8 30) (end 8 36)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceDefinition"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 9 23) (end 9 29)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD::serviceURL"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 44 18) (end 44 20)) (probe (position 44 18))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD"))) (kind specialization) (ordinal 0) (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 47 12) (end 47 15)) (probe (position 47 12))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD::idds"))) (kind featureTyping) (ordinal 0) (authored-target "IDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::IDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 33 21) (end 33 27)) (probe (position 33 21))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::address"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 34 20) (end 34 27)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::portno"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 31 17) (end 31 19)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services"))) (kind featureTyping) (ordinal 0) (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 32 24) (end 32 30)) (probe (position 32 24))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::systemname"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 60 19) (end 60 23)) (probe (position 60 19))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD"))) (kind specialization) (ordinal 0) (authored-target "SysD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 63 20) (end 63 24)) (probe (position 63 20))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SDDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 63 11) (end 63 19)) (probe (position 63 11))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "services")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysD::services")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 50 30) (end 50 50)) (probe (position 50 30))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD"))) (kind specialization) (ordinal 0) (authored-target "SysLocalCloudsDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 53 21) (end 53 33)) (probe (position 53 21))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 53 11) (end 53 20)) (probe (position 53 11))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "locclouds")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 17 17) (end 17 33)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDesign::locclouds"))) (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDesign")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDesign")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 20 24) (end 20 40)) (probe (position 20 24))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::system_of_systems"))) (kind featureTyping) (ordinal 0) (authored-target "SysLocalCloudsDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysLocalCloudsDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 70 16) (end 70 32)) (probe (position 70 16))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 69 16) (end 69 45)) (probe (position 69 16))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 92 35) (end 92 45)) (probe (position 92 35))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata"))) (kind specialization) (ordinal 0) (authored-target "SDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 94 6) (end 94 14)) (probe (position 94 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::IDDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 104 49) (end 104 68)) (probe (position 104 49))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata"))) (kind specialization) (ordinal 0) (authored-target "LocalCloudsMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 105 6) (end 105 14)) (probe (position 105 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsDDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 83 45) (end 83 61)) (probe (position 83 45))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 84 6) (end 84 14)) (probe (position 84 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::LocalCloudsMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 99 42) (end 99 52)) (probe (position 99 42))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata"))) (kind specialization) (ordinal 0) (authored-target "SDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 101 6) (end 101 14)) (probe (position 101 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDDDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 73 38) (end 73 54)) (probe (position 73 38))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 76 6) (end 76 14)) (probe (position 76 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 110 42) (end 110 54)) (probe (position 110 42))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata"))) (kind specialization) (ordinal 0) (authored-target "SysDMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 113 6) (end 113 14)) (probe (position 113 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 87 39) (end 87 55)) (probe (position 87 39))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 88 6) (end 88 14)) (probe (position 88 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysDMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 79 46) (end 79 62)) (probe (position 79 46))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 80 6) (end 80 14)) (probe (position 80 6))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::SysLocalCloudsMetadata::baseType")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 108 21) (end 108 33)) (probe (position 108 21))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_clouddd"))) (kind featureTyping) (ordinal 0) (authored-target "LocalCloudDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::LocalCloudDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 72 16) (end 72 18)) (probe (position 72 16))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sd"))) (kind featureTyping) (ordinal 0) (authored-target "SD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 98 18) (end 98 22)) (probe (position 98 18))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_sddd"))) (kind featureTyping) (ordinal 0) (authored-target "SDDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SDDD")))))
  )
  (query (document "memory://snapshot/ahfprofile_lib.md") (range (start 109 23) (end 109 28)) (probe (position 109 23))
    (reference (id (source (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileMetadata::global_systemsdd"))) (kind featureTyping) (ordinal 0) (authored-target "SysDD")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfprofile_lib.md") (qualified-name "AHFProfileLib::SysDD")))))
  )
)
~~~
