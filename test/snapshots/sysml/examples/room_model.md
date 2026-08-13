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
  (document "memory://snapshot/room_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 8 16) (end 8 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 16) (end 11 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 16) (end 15 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 16) (end 16 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 12) (end 26 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 12) (end 31 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 53 12) (end 55 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 56 12) (end 58 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 59 12) (end 61 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 62 12) (end 64 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 65 12) (end 67 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 68 12) (end 70 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:95bbd68fcc77102e975866ee00d900456df50b7e94709f5891da94c948453a0e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port_Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Flow_Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Student"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Teacher"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RoomDefinitionModelLibrary") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RoomDefinitionModelLibrary::Part_Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RoomDefinitionModelLibrary::Port_Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RoomDefinitionModelLibrary::Flow_Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Classroom"))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hallway"))))
    (declaration (id (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Storageroom"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Flow_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RoomDefinitionModelLibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RoomDefinitionModelLibrary::Part_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RoomDefinitionModelLibrary::Port_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RoomDefinitionModelLibrary::Flow_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Classroom")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hallway")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway")))))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "Storageroom")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/room_model.md") (range (start 3 23) (end 3 42)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 4 23) (end 4 42)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Flow_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 43 23) (end 43 52)) (probe (position 43 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 44 23) (end 44 70)) (probe (position 44 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Part_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 45 23) (end 45 70)) (probe (position 45 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Port_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 46 23) (end 46 70)) (probe (position 46 23))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "RoomDefinitionModelLibrary::Flow_Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 48 19) (end 48 28)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind featureTyping) (ordinal 0) (authored-target "Classroom")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 50 19) (end 50 26)) (probe (position 50 19))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind featureTyping) (ordinal 0) (authored-target "Hallway")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway")))))
  )
  (query (document "memory://snapshot/room_model.md") (range (start 49 19) (end 49 30)) (probe (position 49 19))
    (reference (id (source (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind featureTyping) (ordinal 0) (authored-target "Storageroom")
      (outcome (status resolved) (target (node (document "memory://snapshot/room_model.md") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom")))))
  )
)
~~~
