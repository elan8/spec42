# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFCoreLib
type=file
~~~
# SOURCE
~~~sysml
// /** Mandatory Services and Systems */
library package AHFCoreLib {
	private import AHFProfileLib::*;
	private import ScalarValues::*;
	private import AHFProfileMetadata::*;

	#service port def ServiceDiscovery {
		// The functionalities as Requests (Operations) cannot be defined yet
		// We could consider using flows to designate the functionalities
	}
	
	#service port def ServiceDiscoveryDD :> ServiceDiscovery{
	}
		
	#service port def Authorisation {
		attribute publickey:String; // just as examples
	}

	#service port def AuthorisationDD :> Authorisation{
	}

	
	#clouddd ArrowheadCore{
		// /** Design Level */
		// First the system definitions (SysD) of core systems
		
		#system service_registry {
			#service serviceDiscovery : ServiceDiscovery ;
		}
		
		#system authorization{
			#service authorisation : Authorisation;
			attribute protocol:String = "HTTP";
		}
		
		#system orchestrationDesign; // just indicated for now
		
		// /** Design Description level */		
		#systemdd service_registry_DD :> service_registry{
			#servicedd :>> serviceDiscovery:ServiceDiscoveryDD {
				#idd serviceDiscovery_HTTP ;// nested port for HTTP protocol
				// here we refer the functionalities like operation Register etc.
				#idd serviceDiscovery_MQTT ; // nested port for MQTT protocol
			}
		}
		
		#systemdd authorization_DD :> authorization{
			#servicedd :>> authorisation {
				#idd authorisation_HTTP ; // nested port for HTTP protocol
				#idd authorisation_MQTT ; // nested port for MQTT protocol
			}
			action Echo_behavior :> ServiceMethod;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/ahfcore_lib.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 2 16) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 1) (end 6 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 1) (end 11 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 14 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 22) (end 15 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 1) (end 18 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 3) (end 27 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 3) (end 31 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 22) (end 32 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 39 3) (end 44 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 47 3) (end 51 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 27) (end 51 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:53f8a257c9163ad449d72147501e6727bb87e96ee77f55df942e8e4a8f1a00a8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileLib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileMetadata") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Authorisation")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::protocol"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD"))) (kind extended-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "authorization")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD::Echo_behavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ServiceMethod")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::orchestrationDesign"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ServiceDiscovery")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD"))) (kind extended-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "service_registry")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Authorisation")))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ServiceDiscovery")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Authorisation")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::protocol"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD"))) (kind specialization) (ordinal 0))
      (authored-target "authorization")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")))))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD::Echo_behavior"))) (kind subsetting) (ordinal 0))
      (authored-target "ServiceMethod")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery"))) (kind featureTyping) (ordinal 0))
      (authored-target "ServiceDiscovery")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD"))) (kind specialization) (ordinal 0))
      (authored-target "service_registry")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry")))))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0))
      (authored-target "Authorisation")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0))
      (authored-target "ServiceDiscovery")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::protocol"))) (state literal) (value (kind string) (value "HTTP")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore")))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")))
      (type (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")) (source direct))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::protocol")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore")))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD::Echo_behavior")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD")))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::orchestrationDesign")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore")))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore")))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry")))
      (type (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")) (source direct))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore")))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation")) (scopes any))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey")))
      (featured-by (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD")))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery")) (scopes any))
      (subtype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD")))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 2 16) (end 2 32)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 4 16) (end 4 37)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 31 28) (end 31 41)) (probe (position 31 28))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::authorisation"))) (kind featureTyping) (ordinal 0) (authored-target "Authorisation")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 32 22) (end 32 28)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization::protocol"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 46 32) (end 46 45)) (probe (position 46 32))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD"))) (kind specialization) (ordinal 0) (authored-target "authorization")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization")))))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 51 27) (end 51 40)) (probe (position 51 27))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::authorization_DD::Echo_behavior"))) (kind subsetting) (ordinal 0) (authored-target "ServiceMethod")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 27 31) (end 27 47)) (probe (position 27 31))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery"))) (kind featureTyping) (ordinal 0) (authored-target "ServiceDiscovery")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 38 35) (end 38 51)) (probe (position 38 35))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry_DD"))) (kind specialization) (ordinal 0) (authored-target "service_registry")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ArrowheadCore::service_registry")))))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 15 22) (end 15 28)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 18 38) (end 18 51)) (probe (position 18 38))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0) (authored-target "Authorisation")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))))
    )
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 11 41) (end 11 57)) (probe (position 11 41))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0) (authored-target "ServiceDiscovery")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
    )
  )
)
~~~
