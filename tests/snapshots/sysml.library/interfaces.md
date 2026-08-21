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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 19) (end 6 42))
      )
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 54) (end 28 58))
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
        (severity error)
        (code "recovered_interface_def_body_element")
        (source "parser")
        (range (start 71 8) (end 72 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 71 8) (end 72 8))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:d6d9003112ba8ccb2d20b49c088947a95fa9449a7658ba20dd58f05186fc0ca2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n     * This package defines the base types for interfaces and related structural elements in the SysML language.\n     "))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::Connection") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::connections") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::BinaryConnection") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Connections::binaryConnections") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Ports::Port") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::excludingAt") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::selectOne") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind interface-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * BinaryInterface is the most general class of links between two Ports \n         * on Parts within some containing structure. BinaryInterface is the base \n         * type of all binary InterfaceDefinitions which have exactly two ends. \n         * \n         * Transfers outgoing from each participant Port of a BinaryInterface may be \n         * targeted to the other participant Port (depending on any other Interfaces \n         * in which the Port is also participating).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Interface")) (specialization (reference "BinaryConnection")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers reference ordered nonunique) (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Interface::participant")) (redefinition (reference "BinaryConnection::participant")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind interface-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * Interface is the most general class of links between Ports on Parts \n         * within some containing structure. Interface is the base type of all\n         * InterfaceDefinitions. \n         * \n         * Transfers outgoing from any one of the participant Ports of an Interface \n         * may be targeted to one of the other participant Ports (depending on any \n         * other Interfaces in which the Port is also participating).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Connection")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers reference ordered nonunique) (multiplicity (lower 2) (upper unbounded))) (documentation (doc (text "\n             * The participants of an Interface must be Ports. The interfacingPorts of\n             * each participant Port include all the other participants in the Interface.\n             "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Port")) (redefinition (reference "participant")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants"))))) (kind ref) (membership (kind feature) (visibility protected)) (facts (modifiers nonunique) (multiplicity (lower 1) (upper unbounded))) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility protected)) (relationships (featureTyping (reference "Port")) (subsetting (reference "interfacingPorts")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "thisParticipant"))))) (kind ref) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind interface-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * interfaces is the base feature of all binary InterfaceUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "interfaces")) (specialization (reference "binaryConnections")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind calc-def) (membership (kind owning) (visibility private)) (documentation (doc (text "\n         * Return a sequence that is the same as the input sequence, but with a single\n         * instance of a given value removed. The given value must be in the input sequence.\n         "))) (authored (membership (kind owning) (visibility private)) (relationships (expressionOperand (reference "seq")) (expressionOperand (reference "position")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind attribute) (membership (kind feature) (visibility private)) (facts (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural")) (expressionOperand (reference "seq")) (invocationCallee (reference "size")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers ordered nonunique) (direction in) (multiplicity (lower 1) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "seq")))))
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind interface-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n         * interfaces is the base feature of all InterfaceUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "connections")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::Connection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::connections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::BinaryConnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Connections::binaryConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Ports::Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::excludingAt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::selectOne")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0))
      (authored-target "Interface")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1))
      (authored-target "BinaryConnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "Interface::participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "BinaryConnection::participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0))
      (authored-target "Connection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "participant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Port")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants"))))) (kind subsetting) (ordinal 0))
      (authored-target "interfacingPorts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "thisParticipant"))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0))
      (authored-target "interfaces")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1))
      (authored-target "binaryConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 1))
      (authored-target "position")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind expressionOperand) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind subsetting) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0))
      (authored-target "connections")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface")))
      (supertype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface")))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))
      (subtype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants")))))
      (featured-by (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "thisParticipant")))))
      (featured-by (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces")))
      (supertype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position")))
      (featured-by (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce")))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))
      (featured-by (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce")))
      (subtype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value")))
      (featured-by (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce")))
      (supertype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")))
      (subtype (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/interfaces.md") (range (start 6 19) (end 6 42)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::Connection")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 7 19) (end 7 43)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::connections")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 8 19) (end 8 48)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::BinaryConnection")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 9 19) (end 9 49)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Connections::binaryConnections")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 10 19) (end 10 30)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Ports::Port")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 12 19) (end 12 40)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 13 19) (end 13 42)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 14 19) (end 14 49)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludingAt")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 15 19) (end 15 46)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::selectOne")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 17 19) (end 17 46)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 57 46) (end 57 55)) (probe (position 57 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0) (authored-target "Interface")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 57 57) (end 57 73)) (probe (position 57 57))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1) (authored-target "BinaryConnection")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 69 21) (end 69 43)) (probe (position 69 21))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "Interface::participant")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 69 45) (end 69 74)) (probe (position 69 45))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "BinaryInterface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "BinaryConnection::participant")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 32 40) (end 32 50)) (probe (position 32 40))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0) (authored-target "Connection")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 44 35) (end 44 39)) (probe (position 44 35))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 44 21) (end 44 32)) (probe (position 44 21))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "participant")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 52 46) (end 52 50)) (probe (position 52 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants"))))) (kind featureTyping) (ordinal 0) (authored-target "Port")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 52 71) (end 52 87)) (probe (position 52 71))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "otherParticipants"))))) (kind subsetting) (ordinal 0) (authored-target "interfacingPorts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 51 46) (end 51 50)) (probe (position 51 46))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (path (named (kind library-package) (name "Interfaces")) (named (kind interface-def) (name "Interface")) (anonymous (kind port) (ordinal 0)) (named (kind ref) (name "thisParticipant"))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 82 76) (end 82 86)) (probe (position 82 76))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0) (authored-target "interfaces")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 82 88) (end 82 105)) (probe (position 82 88))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1) (authored-target "binaryConnections")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 29 8) (end 29 11)) (probe (position 29 8))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 29 25) (end 29 33)) (probe (position 29 25))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce"))) (kind expressionOperand) (ordinal 1) (authored-target "position")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 28 37) (end 28 44)) (probe (position 28 37))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 28 59) (end 28 62)) (probe (position 28 59))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind expressionOperand) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 28 54) (end 28 58)) (probe (position 28 54))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::position"))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 26 23) (end 26 26)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::value"))) (kind subsetting) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::excludingOnce::seq")))))
    )
  )
  (query (document "memory://snapshot/interfaces.md") (range (start 75 64) (end 75 75)) (probe (position 75 64))
    (reference (id (source (node (document "memory://snapshot/interfaces.md") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0) (authored-target "connections")
      (outcome (status unresolved)))
    )
  )
)
~~~
