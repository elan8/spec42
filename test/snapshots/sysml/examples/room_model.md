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
# TOKENS
~~~zig
LineComment,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
LineComment,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
LineComment,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPort,KwDef,Ident,OpenCurly,
LineComment,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
LineComment,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
LineComment,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (line_comment)
  (package_def 'RoomModel'
    (package_def 'RoomDefinitionModelLibrary'
      (import_decl private 'Port_Definitions::*')
      (import_decl private 'Flow_Definitions::*')
      (package_def 'Part_Definitions'
        (line_comment)
        (part_def 'Classroom'
          (port_usage 'classEntry' : 'EntryWay_to_Classroom'))
        (part_def 'Storageroom'
          (port_usage 'storageEntry' : 'EntryWay_to_Storageroom'))
        (part_def 'Hallway'
          (line_comment)
          (port_usage 'hallExit_to_Classroom' : ~'EntryWay_to_Classroom')
          (port_usage 'hallExit_to_Storageroom' : ~'EntryWay_to_Storageroom')))
      (package_def 'Port_Definitions'
        (port_def 'EntryWay_to_Classroom'
          (line_comment)
          (ref_usage in ref 'student' : 'Student')
          (ref_usage in ref 'teacher' : 'Teacher')
          (ref_usage in ref 'furniture' : 'Furniture')
          (ref_usage in ref 'air' : 'Air'))
        (port_def 'EntryWay_to_Storageroom'
          (line_comment)
          (ref_usage in ref 'furniture' : 'Furniture')
          (ref_usage in ref 'air' : 'Air')))
      (package_def 'Flow_Definitions'
        (line_comment)
        (part_def 'Air')
        (part_def 'Furniture')
        (part_def 'Student')
        (part_def 'Teacher')))
    (package_def 'Room_Configuration'
      (line_comment)
      (import_decl private 'RoomDefinitionModelLibrary::*')
      (import_decl private 'RoomDefinitionModelLibrary::Part_Definitions::*')
      (import_decl private 'RoomDefinitionModelLibrary::Port_Definitions::*')
      (import_decl private 'RoomDefinitionModelLibrary::Flow_Definitions::*')
      (part_usage 'roomContext'
        (part_usage 'c' : 'Classroom')
        (part_usage 's' : 'Storageroom')
        (part_usage 'h' : 'Hallway')
        (line_comment)
        (flow_usage 'HallToClassroom_Air'
          (connector_end)
          (connector_end))
        (flow_usage 'HallToClassroom_Furniture'
          (connector_end)
          (connector_end))
        (flow_usage 'HallToClassroom_Student'
          (connector_end)
          (connector_end))
        (flow_usage 'HallToClassroom_Teacher'
          (connector_end)
          (connector_end))
        (flow_usage 'HallToStorageroom_Air'
          (connector_end)
          (connector_end))
        (flow_usage 'HallToStorageroom_Furniture'
          (connector_end)
          (connector_end))))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4c64371a767360793a21a6acbbca6c8c330e508d61b48450b1b3d2da32ab08eb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RoomModel"))) (kind "package") (name "RoomModel") (declared-name "RoomModel") (range (start (line 1) (character 0)) (end (line 1) (character 2775))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (kind "package") (name "RoomDefinitionModelLibrary") (declared-name "RoomDefinitionModelLibrary") (range (start (line 2) (character 4)) (end (line 2) (character 1361))) (parent (node (document "d0") (qualified-name "RoomModel"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 8)) (end (line 3) (character 43))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port_Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 23)) (end (line 3) (character 39))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 8)) (end (line 4) (character 43))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (authored (membership (kind Import) (visibility "private") (import (reference "Flow_Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 23)) (end (line 4) (character 39))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))) (kind "package") (name "Flow_Definitions") (declared-name "Flow_Definitions") (range (start (line 33) (character 8)) (end (line 33) (character 256))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air"))) (kind "part def") (name "Air") (declared-name "Air") (range (start (line 35) (character 16)) (end (line 35) (character 29))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture"))) (kind "part def") (name "Furniture") (declared-name "Furniture") (range (start (line 36) (character 16)) (end (line 36) (character 35))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Student"))) (kind "part def") (name "Student") (declared-name "Student") (range (start (line 37) (character 16)) (end (line 37) (character 33))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Teacher"))) (kind "part def") (name "Teacher") (declared-name "Teacher") (range (start (line 38) (character 16)) (end (line 38) (character 33))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))) (kind "package") (name "Part_Definitions") (declared-name "Part_Definitions") (range (start (line 5) (character 8)) (end (line 5) (character 503))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (kind "part def") (name "Classroom") (declared-name "Classroom") (range (start (line 7) (character 12)) (end (line 7) (character 102))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (kind "port") (name "classEntry") (declared-name "classEntry") (range (start (line 8) (character 16)) (end (line 8) (character 55))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (authored (membership (kind Feature)) (relationships (typing (reference "EntryWay_to_Classroom") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (kind "part def") (name "Hallway") (declared-name "Hallway") (range (start (line 13) (character 12)) (end (line 13) (character 226))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (kind "port") (name "hallExit_to_Classroom") (declared-name "hallExit_to_Classroom") (range (start (line 15) (character 16)) (end (line 15) (character 67))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (authored (membership (kind Feature)) (relationships (typing (reference "~EntryWay_to_Classroom") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (kind "port") (name "hallExit_to_Storageroom") (declared-name "hallExit_to_Storageroom") (range (start (line 16) (character 16)) (end (line 16) (character 71))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (authored (membership (kind Feature)) (relationships (typing (reference "~EntryWay_to_Storageroom") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (kind "part def") (name "Storageroom") (declared-name "Storageroom") (range (start (line 10) (character 12)) (end (line 10) (character 108))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (kind "port") (name "storageEntry") (declared-name "storageEntry") (range (start (line 11) (character 16)) (end (line 11) (character 59))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (authored (membership (kind Feature)) (relationships (typing (reference "EntryWay_to_Storageroom") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))) (kind "package") (name "Port_Definitions") (declared-name "Port_Definitions") (range (start (line 19) (character 8)) (end (line 19) (character 466))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (kind "port def") (name "EntryWay_to_Classroom") (declared-name "EntryWay_to_Classroom") (range (start (line 20) (character 12)) (end (line 20) (character 248))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 22) (character 16)) (end (line 22) (character 39))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref student:Student") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 23) (character 16)) (end (line 23) (character 39))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref teacher:Teacher") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter2"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 24) (character 16)) (end (line 24) (character 43))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref furniture:Furniture") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter3"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 25) (character 16)) (end (line 25) (character 31))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (authored (relationships (typing (reference "ref air:Air") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::~EntryWay_to_Classroom"))) (kind "conjugated port definition") (name "~EntryWay_to_Classroom") (declared-name "~EntryWay_to_Classroom") (range (start (line 20) (character 12)) (end (line 20) (character 248))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (kind "port def") (name "EntryWay_to_Storageroom") (declared-name "EntryWay_to_Storageroom") (range (start (line 27) (character 12)) (end (line 27) (character 173))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 29) (character 16)) (end (line 29) (character 45))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (authored (relationships (typing (reference "ref furniture:  Furniture") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 30) (character 16)) (end (line 30) (character 32))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (authored (relationships (typing (reference "ref air: Air") (range none)))))
    (element (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::~EntryWay_to_Storageroom"))) (kind "conjugated port definition") (name "~EntryWay_to_Storageroom") (declared-name "~EntryWay_to_Storageroom") (range (start (line 27) (character 12)) (end (line 27) (character 173))) (parent (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (kind "package") (name "Room_Configuration") (declared-name "Room_Configuration") (range (start (line 41) (character 4)) (end (line 41) (character 1390))) (parent (node (document "d0") (qualified-name "RoomModel"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 43) (character 8)) (end (line 43) (character 53))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 43) (character 23)) (end (line 43) (character 49))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 44) (character 8)) (end (line 44) (character 71))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Part_Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 44) (character 23)) (end (line 44) (character 67))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 45) (character 8)) (end (line 45) (character 71))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Port_Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 45) (character 23)) (end (line 45) (character 67))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 46) (character 8)) (end (line 46) (character 71))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (authored (membership (kind Import) (visibility "private") (import (reference "RoomDefinitionModelLibrary::Flow_Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 46) (character 23)) (end (line 46) (character 67))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind "part") (name "roomContext") (declared-name "roomContext") (range (start (line 47) (character 8)) (end (line 47) (character 1014))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Air"))) (kind "flow") (name "HallToClassroom_Air") (declared-name "HallToClassroom_Air") (range (start (line 53) (character 12)) (end (line 53) (character 122))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Furniture"))) (kind "flow") (name "HallToClassroom_Furniture") (declared-name "HallToClassroom_Furniture") (range (start (line 56) (character 12)) (end (line 56) (character 140))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Student"))) (kind "flow") (name "HallToClassroom_Student") (declared-name "HallToClassroom_Student") (range (start (line 59) (character 12)) (end (line 59) (character 134))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Teacher"))) (kind "flow") (name "HallToClassroom_Teacher") (declared-name "HallToClassroom_Teacher") (range (start (line 62) (character 12)) (end (line 62) (character 134))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Air"))) (kind "flow") (name "HallToStorageroom_Air") (declared-name "HallToStorageroom_Air") (range (start (line 65) (character 12)) (end (line 65) (character 128))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Furniture"))) (kind "flow") (name "HallToStorageroom_Furniture") (declared-name "HallToStorageroom_Furniture") (range (start (line 68) (character 12)) (end (line 68) (character 146))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 48) (character 12)) (end (line 48) (character 29))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Classroom") (range (start (line 48) (character 19)) (end (line 48) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind "part") (name "h") (declared-name "h") (range (start (line 50) (character 12)) (end (line 50) (character 27))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hallway") (range (start (line 50) (character 19)) (end (line 50) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind "part") (name "s") (declared-name "s") (range (start (line 49) (character 12)) (end (line 49) (character 31))) (parent (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Storageroom") (range (start (line 49) (character 19)) (end (line 49) (character 30)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port_Definitions::*") (range (start (line 3) (character 23)) (end (line 3) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Flow_Definitions::*") (range (start (line 4) (character 23)) (end (line 4) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (kind featureTyping) (ordinal 0)) (authored-target "EntryWay_to_Classroom") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (kind featureTyping) (ordinal 0)) (authored-target "~EntryWay_to_Classroom") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (kind featureTyping) (ordinal 0)) (authored-target "~EntryWay_to_Storageroom") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (kind featureTyping) (ordinal 0)) (authored-target "EntryWay_to_Storageroom") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref student:Student") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref teacher:Teacher") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter2"))) (kind featureTyping) (ordinal 0)) (authored-target "ref furniture:Furniture") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter3"))) (kind featureTyping) (ordinal 0)) (authored-target "ref air:Air") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref furniture:  Furniture") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref air: Air") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::*") (range (start (line 43) (character 23)) (end (line 43) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Part_Definitions::*") (range (start (line 44) (character 23)) (end (line 44) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Port_Definitions::*") (range (start (line 45) (character 23)) (end (line 45) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "RoomDefinitionModelLibrary::Flow_Definitions::*") (range (start (line 46) (character 23)) (end (line 46) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 0)) (authored-target "h::hallExit_to_Classroom::air") (range (start (line 54) (character 21)) (end (line 54) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 1)) (authored-target "h::hallExit_to_Classroom::furniture") (range (start (line 57) (character 21)) (end (line 57) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 2)) (authored-target "h::hallExit_to_Classroom::student") (range (start (line 60) (character 21)) (end (line 60) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 3)) (authored-target "h::hallExit_to_Classroom::teacher") (range (start (line 63) (character 21)) (end (line 63) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 4)) (authored-target "h::hallExit_to_Storageroom::air") (range (start (line 66) (character 21)) (end (line 66) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowSource) (ordinal 5)) (authored-target "h::hallExit_to_Storageroom::furniture") (range (start (line 69) (character 21)) (end (line 69) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 0)) (authored-target "c::classEntry::air") (range (start (line 55) (character 19)) (end (line 55) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 1)) (authored-target "c::classEntry::furniture") (range (start (line 58) (character 19)) (end (line 58) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 2)) (authored-target "c::classEntry::student") (range (start (line 61) (character 19)) (end (line 61) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 3)) (authored-target "c::classEntry::teacher") (range (start (line 64) (character 19)) (end (line 64) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 4)) (authored-target "s::storageEntry::air") (range (start (line 67) (character 19)) (end (line 67) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (kind flowTarget) (ordinal 5)) (authored-target "s::storageEntry::furniture") (range (start (line 70) (character 19)) (end (line 70) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (kind featureTyping) (ordinal 0)) (authored-target "Classroom") (range (start (line 48) (character 19)) (end (line 48) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (kind featureTyping) (ordinal 0)) (authored-target "Hallway") (range (start (line 50) (character 19)) (end (line 50) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (kind featureTyping) (ordinal 0)) (authored-target "Storageroom") (range (start (line 49) (character 19)) (end (line 49) (character 30))) (outcome (status unresolved)))
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
