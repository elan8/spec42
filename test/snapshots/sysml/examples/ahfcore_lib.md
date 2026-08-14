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
        (range (start 6 1) (end 6 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 1) (end 11 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 14 10))
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
        (range (start 18 1) (end 18 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 22 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 22 10) (end 54 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:53f8a257c9163ad449d72147501e6727bb87e96ee77f55df942e8e4a8f1a00a8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileLib") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Authorisation"))))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ServiceDiscovery"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD")))
      (supertype (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")) (scopes any subclassification))
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
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 4 16) (end 4 37)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (path (named (kind library-package) (name "AHFCoreLib")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 15 22) (end 15 28)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 18 38) (end 18 51)) (probe (position 18 38))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0) (authored-target "Authorisation")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::Authorisation")))))
  )
  (query (document "memory://snapshot/ahfcore_lib.md") (range (start 11 41) (end 11 57)) (probe (position 11 41))
    (reference (id (source (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0) (authored-target "ServiceDiscovery")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfcore_lib.md") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
  )
)
~~~
