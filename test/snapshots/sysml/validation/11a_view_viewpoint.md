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
  (document "11a_view_viewpoint.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 17) (end 3 19))
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
        (range (start 32 17) (end 32 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 2) (end 41 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 3) (end 47 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 13) (end 49 23))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f1b7b87800c2af2385c52073d0dedc353d2be79fac07d20f16aa8e07958b79e4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint"))) (kind "package") (name "11a-View-Viewpoint") (declared-name "11a-View-Viewpoint"))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (kind "package") (name "SystemModel") (declared-name "SystemModel") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::steeringAngle"))) (kind "attribute") (name "steeringAngle") (declared-name "steeringAngle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (kind "package") (name "ViewModel") (declared-name "ViewModel") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))) (kind "concern") (name "system breakdown") (declared-name "system breakdown") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown::_stakeholder_systems engineer"))) (kind "stakeholder") (name "systems engineer") (declared-name "systems engineer") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (kind "view") (name "system structure generation") (declared-name "system structure generation") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::**"))) (kind "import") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (authored (membership (kind Import) (import (reference "SystemModel::vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (kind "view rendering") (name "asElementTable") (declared-name "asElementTable") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))) (kind "view column") (name "columnView[1]") (declared-name "columnView[1]") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "columnView")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (kind "viewpoint") (name "system structure perspective") (declared-name "system structure perspective") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective::system breakdown"))) (kind "frame") (name "system breakdown") (declared-name "system breakdown") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::systems engineer"))) (kind "part") (name "systems engineer") (declared-name "systems engineer") (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::**"))) (kind membershipImport) (ordinal 0)) (authored-target "SystemModel::vehicle::**") (outcome (status unresolved)) (import (origin expose) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))) (kind redefinition) (ordinal 0)) (authored-target "columnView") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 17) (end 3 19)) (probe (position 3 17))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 3 17) (end 3 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 21) (end 15 25)) (probe (position 15 21))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 15 21) (end 15 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle") (range (start 7 2) (end 7 16)))
        )
      )
    )
    (query (range (start 23 20) (end 23 24)) (probe (position 23 20))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 23 20) (end 23 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle") (range (start 7 2) (end 7 16)))
        )
      )
    )
    (query (range (start 14 22) (end 14 27)) (probe (position 14 22))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 14 22) (end 14 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel") (range (start 8 2) (end 8 17)))
        )
      )
    )
    (query (range (start 22 21) (end 22 26)) (probe (position 22 21))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 22 21) (end 22 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel") (range (start 8 2) (end 8 17)))
        )
      )
    )
    (query (range (start 32 17) (end 32 22)) (probe (position 32 17))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Views::*")
        (range (start 32 17) (end 32 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 17) (end 10 24)) (probe (position 10 17))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 10 17) (end 10 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle") (range (start 5 2) (end 5 19)))
        )
      )
    )
    (query (range (start 11 21) (end 11 30)) (probe (position 11 21))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 11 21) (end 11 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 22) (end 13 31)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 13 22) (end 13 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 22) (end 21 31)) (probe (position 21 22))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 21 22) (end 21 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 49 13) (end 49 23)) (probe (position 49 13))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))
        (kind redefinition) (ordinal 0) (authored-target "columnView")
        (range (start 49 13) (end 49 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 28) (end 12 40)) (probe (position 12 28))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 12 28) (end 12 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly") (range (start 6 2) (end 6 24)))
        )
      )
    )
    (query (range (start 20 27) (end 20 39)) (probe (position 20 27))
      (reference
        (source (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 20 27) (end 20 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly") (range (start 6 2) (end 6 24)))
        )
      )
    )
  )
)
~~~
