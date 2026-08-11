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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "38b73beca1bb8bc169b9ae39099ba770866dd61b0e9bb75698f0907288704a32") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Connections"))) (kind "package") (name "Connections") (declared-name "Connections"))
    (element (id (node (document "d0") (qualified-name "Connections::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind "connection def") (name "BinaryConnection") (declared-name "BinaryConnection") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BinaryLinkObject")) (specializes (reference "Connection")))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (authored (relationships (typing (reference "Anything")) (redefinition (reference "BinaryLinkObject::source")))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (authored (relationships (typing (reference "Anything")) (redefinition (reference "BinaryLinkObject::target")))))
    (element (id (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (kind "import") (name "BinaryLinkObject") (declared-name "BinaryLinkObject") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::BinaryLinkObject") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Connection"))) (kind "connection def") (name "Connection") (declared-name "Connection") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LinkObject")) (specializes (reference "Part")))))
    (element (id (node (document "d0") (qualified-name "Connections::Connection::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Connections::Connection"))))
    (element (id (node (document "d0") (qualified-name "Connections::FlowTransfer"))) (kind "import") (name "FlowTransfer") (declared-name "FlowTransfer") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::FlowTransferBefore"))) (kind "import") (name "FlowTransferBefore") (declared-name "FlowTransferBefore") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransferBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::LinkObject"))) (kind "import") (name "LinkObject") (declared-name "LinkObject") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::LinkObject") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Part"))) (kind "import") (name "Part") (declared-name "Part") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Connections"))))
    (element (id (node (document "d0") (qualified-name "Connections::actions"))) (kind "import") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind "connection def") (name "binaryConnections") (declared-name "binaryConnections") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "connections")) (specializes (reference "binaryLinkObjects")))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryConnections::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Connections::binaryConnections"))))
    (element (id (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (kind "import") (name "binaryLinkObjects") (declared-name "binaryLinkObjects") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::binaryLinkObjects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::connections"))) (kind "connection def") (name "connections") (declared-name "connections") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "linkObjects")) (specializes (reference "parts")))))
    (element (id (node (document "d0") (qualified-name "Connections::connections::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Connections::connections"))))
    (element (id (node (document "d0") (qualified-name "Connections::flowTransfers"))) (kind "import") (name "flowTransfers") (declared-name "flowTransfers") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfers") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::flowTransfersBefore"))) (kind "import") (name "flowTransfersBefore") (declared-name "flowTransfersBefore") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfersBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::linkObjects"))) (kind "import") (name "linkObjects") (declared-name "linkObjects") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::linkObjects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::parts"))) (kind "import") (name "parts") (declared-name "parts") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Connections::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (parent (node (document "d0") (qualified-name "Connections"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Connections::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 0)) (authored-target "BinaryLinkObject") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::BinaryLinkObject")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1)) (authored-target "Connection") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Connection")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::source"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryLinkObject::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Anything")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryConnection::target"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryLinkObject::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::BinaryLinkObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::BinaryLinkObject") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 0)) (authored-target "LinkObject") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::LinkObject")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 1)) (authored-target "Part") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::FlowTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::FlowTransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransferBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::LinkObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::LinkObject") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0)) (authored-target "connections") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::connections")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 1)) (authored-target "binaryLinkObjects") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::binaryLinkObjects")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::binaryLinkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::binaryLinkObjects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 0)) (authored-target "linkObjects") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::linkObjects")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 1)) (authored-target "parts") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections::parts")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections::flowTransfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfers") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::flowTransfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfersBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::linkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::linkObjects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Connections::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 26 54) (end 26 58)) (probe (position 26 54))
      (reference
        (source (document "d0") (qualified-name "Connections::Connection"))
        (kind specialization) (ordinal 1) (authored-target "Part")
        (range (start 26 54) (end 26 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::Part") (range (start 21 4) (end 21 31)))
        )
      )
    )
    (query (range (start 0 13) (end 0 18)) (probe (position 0 13))
      (reference
        (source (document "d0") (qualified-name "Connections::connections"))
        (kind specialization) (ordinal 1) (authored-target "parts")
        (range (start 0 13) (end 0 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::parts") (range (start 22 4) (end 22 32)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Connections::binaryConnections"))
        (kind specialization) (ordinal 1) (authored-target "binaryLinkObjects")
        (range (start 0 13) (end 0 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::binaryLinkObjects") (range (start 13 4) (end 13 46)))
        )
      )
    )
    (query (range (start 26 42) (end 26 52)) (probe (position 26 42))
      (reference
        (source (document "d0") (qualified-name "Connections::Connection"))
        (kind specialization) (ordinal 0) (authored-target "LinkObject")
        (range (start 26 42) (end 26 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::LinkObject") (range (start 10 4) (end 10 39)))
        )
      )
    )
    (query (range (start 34 66) (end 34 76)) (probe (position 34 66))
      (reference
        (source (document "d0") (qualified-name "Connections::BinaryConnection"))
        (kind specialization) (ordinal 1) (authored-target "Connection")
        (range (start 34 66) (end 34 76))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::Connection") (range (start 26 4) (end 26 277)))
        )
      )
    )
    (query (range (start 0 0) (end 0 11)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Connections::binaryConnections"))
        (kind specialization) (ordinal 0) (authored-target "connections")
        (range (start 0 0) (end 0 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::connections") (range (start 46 4) (end 46 196)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Connections::connections"))
        (kind specialization) (ordinal 0) (authored-target "linkObjects")
        (range (start 0 0) (end 0 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::linkObjects") (range (start 11 4) (end 11 40)))
        )
      )
    )
    (query (range (start 21 19) (end 21 30)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Part"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
        (range (start 21 19) (end 21 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 19) (end 22 31)) (probe (position 22 19))
      (reference
        (source (document "d0") (qualified-name "Connections::parts"))
        (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
        (range (start 22 19) (end 22 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 33)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 19) (end 7 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 19) (end 23 34)) (probe (position 23 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 23 19) (end 23 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 19) (end 24 35)) (probe (position 24 19))
      (reference
        (source (document "d0") (qualified-name "Connections::actions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
        (range (start 24 19) (end 24 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 48) (end 34 64)) (probe (position 34 48))
      (reference
        (source (document "d0") (qualified-name "Connections::BinaryConnection"))
        (kind specialization) (ordinal 0) (authored-target "BinaryLinkObject")
        (range (start 34 48) (end 34 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::BinaryLinkObject") (range (start 12 4) (end 12 45)))
        )
      )
    )
    (query (range (start 0 13) (end 0 30)) (probe (position 0 13))
      (reference
        (source (document "d0") (qualified-name "Connections::connections"))
        (kind specialization) (ordinal 1) (authored-target "parts")
        (range (start 0 13) (end 0 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::parts") (range (start 22 4) (end 22 32)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Connections::binaryConnections"))
        (kind specialization) (ordinal 1) (authored-target "binaryLinkObjects")
        (range (start 0 13) (end 0 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections::binaryLinkObjects") (range (start 13 4) (end 13 46)))
        )
      )
    )
    (query (range (start 10 19) (end 10 38)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "Connections::LinkObject"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::LinkObject")
        (range (start 10 19) (end 10 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 19) (end 14 38)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Transfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
        (range (start 14 19) (end 14 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 39)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "Connections::linkObjects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::linkObjects")
        (range (start 11 19) (end 11 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 39)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "Connections::transfers"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfers")
        (range (start 15 19) (end 15 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 19) (end 20 40)) (probe (position 20 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 20 19) (end 20 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 42)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Connections::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 19) (end 8 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 42)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "Connections::FlowTransfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransfer")
        (range (start 16 19) (end 16 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 19) (end 17 43)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "Connections::flowTransfers"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfers")
        (range (start 17 19) (end 17 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 33) (end 42 57)) (probe (position 42 33))
      (reference
        (source (document "d0") (qualified-name "Connections::BinaryConnection::source"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryLinkObject::source")
        (range (start 42 33) (end 42 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 33) (end 43 57)) (probe (position 43 33))
      (reference
        (source (document "d0") (qualified-name "Connections::BinaryConnection::target"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryLinkObject::target")
        (range (start 43 33) (end 43 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 44)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "Connections::BinaryLinkObject"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::BinaryLinkObject")
        (range (start 12 19) (end 12 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 45)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Connections::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 9 19) (end 9 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 19) (end 13 45)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "Connections::binaryLinkObjects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::binaryLinkObjects")
        (range (start 13 19) (end 13 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 19) (end 18 48)) (probe (position 18 19))
      (reference
        (source (document "d0") (qualified-name "Connections::FlowTransferBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransferBefore")
        (range (start 18 19) (end 18 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 19) (end 19 49)) (probe (position 19 19))
      (reference
        (source (document "d0") (qualified-name "Connections::flowTransfersBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfersBefore")
        (range (start 19 19) (end 19 49))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
