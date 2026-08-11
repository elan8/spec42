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
  (document "ports.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 45) (end 22 68))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ed91bc6097083815cee302c9513da81cc772dc0c008b977acc961f669283973e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Ports"))) (kind "package") (name "Ports") (declared-name "Ports"))
    (element (id (node (document "d0") (qualified-name "Ports::Object"))) (kind "import") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Ports"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Ports::Port"))) (kind "port def") (name "Port") (declared-name "Port") (parent (node (document "d0") (qualified-name "Ports"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Object")))))
    (element (id (node (document "d0") (qualified-name "Ports::Port::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Ports::Port"))))
    (element (id (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind "port") (name "subports") (declared-name "subports") (parent (node (document "d0") (qualified-name "Ports::Port"))) (authored (membership (kind Feature)) (relationships (typing (reference "Port")) (subsetting (reference "ports")) (subsetting (reference "timeEnclosedOccurrences")))))
    (element (id (node (document "d0") (qualified-name "Ports::Port::subports::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Ports::Port::subports"))))
    (element (id (node (document "d0") (qualified-name "Ports::Port::~Port"))) (kind "conjugated port definition") (name "~Port") (declared-name "~Port") (parent (node (document "d0") (qualified-name "Ports::Port"))))
    (element (id (node (document "d0") (qualified-name "Ports::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Ports"))))
    (element (id (node (document "d0") (qualified-name "Ports::objects"))) (kind "import") (name "objects") (declared-name "objects") (parent (node (document "d0") (qualified-name "Ports"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Ports::ports"))) (kind "port def") (name "ports") (declared-name "ports") (parent (node (document "d0") (qualified-name "Ports"))) (authored (membership (kind Owning)) (relationships (specializes (reference "objects")))))
    (element (id (node (document "d0") (qualified-name "Ports::ports::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Ports::ports"))))
    (element (id (node (document "d0") (qualified-name "Ports::ports::~ports"))) (kind "conjugated port definition") (name "~ports") (declared-name "~ports") (parent (node (document "d0") (qualified-name "Ports::ports"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Ports::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Ports::Port"))) (kind specialization) (ordinal 0)) (authored-target "Object") (outcome (status resolved) (target (node (document "d0") (qualified-name "Ports::Object")))))
    (reference (id (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (outcome (status resolved) (target (node (document "d0") (qualified-name "Ports::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 0)) (authored-target "ports") (outcome (status resolved) (target (node (document "d0") (qualified-name "Ports::ports")))))
    (reference (id (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 1)) (authored-target "timeEnclosedOccurrences") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Ports::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Ports::ports"))) (kind specialization) (ordinal 0)) (authored-target "objects") (outcome (status resolved) (target (node (document "d0") (qualified-name "Ports::objects")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Ports::Port"))) (target (node (document "d0") (qualified-name "Ports::Object"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Ports::Port"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (target (node (document "d0") (qualified-name "Ports::Port"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (target (node (document "d0") (qualified-name "Ports::ports"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Ports::Port::subports"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Ports::ports"))) (target (node (document "d0") (qualified-name "Ports::objects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Ports::ports"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 38) (end 22 43)) (probe (position 22 38))
      (reference
        (source (document "d0") (qualified-name "Ports::Port::subports"))
        (kind subsetting) (ordinal 0) (authored-target "ports")
        (range (start 22 38) (end 22 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Ports::ports") (range (start 47 4) (end 47 156)))
        )
      )
    )
    (query (range (start 10 30) (end 10 36)) (probe (position 10 30))
      (reference
        (source (document "d0") (qualified-name "Ports::Port"))
        (kind specialization) (ordinal 0) (authored-target "Object")
        (range (start 10 30) (end 10 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Ports::Object") (range (start 7 4) (end 7 35)))
        )
      )
    )
    (query (range (start 0 0) (end 0 7)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Ports::ports"))
        (kind specialization) (ordinal 0) (authored-target "objects")
        (range (start 0 0) (end 0 7))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Ports::objects") (range (start 8 4) (end 8 36)))
        )
      )
    )
    (query (range (start 7 19) (end 7 34)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Ports::Object"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
        (range (start 7 19) (end 7 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 35)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Ports::objects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
        (range (start 8 19) (end 8 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 45) (end 22 68)) (probe (position 22 45))
      (reference
        (source (document "d0") (qualified-name "Ports::Port::subports"))
        (kind subsetting) (ordinal 1) (authored-target "timeEnclosedOccurrences")
        (range (start 22 45) (end 22 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
