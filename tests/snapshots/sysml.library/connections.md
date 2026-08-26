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
  (document "memory://snapshot/connections.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 19) (end 7 33))
      )
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 42) (end 26 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 54) (end 26 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 48) (end 34 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 20) (end 42 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 20) (end 43 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 46 67) (end 46 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 46 80) (end 46 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 53 86) (end 53 103))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:417d85afc024acf2825e49af8e164fb1cc2aa4f80384f96b4a908553b1aefca7") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n     * This package defines the base types for connections and related structural elements \n     * in the SysML language.\n     "))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensDuring") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::LinkObject") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::linkObjects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::BinaryLinkObject") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::binaryLinkObjects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::Transfer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::transfers") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::FlowTransfer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::flowTransfers") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::FlowTransferBefore") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::flowTransfersBefore") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 14))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 15))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 16))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 17))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * BinaryConnection is the most general class of binary links between two things \n         * within some containing structure. BinaryConnection is the base type of all \n         * ConnectionDefinitions with exactly two ends.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryLinkObject")) (specialization (reference "Connection")))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * Connection is the most general class of links between things within some \n         * containing structure. Connection is the base type of all ConnectionDefinitions.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LinkObject")) (specialization (reference "Part")))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * binaryConnections is the base feature of all binary ConnectionUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "connections")) (specialization (reference "binaryLinkObjects")))))
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * connections is the base feature of all ConnectionUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "linkObjects")) (specialization (reference "parts")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::LinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::linkObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::BinaryLinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::binaryLinkObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::FlowTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::flowTransfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::FlowTransferBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::flowTransfersBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1))
      (authored-target "Connection")
      (outcome (status resolved) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection")))))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 0))
      (authored-target "LinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 1))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0))
      (authored-target "connections")
      (outcome (status resolved) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections")))))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 1))
      (authored-target "binaryLinkObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 0))
      (authored-target "linkObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 1))
      (authored-target "parts")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::source")))
      (featured-by (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection")))
    )
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::target")))
      (featured-by (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection")))
    )
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection")))
      (subtype (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections")))
      (supertype (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections")))
      (subtype (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connections.md") (range (start 7 19) (end 7 33)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 8 19) (end 8 42)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 9 19) (end 9 45)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 10 19) (end 10 38)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::LinkObject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 11 19) (end 11 39)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::linkObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 12 19) (end 12 44)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::BinaryLinkObject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 13 19) (end 13 45)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::binaryLinkObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 14 19) (end 14 38)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 15 19) (end 15 39)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 16 19) (end 16 42)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 17 19) (end 17 43)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfers")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 18 19) (end 18 48)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransferBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 19 19) (end 19 49)) (probe (position 19 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfersBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 20 19) (end 20 40)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 21 19) (end 21 30)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 22 19) (end 22 31)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 23 19) (end 23 34)) (probe (position 23 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 24 19) (end 24 35)) (probe (position 24 19))
    (reference (id (source (node (document "memory://snapshot/connections.md") (path (named (kind library-package) (name "Connections")) (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 34 48) (end 34 64)) (probe (position 34 48))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 0) (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 34 66) (end 34 76)) (probe (position 34 66))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection"))) (kind specialization) (ordinal 1) (authored-target "Connection")
      (outcome (status resolved) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection")))))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 42 20) (end 42 28)) (probe (position 42 20))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 43 20) (end 43 28)) (probe (position 43 20))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 26 42) (end 26 52)) (probe (position 26 42))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 0) (authored-target "LinkObject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 26 54) (end 26 58)) (probe (position 26 54))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::Connection"))) (kind specialization) (ordinal 1) (authored-target "Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 53 73) (end 53 84)) (probe (position 53 73))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 0) (authored-target "connections")
      (outcome (status resolved) (target (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections")))))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 53 86) (end 53 103)) (probe (position 53 86))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::binaryConnections"))) (kind specialization) (ordinal 1) (authored-target "binaryLinkObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 46 67) (end 46 78)) (probe (position 46 67))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 0) (authored-target "linkObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/connections.md") (range (start 46 80) (end 46 85)) (probe (position 46 80))
    (reference (id (source (node (document "memory://snapshot/connections.md") (qualified-name "Connections::connections"))) (kind specialization) (ordinal 1) (authored-target "parts")
      (outcome (status unresolved)))
    )
  )
)
~~~
