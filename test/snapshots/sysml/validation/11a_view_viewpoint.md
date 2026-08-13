# META
~~~ini
description=SysML Validation (11-View and Viewpoint): 11a-View-Viewpoint
type=file
~~~
# SOURCE
~~~sysml
package '11a-View-Viewpoint' {
	
	package SystemModel {
		private import SI::*;
		
		part def Vehicle;
		part def AxleAssembly;
		part def Axle;
		part def Wheel;
		
		part vehicle : Vehicle {
			attribute mass :> ISQ::mass = 2500[SI::kg];
			part frontAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 150[kg];
				part frontWheel : Wheel[2];
				part frontAxle : Axle[1] {
					attribute mass;
					attribute steeringAngle;
				}
			}
			part rearAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 250[kg];
				part rearWheel : Wheel[2];
				part rearAxle : Axle[1] {
					attribute mass;
				}
			}
		}
		
	}
	
	package ViewModel {
		private import Views::*;
	
		part 'systems engineer';
		
		concern 'system breakdown' {
			subject;
			stakeholder :>> 'systems engineer';
		}
		
		viewpoint 'system structure perspective' {		
			frame 'system breakdown';
		}
		
		view 'system structure generation' {
			satisfy 'system structure perspective';
			expose SystemModel::vehicle::**[@SysML::PartUsage];
			render asElementTable {
				view :>> columnView[1] {
					render asTextualNotation;
				}
			}
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/11a_view_viewpoint.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 17) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 21) (end 11 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 22) (end 13 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 22) (end 21 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 17) (end 32 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 37 3) (end 37 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 38 3) (end 38 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 2) (end 43 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 46 3) (end 46 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 47 3) (end 47 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 48 3) (end 52 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:71e7053996960d8442c98420d9efb3db30302b03e498769531a06e20a1f9e94b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::steeringAngle"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::ViewModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Views") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))) (kind concern) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::ViewModel::systems engineer"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Views")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 3 17) (end 3 22)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 10 17) (end 10 24)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 12 28) (end 12 40)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 15 21) (end 15 25)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 14 22) (end 14 27)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 13 22) (end 13 31)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 20 27) (end 20 39)) (probe (position 20 27))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 21 22) (end 21 31)) (probe (position 21 22))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 23 20) (end 23 24)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 22 21) (end 22 26)) (probe (position 22 21))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/11a_view_viewpoint.md") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
  )
  (query (document "memory://snapshot/11a_view_viewpoint.md") (range (start 32 17) (end 32 25)) (probe (position 32 17))
    (reference (id (source (node (document "memory://snapshot/11a_view_viewpoint.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Views")
      (outcome (status unresolved)))
  )
)
~~~
