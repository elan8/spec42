# META
~~~ini
description=Standard Library: Systems Library/Ports
type=file
~~~
# SOURCE
~~~sysml
standard library package Ports {
    doc
    /*
     * This package defines the base types for ports and related structural elements 
     * in the SysML language.
     */

    private import Objects::Object;
    private import Objects::objects;
    
    abstract port def Port :> Object {
        doc
        /*
         * Port is the most general class of objects that represent connection points
         * for interacting with a Part. Port is the base type of all PortDefinitions.
         * 
         * Transfers outgoing from a Port are always targeted to a Port connected to
         * the original Port by an Interface.
         */
    
        ref self: Port :>> Object::self;
        
        port subports: Port [0..*] :> ports, timeEnclosedOccurrences {
            doc
            /*
             * The Ports that are subports of this Port.
             */
        }
        
        abstract ref port interfacingPorts : Port[0..*] nonunique :> ports {
            doc
            /*
             * Ports that are connected to this Port by an Interface.
             */
        }
        
        ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf {
            doc
            /* 
             * The target of each of the outgoingTransfersFromSelf of a Port must be an interfacingPort.
             */
             
             end ref source;
             end ref target;
        }
    }
    
    abstract port ports : Port[0..*] nonunique :> objects {
        doc
        /*
         * ports is the base feature of all PortUsages.
         */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/ports.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 30) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 27) (end 20 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 45) (end 22 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 16) (end 36 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 45) (end 36 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 50) (end 47 57))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d698ce266ca31d5ae2c3fe2123880b7d2dcefc42a0bf0f2835761452033c2425") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n     * This package defines the base types for ports and related structural elements \n     * in the SysML language.\n     "))))
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Object") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::objects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (kind port-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * Port is the most general class of objects that represent connection points\n         * for interacting with a Part. Port is the base type of all PortDefinitions.\n         * \n         * Transfers outgoing from a Port are always targeted to a Port connected to\n         * the original Port by an Interface.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (documentation (doc (text " \n             * The target of each of the outgoingTransfersFromSelf of a Port must be an interfacingPort.\n             "))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "interfacingPorts::incomingTransfersToSelf")) (redefinition (reference "outgoingTransfersFromSelf")))))
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)) (named (kind connection) (name "source"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)) (named (kind connection) (name "target"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind ref) (membership (kind feature) (visibility default)) (facts (modifiers abstract nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n             * Ports that are connected to this Port by an Interface.\n             "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (subsetting (reference "ports")))))
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (redefinition (reference "Object::self")))))
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind port) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n             * The Ports that are subports of this Port.\n             "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (subsetting (reference "ports")) (subsetting (reference "timeEnclosedOccurrences")))))
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports"))) (kind port-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * ports is the base feature of all PortUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "objects")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "interfacingPorts::incomingTransfersToSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "outgoingTransfersFromSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind subsetting) (ordinal 0))
      (authored-target "ports")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")))))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Object::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 0))
      (authored-target "ports")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")))))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 1))
      (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports"))) (kind specialization) (ordinal 0))
      (authored-target "objects")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))
      (subtype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts")) (scopes any))
      (subtype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self")) (scopes any))
      (subtype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)) (named (kind connection) (name "source")))))
      (featured-by (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)) (named (kind connection) (name "target")))))
      (featured-by (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts")))
      (featured-by (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))
      (type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (source direct))
      (supertype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self")))
      (featured-by (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))
      (type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (source direct))
      (supertype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports")))
      (featured-by (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))
      (type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (source direct))
      (supertype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")))
      (subtype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts")) (scopes any feature))
      (subtype (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ports.md") (range (start 7 19) (end 7 34)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 8 19) (end 8 35)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 10 30) (end 10 36)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 36 45) (end 36 85)) (probe (position 36 45))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "interfacingPorts::incomingTransfersToSelf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 36 16) (end 36 41)) (probe (position 36 16))
    (reference (id (source (node (document "memory://snapshot/ports.md") (path (named (kind library-package) (name "Ports")) (named (kind port-def) (name "Port")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "outgoingTransfersFromSelf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 29 45) (end 29 49)) (probe (position 29 45))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 29 69) (end 29 74)) (probe (position 29 69))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::interfacingPorts"))) (kind subsetting) (ordinal 0) (authored-target "ports")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")))))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 20 18) (end 20 22)) (probe (position 20 18))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 20 27) (end 20 39)) (probe (position 20 27))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::self"))) (kind redefinition) (ordinal 0) (authored-target "Object::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 22 23) (end 22 27)) (probe (position 22 23))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port")))))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 22 38) (end 22 43)) (probe (position 22 38))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 0) (authored-target "ports")
      (outcome (status resolved) (target (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports")))))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 22 45) (end 22 68)) (probe (position 22 45))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 1) (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ports.md") (range (start 47 50) (end 47 57)) (probe (position 47 50))
    (reference (id (source (node (document "memory://snapshot/ports.md") (qualified-name "Ports::ports"))) (kind specialization) (ordinal 0) (authored-target "objects")
      (outcome (status unresolved)))
    )
  )
)
~~~
