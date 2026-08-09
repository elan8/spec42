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
    package RoomDefinitionModelLibrary {
        private import Port_Definitions::*;
        private import Flow_Definitions::*;
        package Part_Definitions {
            // Rooms
            part def Classroom {
                port classEntry : EntryWay_to_Classroom;
            }
            part def Storageroom {
                port storageEntry : EntryWay_to_Storageroom;
            }
            part def Hallway {
                // conjugate ports with ~
                port hallExit_to_Classroom : ~EntryWay_to_Classroom;
                port hallExit_to_Storageroom : ~EntryWay_to_Storageroom;
            }
        }
        package Port_Definitions {
            port def EntryWay_to_Classroom {
                //flow properties
                in ref student : Student;
                in ref teacher : Teacher;
                in ref furniture : Furniture;
                in ref air : Air;
            }
            port def EntryWay_to_Storageroom {
                //flow properties
                in ref furniture : Furniture;
                in ref air : Air;
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
    package Room_Configuration {
        // defining the parts and their interconnection in context 
        private import RoomDefinitionModelLibrary::*;
        private import RoomDefinitionModelLibrary::Part_Definitions::*;
        private import RoomDefinitionModelLibrary::Port_Definitions::*;
        private import RoomDefinitionModelLibrary::Flow_Definitions::*;
        part roomContext {
            part c : Classroom;
            part s : Storageroom;
            part h : Hallway;

            //  Connectors and item flows between hallway and classroom
            flow HallToClassroom_Air from h.hallExit_to_Classroom.air to c.classEntry.air;
            flow HallToClassroom_Furniture from h.hallExit_to_Classroom.furniture to c.classEntry.furniture;
            flow HallToClassroom_Student from h.hallExit_to_Classroom.student to c.classEntry.student;
            flow HallToClassroom_Teacher from h.hallExit_to_Classroom.teacher to c.classEntry.teacher;
            flow HallToStorageroom_Air from h.hallExit_to_Storageroom.air to s.storageEntry.air;
            flow HallToStorageroom_Furniture from h.hallExit_to_Storageroom.furniture to s.storageEntry.furniture;
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
(model
  (namespace
    (package 'RoomModel'
      (package 'RoomDefinitionModelLibrary'
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions'[package])
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions'[package])
        (package 'Part_Definitions'
          (part_def 'Classroom'
            (port_usage composite 'classEntry' : 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom'[port_def]))
          (part_def 'Storageroom'
            (port_usage composite 'storageEntry' : 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom'[port_def]))
          (part_def 'Hallway'
            (port_usage composite 'hallExit_to_Classroom' : 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom'[port_def] ~ 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Classroom'[port_def])
            (port_usage composite 'hallExit_to_Storageroom' : 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom'[port_def] ~ 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions::EntryWay_to_Storageroom'[port_def])))
        (package 'Port_Definitions'
          (port_def 'EntryWay_to_Classroom'
            (reference_usage in reference 'student' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Student'[part_def])
            (reference_usage in reference 'teacher' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Teacher'[part_def])
            (reference_usage in reference 'furniture' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture'[part_def])
            (reference_usage in reference 'air' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air'[part_def]))
          (port_def 'EntryWay_to_Storageroom'
            (reference_usage in reference 'furniture' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Furniture'[part_def])
            (reference_usage in reference 'air' : 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions::Air'[part_def])))
        (package 'Flow_Definitions'
          (part_def 'Air')
          (part_def 'Furniture')
          (part_def 'Student')
          (part_def 'Teacher')))
      (package 'Room_Configuration'
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary'[package])
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary::Part_Definitions'[package])
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary::Port_Definitions'[package])
        (namespace_import private -> 'RoomModel::RoomDefinitionModelLibrary::Flow_Definitions'[package])
        (part_usage 'roomContext'
          (part_usage composite 'c' : 'RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Classroom'[part_def])
          (part_usage composite 's' : 'RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Storageroom'[part_def])
          (part_usage composite 'h' : 'RoomModel::RoomDefinitionModelLibrary::Part_Definitions::Hallway'[part_def])
          (flow_usage composite 'HallToClassroom_Air'
            (connector_end 'h.hallExit_to_Classroom.air')
            (connector_end 'c.classEntry.air'))
          (flow_usage composite 'HallToClassroom_Furniture'
            (connector_end 'h.hallExit_to_Classroom.furniture')
            (connector_end 'c.classEntry.furniture'))
          (flow_usage composite 'HallToClassroom_Student'
            (connector_end 'h.hallExit_to_Classroom.student')
            (connector_end 'c.classEntry.student'))
          (flow_usage composite 'HallToClassroom_Teacher'
            (connector_end 'h.hallExit_to_Classroom.teacher')
            (connector_end 'c.classEntry.teacher'))
          (flow_usage composite 'HallToStorageroom_Air'
            (connector_end 'h.hallExit_to_Storageroom.air')
            (connector_end 's.storageEntry.air'))
          (flow_usage composite 'HallToStorageroom_Furniture'
            (connector_end 'h.hallExit_to_Storageroom.furniture')
            (connector_end 's.storageEntry.furniture')))))))
~~~
