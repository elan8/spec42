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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel"))) (name "RoomModel") (declared-name "RoomModel")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary"))) (name "RoomDefinitionModelLibrary") (declared-name "RoomDefinitionModelLibrary")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::*#import"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions"))) (name "Flow_Definitions") (declared-name "Flow_Definitions")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air"))) (name "Air") (declared-name "Air") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture"))) (name "Furniture") (declared-name "Furniture") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Student"))) (name "Student") (declared-name "Student") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Teacher"))) (name "Teacher") (declared-name "Teacher") (declared))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions"))) (name "Part_Definitions") (declared-name "Part_Definitions")
              (contains
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))) (name "Classroom") (declared-name "Classroom") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (name "classEntry") (declared-name "classEntry") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))) (name "Hallway") (declared-name "Hallway") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (name "hallExit_to_Classroom") (declared-name "hallExit_to_Classroom") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (name "hallExit_to_Storageroom") (declared-name "hallExit_to_Storageroom") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))) (name "Storageroom") (declared-name "Storageroom") (declared)
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (name "storageEntry") (declared-name "storageEntry") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom")))))
                  )
                )
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions"))) (name "Port_Definitions") (declared-name "Port_Definitions")
              (contains
                (element (kind "port def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))) (name "EntryWay_to_Classroom") (declared-name "EntryWay_to_Classroom")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter2"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::ref#in_out_parameter3"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::~EntryWay_to_Classroom"))) (name "~EntryWay_to_Classroom") (declared-name "~EntryWay_to_Classroom") (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom")))))
                  )
                )
                (element (kind "port def") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))) (name "EntryWay_to_Storageroom") (declared-name "EntryWay_to_Storageroom")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::ref#in_out_parameter"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::~EntryWay_to_Storageroom"))) (name "~EntryWay_to_Storageroom") (declared-name "~EntryWay_to_Storageroom") (effective (featuring-type (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration"))) (name "Room_Configuration") (declared-name "Room_Configuration")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import2"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::*#import3"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext"))) (name "roomContext") (declared-name "roomContext") (declared (properties (ordered false)))
              (contains
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Air"))) (name "HallToClassroom_Air") (declared-name "HallToClassroom_Air"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Furniture"))) (name "HallToClassroom_Furniture") (declared-name "HallToClassroom_Furniture"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Student"))) (name "HallToClassroom_Student") (declared-name "HallToClassroom_Student"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToClassroom_Teacher"))) (name "HallToClassroom_Teacher") (declared-name "HallToClassroom_Teacher"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Air"))) (name "HallToStorageroom_Air") (declared-name "HallToStorageroom_Air"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::HallToStorageroom_Furniture"))) (name "HallToStorageroom_Furniture") (declared-name "HallToStorageroom_Furniture"))
                (element (kind "part") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (name "c") (declared-name "c") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (name "h") (declared-name "h") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (name "s") (declared-name "s") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::~EntryWay_to_Classroom"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::~EntryWay_to_Storageroom"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom::classEntry"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Classroom"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom::~EntryWay_to_Classroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway::hallExit_to_Storageroom"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom::~EntryWay_to_Storageroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom::storageEntry"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::c"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::h"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RoomModel::Room_Configuration::roomContext::s"))) (to (node (document "d0") (qualified-name "RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/room_model.md"
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
        (range (start 43 8) (end 43 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 44 8) (end 44 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 45 8) (end 45 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 8) (end 46 71))
      )
    )
  )
)
~~~
