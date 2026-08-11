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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f1b7b87800c2af2385c52073d0dedc353d2be79fac07d20f16aa8e07958b79e4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint"))) (kind "package") (name "11a-View-Viewpoint") (declared-name "11a-View-Viewpoint") (range (start (line 0) (character 0)) (end (line 0) (character 1144))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (kind "package") (name "SystemModel") (declared-name "SystemModel") (range (start (line 2) (character 1)) (end (line 2) (character 611))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 2)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 17)) (end (line 3) (character 19))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (range (start (line 7) (character 2)) (end (line 7) (character 16))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 6) (character 2)) (end (line 6) (character 24))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 5) (character 2)) (end (line 5) (character 19))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 8) (character 2)) (end (line 8) (character 17))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 2)) (end (line 10) (character 472))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 10) (character 17)) (end (line 10) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 12) (character 3)) (end (line 12) (character 213))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 12) (character 28)) (end (line 12) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (range (start (line 15) (character 4)) (end (line 15) (character 87))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 15) (character 21)) (end (line 15) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 16) (character 5)) (end (line 16) (character 20))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::steeringAngle"))) (kind "attribute") (name "steeringAngle") (declared-name "steeringAngle") (range (start (line 17) (character 5)) (end (line 17) (character 29))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (range (start (line 14) (character 4)) (end (line 14) (character 31))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 14) (character 22)) (end (line 14) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 13) (character 4)) (end (line 13) (character 42))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 13) (character 22)) (end (line 13) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 11) (character 3)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 11) (character 21)) (end (line 11) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 20) (character 3)) (end (line 20) (character 180))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 20) (character 27)) (end (line 20) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 21) (character 4)) (end (line 21) (character 42))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 21) (character 22)) (end (line 21) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 23) (character 4)) (end (line 23) (character 56))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 23) (character 20)) (end (line 23) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 24) (character 5)) (end (line 24) (character 20))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 22) (character 4)) (end (line 22) (character 30))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 22) (character 21)) (end (line 22) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (kind "package") (name "ViewModel") (declared-name "ViewModel") (range (start (line 31) (character 1)) (end (line 31) (character 495))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 32) (character 2)) (end (line 32) (character 26))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 32) (character 17)) (end (line 32) (character 22))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))) (kind "concern") (name "system breakdown") (declared-name "system breakdown") (range (start (line 36) (character 2)) (end (line 36) (character 85))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown::_stakeholder_systems engineer"))) (kind "stakeholder") (name "systems engineer") (declared-name "systems engineer") (range (start (line 38) (character 3)) (end (line 38) (character 38))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (kind "view") (name "system structure generation") (declared-name "system structure generation") (range (start (line 45) (character 2)) (end (line 45) (character 238))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::**"))) (kind "import") (name "**") (declared-name "**") (range (start (line 47) (character 3)) (end (line 47) (character 54))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (authored (membership (kind Import) (import (reference "SystemModel::vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (kind "view rendering") (name "asElementTable") (declared-name "asElementTable") (range (start (line 48) (character 3)) (end (line 48) (character 97))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))) (kind "view column") (name "columnView[1]") (declared-name "columnView[1]") (range (start (line 49) (character 4)) (end (line 49) (character 65))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "columnView") (range (start (line 49) (character 13)) (end (line 49) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (kind "viewpoint") (name "system structure perspective") (declared-name "system structure perspective") (range (start (line 41) (character 2)) (end (line 41) (character 79))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective::system breakdown"))) (kind "frame") (name "system breakdown") (declared-name "system breakdown") (range (start (line 42) (character 3)) (end (line 42) (character 28))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::systems engineer"))) (kind "part") (name "systems engineer") (declared-name "systems engineer") (range (start (line 34) (character 2)) (end (line 34) (character 26))) (parent (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 3) (character 17)) (end (line 3) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 10) (character 17)) (end (line 10) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 12) (character 28)) (end (line 12) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 15) (character 21)) (end (line 15) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 14) (character 22)) (end (line 14) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 13) (character 22)) (end (line 13) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 11) (character 21)) (end (line 11) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 20) (character 27)) (end (line 20) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 21) (character 22)) (end (line 21) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 23) (character 20)) (end (line 23) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 22) (character 21)) (end (line 22) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (range (start (line 32) (character 17)) (end (line 32) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::**"))) (kind membershipImport) (ordinal 0)) (authored-target "SystemModel::vehicle::**") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))) (kind redefinition) (ordinal 0)) (authored-target "columnView") (range (start (line 49) (character 13)) (end (line 49) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
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
