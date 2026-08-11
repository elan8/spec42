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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 19) (end 13 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 19) (end 18 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 19) (end 19 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 19) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 19) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 19) (end 22 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 19) (end 23 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 19) (end 24 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 33) (end 42 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 33) (end 43 57))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b74da91a9bcd7d225cda3b0460bb8ebd89a05e7c1836fc59dba28e8c08149a92") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Connections"))) (kind "package") (name "Connections") (declared-name "Connections") (range (start (line 0) (character 0)) (end (line 0) (character 2133))))
    (element (id (node (document "d0") (qualified-name "Connections::Action"))) (kind "import") (name "Action") (declared-name "Action") (range (start (line 23) (character 4)) (end (line 23) (character 35))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 23) (character 19)) (end (line 23) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Connections::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 4)) (end (line 7) (character 34))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind "connection def") (name "BinaryConnection") (declared-name "BinaryConnection") (range (start (line 34) (character 4)) (end (line 34) (character 475))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BinaryLinkObject") (range (start (line 34) (character 48)) (end (line 34) (character 64)))) (specializes (reference "Connection") (range (start (line 34) (character 66)) (end (line 34) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::_documentation"))) (kind "documentation") (name "") (range (start (line 34) (character 4)) (end (line 34) (character 475))) (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 42) (character 8)) (end (line 42) (character 58))) (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (authored (relationships (typing (reference "Anything") (range none)) (redefinition (reference "BinaryLinkObject::source") (range (start (line 42) (character 33)) (end (line 42) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 43) (character 8)) (end (line 43) (character 58))) (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (authored (relationships (typing (reference "Anything") (range none)) (redefinition (reference "BinaryLinkObject::target") (range (start (line 43) (character 33)) (end (line 43) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (kind "import") (name "BinaryLinkObject") (declared-name "BinaryLinkObject") (range (start (line 12) (character 4)) (end (line 12) (character 45))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::BinaryLinkObject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Connections::Connection"))) (kind "connection def") (name "Connection") (declared-name "Connection") (range (start (line 26) (character 4)) (end (line 26) (character 277))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LinkObject") (range (start (line 26) (character 42)) (end (line 26) (character 52)))) (specializes (reference "Part") (range (start (line 26) (character 54)) (end (line 26) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Connections::Connection::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 4)) (end (line 26) (character 277))) (parent (node (document "d0") (qualified-name "Connections::Connection"))))
    (element (id (node (document "d0") (qualified-name "Connections::FlowTransfer"))) (kind "import") (name "FlowTransfer") (declared-name "FlowTransfer") (range (start (line 16) (character 4)) (end (line 16) (character 43))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Connections::FlowTransferBefore"))) (kind "import") (name "FlowTransferBefore") (declared-name "FlowTransferBefore") (range (start (line 18) (character 4)) (end (line 18) (character 49))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransferBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 19)) (end (line 18) (character 48))))))
    (element (id (node (document "d0") (qualified-name "Connections::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (range (start (line 9) (character 4)) (end (line 9) (character 46))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Connections::LinkObject"))) (kind "import") (name "LinkObject") (declared-name "LinkObject") (range (start (line 10) (character 4)) (end (line 10) (character 39))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::LinkObject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Connections::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 20) (character 4)) (end (line 20) (character 41))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Connections::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 8) (character 4)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Connections::Part"))) (kind "import") (name "Part") (declared-name "Part") (range (start (line 21) (character 4)) (end (line 21) (character 31))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 19)) (end (line 21) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Connections::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (range (start (line 14) (character 4)) (end (line 14) (character 39))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Connections::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2133))) (parent (node (document "d0") (qualified-name "Connections"))))
    (element (id (node (document "d0") (qualified-name "Connections::actions"))) (kind "import") (name "actions") (declared-name "actions") (range (start (line 24) (character 4)) (end (line 24) (character 36))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 19)) (end (line 24) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind "connection def") (name "binaryConnections") (declared-name "binaryConnections") (range (start (line 53) (character 4)) (end (line 53) (character 227))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "connections") (range (start (line 0) (character 0)) (end (line 0) (character 11)))) (specializes (reference "binaryLinkObjects") (range (start (line 0) (character 13)) (end (line 0) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryConnections::_documentation"))) (kind "documentation") (name "") (range (start (line 53) (character 4)) (end (line 53) (character 227))) (parent (node (document "d0") (qualified-name "Connections::binaryConnections"))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (kind "import") (name "binaryLinkObjects") (declared-name "binaryLinkObjects") (range (start (line 13) (character 4)) (end (line 13) (character 46))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::binaryLinkObjects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 19)) (end (line 13) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Connections::connections"))) (kind "connection def") (name "connections") (declared-name "connections") (range (start (line 46) (character 4)) (end (line 46) (character 196))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "linkObjects") (range (start (line 0) (character 0)) (end (line 0) (character 11)))) (specializes (reference "parts") (range (start (line 0) (character 13)) (end (line 0) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "Connections::connections::_documentation"))) (kind "documentation") (name "") (range (start (line 46) (character 4)) (end (line 46) (character 196))) (parent (node (document "d0") (qualified-name "Connections::connections"))))
    (element (id (node (document "d0") (qualified-name "Connections::flowTransfers"))) (kind "import") (name "flowTransfers") (declared-name "flowTransfers") (range (start (line 17) (character 4)) (end (line 17) (character 44))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfers") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Connections::flowTransfersBefore"))) (kind "import") (name "flowTransfersBefore") (declared-name "flowTransfersBefore") (range (start (line 19) (character 4)) (end (line 19) (character 50))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfersBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 19)) (end (line 19) (character 49))))))
    (element (id (node (document "d0") (qualified-name "Connections::linkObjects"))) (kind "import") (name "linkObjects") (declared-name "linkObjects") (range (start (line 11) (character 4)) (end (line 11) (character 40))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::linkObjects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Connections::parts"))) (kind "import") (name "parts") (declared-name "parts") (range (start (line 22) (character 4)) (end (line 22) (character 32))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 19)) (end (line 22) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Connections::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (range (start (line 15) (character 4)) (end (line 15) (character 40))) (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Connections::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (range (start (line 23) (character 19)) (end (line 23) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 19)) (end (line 7) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 0)) (authored-target "BinaryLinkObject") (range (start (line 34) (character 48)) (end (line 34) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::BinaryLinkObject")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1)) (authored-target "Connection") (range (start (line 34) (character 66)) (end (line 34) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Connection")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryLinkObject::source") (range (start (line 42) (character 33)) (end (line 42) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryLinkObject::target") (range (start (line 43) (character 33)) (end (line 43) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::BinaryLinkObject") (range (start (line 12) (character 19)) (end (line 12) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 0)) (authored-target "LinkObject") (range (start (line 26) (character 42)) (end (line 26) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::LinkObject")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 1)) (authored-target "Part") (range (start (line 26) (character 54)) (end (line 26) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::FlowTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransfer") (range (start (line 16) (character 19)) (end (line 16) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::FlowTransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransferBefore") (range (start (line 18) (character 19)) (end (line 18) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (range (start (line 9) (character 19)) (end (line 9) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::LinkObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::LinkObject") (range (start (line 10) (character 19)) (end (line 10) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 20) (character 19)) (end (line 20) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 8) (character 19)) (end (line 8) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (range (start (line 21) (character 19)) (end (line 21) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (range (start (line 14) (character 19)) (end (line 14) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (range (start (line 24) (character 19)) (end (line 24) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0)) (authored-target "connections") (range (start (line 0) (character 0)) (end (line 0) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::connections")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 1)) (authored-target "binaryLinkObjects") (range (start (line 0) (character 13)) (end (line 0) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::binaryLinkObjects")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::binaryLinkObjects") (range (start (line 13) (character 19)) (end (line 13) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 0)) (authored-target "linkObjects") (range (start (line 0) (character 0)) (end (line 0) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::linkObjects")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 1)) (authored-target "parts") (range (start (line 0) (character 13)) (end (line 0) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::flowTransfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfers") (range (start (line 17) (character 19)) (end (line 17) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::flowTransfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfersBefore") (range (start (line 19) (character 19)) (end (line 19) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::linkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::linkObjects") (range (start (line 11) (character 19)) (end (line 11) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (range (start (line 22) (character 19)) (end (line 22) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (range (start (line 15) (character 19)) (end (line 15) (character 39))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (target (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (target (node (document "d0") (qualified-name "Connections::Connection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (target (node (document "d0") (qualified-name "Connections::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (target (node (document "d0") (qualified-name "Connections::Anything"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::Connection"))) (target (node (document "d0") (qualified-name "Connections::LinkObject"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::Connection"))) (target (node (document "d0") (qualified-name "Connections::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (target (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (target (node (document "d0") (qualified-name "Connections::connections"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::connections"))) (target (node (document "d0") (qualified-name "Connections::linkObjects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Connections::connections"))) (target (node (document "d0") (qualified-name "Connections::parts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
