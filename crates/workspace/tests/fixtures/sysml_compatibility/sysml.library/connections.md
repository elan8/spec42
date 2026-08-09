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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Connections"))) (name "Connections") (declared-name "Connections")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (name "BinaryConnection") (declared-name "BinaryConnection")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Connections::BinaryConnection::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Connections::BinaryConnection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Connections::BinaryConnection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Connections::BinaryConnection")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (name "BinaryLinkObject") (declared-name "BinaryLinkObject"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Connections::Connection"))) (name "Connection") (declared-name "Connection")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Connections::Connection::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Connections::Connection")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::FlowTransfer"))) (name "FlowTransfer") (declared-name "FlowTransfer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::FlowTransferBefore"))) (name "FlowTransferBefore") (declared-name "FlowTransferBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::HappensDuring"))) (name "HappensDuring") (declared-name "HappensDuring"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::LinkObject"))) (name "LinkObject") (declared-name "LinkObject"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Part"))) (name "Part") (declared-name "Part"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::Transfer"))) (name "Transfer") (declared-name "Transfer"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Connections::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::actions"))) (name "actions") (declared-name "actions"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Connections::binaryConnections"))) (name "binaryConnections") (declared-name "binaryConnections")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Connections::binaryConnections::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Connections::binaryConnections")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (name "binaryLinkObjects") (declared-name "binaryLinkObjects"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Connections::connections"))) (name "connections") (declared-name "connections")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Connections::connections::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Connections::connections")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::flowTransfers"))) (name "flowTransfers") (declared-name "flowTransfers"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::flowTransfersBefore"))) (name "flowTransfersBefore") (declared-name "flowTransfersBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::linkObjects"))) (name "linkObjects") (declared-name "linkObjects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::parts"))) (name "parts") (declared-name "parts"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Connections::transfers"))) (name "transfers") (declared-name "transfers"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Connections::BinaryConnection::_documentation"))) (to (node (document "d0") (qualified-name "Connections::BinaryConnection"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Connections::Connection::_documentation"))) (to (node (document "d0") (qualified-name "Connections::Connection"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Connections::_documentation"))) (to (node (document "d0") (qualified-name "Connections"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Connections::binaryConnections::_documentation"))) (to (node (document "d0") (qualified-name "Connections::binaryConnections"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Connections::connections::_documentation"))) (to (node (document "d0") (qualified-name "Connections::connections"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (to (node (document "d0") (qualified-name "Connections::Connection"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Connections::binaryConnections"))) (to (node (document "d0") (qualified-name "Connections::connections"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
