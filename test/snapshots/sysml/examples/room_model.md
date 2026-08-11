# META
~~~ini
description=SysML Example (Room Model): RoomModel
type=file
~~~
# SOURCE
~~~sysml
// SysML v2 Interpretation of the SysML v1 Room Connection Example
package RoomModel { 
    package RoomDefinitionModelLibrary{
        private import Port_Definitions::*;
        private import Flow_Definitions::*;
        package Part_Definitions{
            // Rooms
            part def Classroom {
                port classEntry: EntryWay_to_Classroom;
            }
            part def Storageroom {
                port storageEntry: EntryWay_to_Storageroom;
            }
            part def Hallway {
                // conjugate ports with ~
                port hallExit_to_Classroom: ~EntryWay_to_Classroom;
                port hallExit_to_Storageroom: ~EntryWay_to_Storageroom;
            }
        }
        package Port_Definitions{
            port def EntryWay_to_Classroom {
                //flow properties
                in ref student:Student;
                in ref teacher:Teacher;
                in ref furniture:Furniture;
                in ref air:Air;
            }
            port def EntryWay_to_Storageroom {
                //flow properties
                in ref furniture:  Furniture;
                in ref air: Air;
            }
        }
        package Flow_Definitions {
                // Conveyed items between Hallway, Classroom, and Storageroom
                part def Air;
                part def Furniture;
                part def Student;
                part def Teacher;
        }
    }
    package Room_Configuration{
        // defining the parts and their interconnection in context 
        private import RoomDefinitionModelLibrary::*;
        private import RoomDefinitionModelLibrary::Part_Definitions::*;
        private import RoomDefinitionModelLibrary::Port_Definitions::*;
        private import RoomDefinitionModelLibrary::Flow_Definitions::*;
        part roomContext{
            part c:Classroom;
            part s:Storageroom;
            part h:Hallway;
            
    		//  Connectors and item flows between hallway and classroom
            flow HallToClassroom_Air
                from h.hallExit_to_Classroom.air
                to c.classEntry.air;
            flow HallToClassroom_Furniture
                from h.hallExit_to_Classroom.furniture
                to c.classEntry.furniture;
            flow HallToClassroom_Student
                from h.hallExit_to_Classroom.student
                to c.classEntry.student;
            flow HallToClassroom_Teacher
                from h.hallExit_to_Classroom.teacher
                to c.classEntry.teacher;
            flow HallToStorageroom_Air
                from h.hallExit_to_Storageroom.air
                to s.storageEntry.air;
            flow HallToStorageroom_Furniture
                from h.hallExit_to_Storageroom.furniture
                to s.storageEntry.furniture;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "room_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 16) (end 22 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 16) (end 23 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 16) (end 24 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 16) (end 25 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 16) (end 29 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 16) (end 30 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 43 23) (end 43 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 44 23) (end 44 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 45 23) (end 45 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 23) (end 46 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 19) (end 48 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 19) (end 49 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 19) (end 50 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 21) (end 54 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 19) (end 55 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 21) (end 57 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 19) (end 58 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 21) (end 60 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 19) (end 61 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 21) (end 63 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 19) (end 64 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 21) (end 66 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 67 19) (end 67 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 21) (end 69 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 19) (end 70 43))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4c64371a767360793a21a6acbbca6c8c330e508d61b48450b1b3d2da32ab08eb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RoomModel"))) (kind "package") (name "RoomModel") (declared-name "RoomModel"))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (kind "package") (name "RoomDefinitionModelLibrary") (declared-name "RoomDefinitionModelLibrary") (parent (node (document "d0") (qualified-name "RoomModel"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port_Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (authored (membership (kind Import) (visibility "private") (import (reference "Flow_Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))) (kind "package") (name "Flow_Definitions") (declared-name "Flow_Definitions") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air"))) (kind "part def") (name "Air") (declared-name "Air") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture"))) (kind "part def") (name "Furniture") (declared-name "Furniture") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Student"))) (kind "part def") (name "Student") (declared-name "Student") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Teacher"))) (kind "part def") (name "Teacher") (declared-name "Teacher") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))) (kind "package") (name "Part_Definitions") (declared-name "Part_Definitions") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (kind "part def") (name "Classroom") (declared-name "Classroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (kind "port") (name "classEntry") (declared-name "classEntry") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (authored (membership (kind Feature)) (relationships (typing (reference "EntryWay_to_Classroom")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (kind "part def") (name "Hallway") (declared-name "Hallway") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (kind "port") (name "hallExit_to_Classroom") (declared-name "hallExit_to_Classroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (authored (membership (kind Feature)) (relationships (typing (reference "~EntryWay_to_Classroom")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (kind "port") (name "hallExit_to_Storageroom") (declared-name "hallExit_to_Storageroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (authored (membership (kind Feature)) (relationships (typing (reference "~EntryWay_to_Storageroom")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (kind "part def") (name "Storageroom") (declared-name "Storageroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (kind "port") (name "storageEntry") (declared-name "storageEntry") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (authored (membership (kind Feature)) (relationships (typing (reference "EntryWay_to_Storageroom")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))) (kind "package") (name "Port_Definitions") (declared-name "Port_Definitions") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (kind "port def") (name "EntryWay_to_Classroom") (declared-name "EntryWay_to_Classroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref student:Student")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref teacher:Teacher")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter2"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref furniture:Furniture")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter3"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref air:Air")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::~EntryWay_to_Classroom"))) (kind "conjugated port definition") (name "~EntryWay_to_Classroom") (declared-name "~EntryWay_to_Classroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (kind "port def") (name "EntryWay_to_Storageroom") (declared-name "EntryWay_to_Storageroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (authored (relationships (typing (reference "ref furniture:  Furniture")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (authored (relationships (typing (reference "ref air: Air")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::~EntryWay_to_Storageroom"))) (kind "conjugated port definition") (name "~EntryWay_to_Storageroom") (declared-name "~EntryWay_to_Storageroom") (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (kind "package") (name "Room_Configuration") (declared-name "Room_Configuration") (parent (node (document "d0") (qualified-name "RoomModel"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Part_Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Port_Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Flow_Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind "part") (name "roomContext") (declared-name "roomContext") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Air"))) (kind "flow") (name "HallToClassroom_Air") (declared-name "HallToClassroom_Air") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Furniture"))) (kind "flow") (name "HallToClassroom_Furniture") (declared-name "HallToClassroom_Furniture") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Student"))) (kind "flow") (name "HallToClassroom_Student") (declared-name "HallToClassroom_Student") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Teacher"))) (kind "flow") (name "HallToClassroom_Teacher") (declared-name "HallToClassroom_Teacher") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Air"))) (kind "flow") (name "HallToStorageroom_Air") (declared-name "HallToStorageroom_Air") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Furniture"))) (kind "flow") (name "HallToStorageroom_Furniture") (declared-name "HallToStorageroom_Furniture") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind "part") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Classroom")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind "part") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hallway")))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind "part") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Storageroom")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port_Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Flow_Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (kind featureTyping) (ordinal 0)) (authored-target "EntryWay_to_Classroom") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (kind featureTyping) (ordinal 0)) (authored-target "~EntryWay_to_Classroom") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (kind featureTyping) (ordinal 0)) (authored-target "~EntryWay_to_Storageroom") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (kind featureTyping) (ordinal 0)) (authored-target "EntryWay_to_Storageroom") (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref student:Student") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref teacher:Teacher") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter2"))) (kind featureTyping) (ordinal 0)) (authored-target "ref furniture:Furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter3"))) (kind featureTyping) (ordinal 0)) (authored-target "ref air:Air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref furniture:  Furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref air: Air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Part_Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Port_Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Flow_Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 0)) (authored-target "h::hallExit_to_Classroom::air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 1)) (authored-target "h::hallExit_to_Classroom::furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 2)) (authored-target "h::hallExit_to_Classroom::student") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 3)) (authored-target "h::hallExit_to_Classroom::teacher") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 4)) (authored-target "h::hallExit_to_Storageroom::air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 5)) (authored-target "h::hallExit_to_Storageroom::furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 0)) (authored-target "c::classEntry::air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 1)) (authored-target "c::classEntry::furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 2)) (authored-target "c::classEntry::student") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 3)) (authored-target "c::classEntry::teacher") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 4)) (authored-target "s::storageEntry::air") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 5)) (authored-target "s::storageEntry::furniture") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind featureTyping) (ordinal 0)) (authored-target "Classroom") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind featureTyping) (ordinal 0)) (authored-target "Hallway") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind featureTyping) (ordinal 0)) (authored-target "Storageroom") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 50 19) (end 50 26)) (probe (position 50 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))
        (kind featureTyping) (ordinal 0) (authored-target "Hallway")
        (range (start 50 19) (end 50 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 19) (end 48 28)) (probe (position 48 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))
        (kind featureTyping) (ordinal 0) (authored-target "Classroom")
        (range (start 48 19) (end 48 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 49 19) (end 49 30)) (probe (position 49 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))
        (kind featureTyping) (ordinal 0) (authored-target "Storageroom")
        (range (start 49 19) (end 49 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 23) (end 3 39)) (probe (position 3 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Port_Definitions::*")
        (range (start 3 23) (end 3 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions") (range (start 19 8) (end 19 466)))
        )
      )
    )
    (query (range (start 4 23) (end 4 39)) (probe (position 4 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Flow_Definitions::*")
        (range (start 4 23) (end 4 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions") (range (start 33 8) (end 33 256)))
        )
      )
    )
    (query (range (start 55 19) (end 55 35)) (probe (position 55 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 0) (authored-target "c::classEntry::air")
        (range (start 55 19) (end 55 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 19) (end 67 37)) (probe (position 67 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 4) (authored-target "s::storageEntry::air")
        (range (start 67 19) (end 67 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 19) (end 61 39)) (probe (position 61 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 2) (authored-target "c::classEntry::student")
        (range (start 61 19) (end 61 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 19) (end 64 39)) (probe (position 64 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 3) (authored-target "c::classEntry::teacher")
        (range (start 64 19) (end 64 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 19) (end 58 41)) (probe (position 58 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 1) (authored-target "c::classEntry::furniture")
        (range (start 58 19) (end 58 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 70 19) (end 70 43)) (probe (position 70 19))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowTarget) (ordinal 5) (authored-target "s::storageEntry::furniture")
        (range (start 70 19) (end 70 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 23) (end 43 49)) (probe (position 43 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::*")
        (range (start 43 23) (end 43 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 21) (end 54 48)) (probe (position 54 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 0) (authored-target "h::hallExit_to_Classroom::air")
        (range (start 54 21) (end 54 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 66 21) (end 66 50)) (probe (position 66 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 4) (authored-target "h::hallExit_to_Storageroom::air")
        (range (start 66 21) (end 66 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 60 21) (end 60 52)) (probe (position 60 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 2) (authored-target "h::hallExit_to_Classroom::student")
        (range (start 60 21) (end 60 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 63 21) (end 63 52)) (probe (position 63 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 3) (authored-target "h::hallExit_to_Classroom::teacher")
        (range (start 63 21) (end 63 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 21) (end 57 54)) (probe (position 57 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 1) (authored-target "h::hallExit_to_Classroom::furniture")
        (range (start 57 21) (end 57 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 21) (end 69 56)) (probe (position 69 21))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))
        (kind flowSource) (ordinal 5) (authored-target "h::hallExit_to_Storageroom::furniture")
        (range (start 69 21) (end 69 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 23) (end 44 67)) (probe (position 44 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Part_Definitions::*")
        (range (start 44 23) (end 44 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 23) (end 45 67)) (probe (position 45 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Port_Definitions::*")
        (range (start 45 23) (end 45 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 23) (end 46 67)) (probe (position 46 23))
      (reference
        (source (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Flow_Definitions::*")
        (range (start 46 23) (end 46 67))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
