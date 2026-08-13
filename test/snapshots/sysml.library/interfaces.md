# META
~~~ini
description=Standard Library: Systems Library/Interfaces
type=file
~~~
# SOURCE
~~~sysml
standard library package Interfaces {
    doc
    /*
     * This package defines the base types for interfaces and related structural elements in the SysML language.
     */
     
    private import Connections::Connection;
    private import Connections::connections;
    private import Connections::BinaryConnection;
    private import Connections::binaryConnections;
    private import Ports::Port;
    
    private import ScalarValues::Natural;
    private import SequenceFunctions::size;
    private import SequenceFunctions::excludingAt;
    private import ControlFunctions::selectOne;
    
    private import SequenceFunctions::notEmpty;
    
    private calc def excludingOnce {
        doc
        /*
         * Return a sequence that is the same as the input sequence, but with a single
         * instance of a given value removed. The given value must be in the input sequence.
         */
        in seq[1..*] nonunique ordered; 
        in value[1] :> seq;
        
        private attribute position : Natural[1] = (1..size(seq))->selectOne{in i; seq#(i) == value};
        seq->excludingAt(position)
    }
    
    abstract interface def Interface :> Connection {
        doc
        /*
         * Interface is the most general class of links between Ports on Parts 
         * within some containing structure. Interface is the base type of all
         * InterfaceDefinitions. 
         * 
         * Transfers outgoing from any one of the participant Ports of an Interface 
         * may be targeted to one of the other participant Ports (depending on any 
         * other Interfaces in which the Port is also participating).
         */
         
        ref port :>> participant : Port [2..*] nonunique ordered {
            doc
            /*
             * The participants of an Interface must be Ports. The interfacingPorts of
             * each participant Port include all the other participants in the Interface.
             */
              
            protected ref thisParticipant :>> self;
            protected ref otherParticipants : Port [1..*] nonunique :> interfacingPorts
                default participant->excludingOnce(thisParticipant);            
        }
    }
    
    abstract interface def BinaryInterface :> Interface, BinaryConnection {
        doc
        /*
         * BinaryInterface is the most general class of links between two Ports 
         * on Parts within some containing structure. BinaryInterface is the base 
         * type of all binary InterfaceDefinitions which have exactly two ends. 
         * 
         * Transfers outgoing from each participant Port of a BinaryInterface may be 
         * targeted to the other participant Port (depending on any other Interfaces 
         * in which the Port is also participating).
         */
        
        ref port :>> Interface::participant, BinaryConnection::participant[2] nonunique ordered;
     
        end port source: Port :>> BinaryConnection::source; 
        end port target: Port :>> BinaryConnection::target;
    }
    
    abstract interface interfaces: Interface[0..*] nonunique :> connections {
        doc
        /*
         * interfaces is the base feature of all InterfaceUsages.
         */
    }
    
    abstract interface binaryInterfaces: BinaryInterface[0..*] nonunique :> interfaces, binaryConnections {
        doc
        /*
         * interfaces is the base feature of all binary InterfaceUsages.
         */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/interfaces.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 19) (end 13 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 37) (end 28 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 28 50) (end 28 99))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 8) (end 29 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 40) (end 32 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 21) (end 44 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 35) (end 44 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 46) (end 51 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 46) (end 52 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 71) (end 52 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 57 57) (end 57 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 21) (end 69 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 45) (end 69 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 25) (end 71 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 25) (end 72 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 75 64) (end 75 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 82 88) (end 82 105))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d6d9003112ba8ccb2d20b49c088947a95fa9449a7658ba20dd58f05186fc0ca2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::Connection") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::connections") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::BinaryConnection") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::binaryConnections") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Ports::Port") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::excludingAt") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::selectOne") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Interface")) (specialization (reference "BinaryConnection"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Interface::participant")) (redefinition (reference "BinaryConnection::participant"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Connection"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (redefinition (reference "participant"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::otherParticipants"))) (kind ref) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (featureTyping (reference "Port")) (subsetting (reference "interfacingPorts"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::thisParticipant"))) (kind ref) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "interfaces")) (specialization (reference "binaryConnections"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind calc-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "seq") (direction in))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "connections"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::Connection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::connections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::BinaryConnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::binaryConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Ports::Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::excludingAt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::selectOne")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0))
      (authored-target "Interface")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1))
      (authored-target "BinaryConnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "Interface::participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "BinaryConnection::participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0))
      (authored-target "Connection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::otherParticipants"))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::otherParticipants"))) (kind subsetting) (ordinal 0))
      (authored-target "interfacingPorts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::thisParticipant"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0))
      (authored-target "interfaces")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1))
      (authored-target "binaryConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0))
      (authored-target "connections")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/interfaces.md") (range (start 6 19) (end 6 42)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::Connection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 7 19) (end 7 43)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::connections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 8 19) (end 8 48)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::BinaryConnection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 9 19) (end 9 49)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::binaryConnections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 10 19) (end 10 30)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Ports::Port")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 12 19) (end 12 40)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 13 19) (end 13 42)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 14 19) (end 14 49)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludingAt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 15 19) (end 15 46)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::selectOne")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 17 19) (end 17 46)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 57 46) (end 57 55)) (probe (position 57 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0) (authored-target "Interface")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 57 57) (end 57 73)) (probe (position 57 57))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1) (authored-target "BinaryConnection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 69 21) (end 69 43)) (probe (position 69 21))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "Interface::participant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 69 45) (end 69 74)) (probe (position 69 45))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "BinaryConnection::participant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 71 25) (end 71 29)) (probe (position 71 25))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 72 25) (end 72 29)) (probe (position 72 25))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 32 40) (end 32 50)) (probe (position 32 40))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0) (authored-target "Connection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 44 35) (end 44 39)) (probe (position 44 35))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 44 21) (end 44 32)) (probe (position 44 21))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 52 46) (end 52 50)) (probe (position 52 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::otherParticipants"))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 52 71) (end 52 87)) (probe (position 52 71))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::otherParticipants"))) (kind subsetting) (ordinal 0) (authored-target "interfacingPorts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 51 46) (end 51 50)) (probe (position 51 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface::::thisParticipant"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 82 76) (end 82 86)) (probe (position 82 76))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0) (authored-target "interfaces")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")))))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 82 88) (end 82 105)) (probe (position 82 88))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1) (authored-target "binaryConnections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 28 37) (end 28 44)) (probe (position 28 37))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 26 23) (end 26 26)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind featureTyping) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 75 64) (end 75 75)) (probe (position 75 64))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0) (authored-target "connections")
      (outcome (status unresolved)))
  )
)
~~~
