# META
~~~ini
description=Standard Library: Systems Library/Connections
type=file
~~~
# SOURCE
~~~sysml
standard library package Connections {
    doc
    /*
     * This package defines the base types for connections and related structural elements 
     * in the SysML language.
     */

    private import Base::Anything;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::LinkObject;
    private import Objects::linkObjects;
    private import Objects::BinaryLinkObject;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import ScalarValues::Natural;
    private import Parts::Part;
    private import Parts::parts;
    private import Actions::Action;
    private import Actions::actions;

    abstract connection def Connection :> LinkObject, Part {
        doc
        /*
         * Connection is the most general class of links between things within some 
         * containing structure. Connection is the base type of all ConnectionDefinitions.
         */
    }
     
    abstract connection def BinaryConnection :> BinaryLinkObject, Connection {
        doc
        /*
         * BinaryConnection is the most general class of binary links between two things 
         * within some containing structure. BinaryConnection is the base type of all 
         * ConnectionDefinitions with exactly two ends.
         */
    
        end source: Anything :>> BinaryLinkObject::source;
        end target: Anything :>> BinaryLinkObject::target;
    }
    
    abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts {
        doc
        /*
         * connections is the base feature of all ConnectionUsages.
         */
    }
    
    abstract connection binaryConnections: Connection[0..*] nonunique :> connections, binaryLinkObjects {
        doc
        /*
         * binaryConnections is the base feature of all binary ConnectionUsages.
         */
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'LinkObject'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'BinaryLinkObject'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLinkObject::source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLinkObject::target'
semantic.unresolved_name 'linkObjects'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'binaryLinkObjects'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'LinkObject'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'BinaryLinkObject'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLinkObject::source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLinkObject::target'
semantic.unresolved_name 'linkObjects'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'binaryLinkObjects'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwConnection,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConnection,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Connections'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'Objects::LinkObject')
    (import_decl private 'Objects::linkObjects')
    (import_decl private 'Objects::BinaryLinkObject')
    (import_decl private 'Objects::binaryLinkObjects')
    (import_decl private 'Transfers::Transfer')
    (import_decl private 'Transfers::transfers')
    (import_decl private 'Transfers::FlowTransfer')
    (import_decl private 'Transfers::flowTransfers')
    (import_decl private 'Transfers::FlowTransferBefore')
    (import_decl private 'Transfers::flowTransfersBefore')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (import_decl private 'Actions::Action')
    (import_decl private 'Actions::actions')
    (connection_def abstract 'Connection' :> 'LinkObject', 'Part'
      (documentation))
    (connection_def abstract 'BinaryConnection' :> 'BinaryLinkObject', 'Connection'
      (documentation)
      (interface_end end 'source' : 'Anything' :>> 'BinaryLinkObject::source')
      (interface_end end 'target' : 'Anything' :>> 'BinaryLinkObject::target'))
    (connection_usage 'Connection' :> 'linkObjects', 'parts' 'connections' multiplicity
      (documentation))
    (connection_usage 'Connection' :> 'connections', 'binaryLinkObjects' 'binaryConnections' multiplicity
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Connections {
    doc /*
     * This package defines the base types for connections and related structural elements 
     * in the SysML language.
     */

    private import Base::Anything;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::LinkObject;
    private import Objects::linkObjects;
    private import Objects::BinaryLinkObject;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import ScalarValues::Natural;
    private import Parts::Part;
    private import Parts::parts;
    private import Actions::Action;
    private import Actions::actions;

    abstract connection def Connection :> LinkObject, Part {
        doc /*
         * Connection is the most general class of links between things within some 
         * containing structure. Connection is the base type of all ConnectionDefinitions.
         */
    }

    abstract connection def BinaryConnection :> BinaryLinkObject, Connection {
        doc /*
         * BinaryConnection is the most general class of binary links between two things 
         * within some containing structure. BinaryConnection is the base type of all 
         * ConnectionDefinitions with exactly two ends.
         */

        end source : Anything :>> BinaryLinkObject::source;
        end target : Anything :>> BinaryLinkObject::target;
    }

    abstract connection connections : Connection :> linkObjects, parts [0..*] {
        doc /*
         * connections is the base feature of all ConnectionUsages.
         */
    }

    abstract connection binaryConnections : Connection :> connections, binaryLinkObjects [0..*] {
        doc /*
         * binaryConnections is the base feature of all binary ConnectionUsages.
         */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Connections'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'Objects::LinkObject'[unresolved])
      (membership_import private -> 'Objects::linkObjects'[unresolved])
      (membership_import private -> 'Objects::BinaryLinkObject'[unresolved])
      (membership_import private -> 'Objects::binaryLinkObjects'[unresolved])
      (membership_import private -> 'Transfers::Transfer'[unresolved])
      (membership_import private -> 'Transfers::transfers'[unresolved])
      (membership_import private -> 'Transfers::FlowTransfer'[unresolved])
      (membership_import private -> 'Transfers::flowTransfers'[unresolved])
      (membership_import private -> 'Transfers::FlowTransferBefore'[unresolved])
      (membership_import private -> 'Transfers::flowTransfersBefore'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'Parts::Part'[unresolved])
      (membership_import private -> 'Parts::parts'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Actions::actions'[unresolved])
      (connection_def abstract 'Connection' :> 'LinkObject'[unresolved] :> 'Part'[unresolved]
        (documentation))
      (connection_def abstract 'BinaryConnection' :> 'BinaryLinkObject'[unresolved] :> 'Connections::Connection'[connection_def]
        (documentation)
        (port_usage end 'source' : 'Anything'[unresolved] :>> 'BinaryLinkObject::source'[unresolved])
        (port_usage end 'target' : 'Anything'[unresolved] :>> 'BinaryLinkObject::target'[unresolved]))
      (connection_usage abstract 'connections' : 'Connections::Connection'[connection_def] :> 'linkObjects'[unresolved] :> 'parts'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (connection_usage abstract 'binaryConnections' : 'Connections::Connection'[connection_def] :> 'Connections::connections'[connection_usage] :> 'binaryLinkObjects'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
