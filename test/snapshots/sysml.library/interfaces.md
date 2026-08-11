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
  (document "interfaces.md"
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
        (range (start 25 8) (end 25 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 8) (end 26 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 21) (end 44 32))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 34) (end 71 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 34) (end 72 58))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b6fb19a15252373924cf0a9c348559de316184d0d2a9d143b9f450bd506089c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interfaces"))) (kind "package") (name "Interfaces") (declared-name "Interfaces") (range (start (line 0) (character 0)) (end (line 0) (character 3467))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryConnection"))) (kind "import") (name "BinaryConnection") (declared-name "BinaryConnection") (range (start (line 8) (character 4)) (end (line 8) (character 49))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "Connections::BinaryConnection") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 48))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (kind "interface def") (name "BinaryInterface") (declared-name "BinaryInterface") (range (start (line 57) (character 4)) (end (line 57) (character 831))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Interface") (range (start (line 57) (character 46)) (end (line 57) (character 55)))) (specializes (reference "BinaryConnection") (range (start (line 57) (character 57)) (end (line 57) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::"))) (kind "ref") (name "") (range (start (line 69) (character 8)) (end (line 69) (character 96))) (parent (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Interface::participant") (range (start (line 69) (character 21)) (end (line 69) (character 43)))) (redefinition (reference "BinaryConnection::participant") (range (start (line 69) (character 45)) (end (line 69) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::_documentation"))) (kind "documentation") (name "") (range (start (line 57) (character 4)) (end (line 57) (character 831))) (parent (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 71) (character 8)) (end (line 71) (character 59))) (parent (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (authored (relationships (typing (reference "Port") (range none)) (redefinition (reference "BinaryConnection::source") (range (start (line 71) (character 34)) (end (line 71) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 72) (character 8)) (end (line 72) (character 59))) (parent (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (authored (relationships (typing (reference "Port") (range none)) (redefinition (reference "BinaryConnection::target") (range (start (line 72) (character 34)) (end (line 72) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Connection"))) (kind "import") (name "Connection") (declared-name "Connection") (range (start (line 6) (character 4)) (end (line 6) (character 43))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "Connections::Connection") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Interface"))) (kind "interface def") (name "Interface") (declared-name "Interface") (range (start (line 32) (character 4)) (end (line 32) (character 1084))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Connection") (range (start (line 32) (character 40)) (end (line 32) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Interface::"))) (kind "ref") (name "") (range (start (line 44) (character 8)) (end (line 44) (character 536))) (parent (node (document "d0") (qualified-name "Interfaces::Interface"))) (authored (membership (kind Feature)) (relationships (typing (reference "Port") (range (start (line 44) (character 34)) (end (line 44) (character 39)))) (redefinition (reference "participant") (range (start (line 44) (character 21)) (end (line 44) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Interface::_documentation"))) (kind "documentation") (name "") (range (start (line 32) (character 4)) (end (line 32) (character 1084))) (parent (node (document "d0") (qualified-name "Interfaces::Interface"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 12) (character 4)) (end (line 12) (character 41))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::Port"))) (kind "import") (name "Port") (declared-name "Port") (range (start (line 10) (character 4)) (end (line 10) (character 31))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "Ports::Port") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3467))) (parent (node (document "d0") (qualified-name "Interfaces"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::binaryConnections"))) (kind "import") (name "binaryConnections") (declared-name "binaryConnections") (range (start (line 9) (character 4)) (end (line 9) (character 50))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "Connections::binaryConnections") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 49))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (kind "interface def") (name "binaryInterfaces") (declared-name "binaryInterfaces") (range (start (line 82) (character 4)) (end (line 82) (character 221))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Owning)) (relationships (specializes (reference "interfaces") (range (start (line 0) (character 0)) (end (line 0) (character 10)))) (specializes (reference "binaryConnections") (range (start (line 0) (character 12)) (end (line 0) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::binaryInterfaces::_documentation"))) (kind "documentation") (name "") (range (start (line 82) (character 4)) (end (line 82) (character 221))) (parent (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::connections"))) (kind "import") (name "connections") (declared-name "connections") (range (start (line 7) (character 4)) (end (line 7) (character 44))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "Connections::connections") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::excludingAt"))) (kind "import") (name "excludingAt") (declared-name "excludingAt") (range (start (line 14) (character 4)) (end (line 14) (character 50))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::excludingAt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 49))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::excludingOnce"))) (kind "calc def") (name "excludingOnce") (declared-name "excludingOnce") (range (start (line 19) (character 4)) (end (line 19) (character 471))) (parent (node (document "d0") (qualified-name "Interfaces"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::_documentation"))) (kind "documentation") (name "") (range (start (line 19) (character 4)) (end (line 19) (character 471))) (parent (node (document "d0") (qualified-name "Interfaces::excludingOnce"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::seq"))) (kind "in out parameter") (name "seq") (declared-name "seq") (range (start (line 25) (character 8)) (end (line 25) (character 39))) (parent (node (document "d0") (qualified-name "Interfaces::excludingOnce"))) (authored (relationships (typing (reference "seq[1..*] nonunique ordered") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::value"))) (kind "in out parameter") (name "value") (declared-name "value") (range (start (line 26) (character 8)) (end (line 26) (character 27))) (parent (node (document "d0") (qualified-name "Interfaces::excludingOnce"))) (authored (relationships (typing (reference "value[1] :> seq") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interfaces::interfaces"))) (kind "interface def") (name "interfaces") (declared-name "interfaces") (range (start (line 75) (character 4)) (end (line 75) (character 184))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Owning)) (relationships (specializes (reference "connections") (range (start (line 0) (character 0)) (end (line 0) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::interfaces::_documentation"))) (kind "documentation") (name "") (range (start (line 75) (character 4)) (end (line 75) (character 184))) (parent (node (document "d0") (qualified-name "Interfaces::interfaces"))))
    (element (id (node (document "d0") (qualified-name "Interfaces::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (range (start (line 17) (character 4)) (end (line 17) (character 47))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::selectOne"))) (kind "import") (name "selectOne") (declared-name "selectOne") (range (start (line 15) (character 4)) (end (line 15) (character 47))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::selectOne") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Interfaces::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 13) (character 4)) (end (line 13) (character 43))) (parent (node (document "d0") (qualified-name "Interfaces"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 19)) (end (line 13) (character 42))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryConnection"))) (kind membershipImport) (ordinal 0)) (authored-target "Connections::BinaryConnection") (range (start (line 8) (character 19)) (end (line 8) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0)) (authored-target "Interface") (range (start (line 57) (character 46)) (end (line 57) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::Interface")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1)) (authored-target "BinaryConnection") (range (start (line 57) (character 57)) (end (line 57) (character 73))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::BinaryConnection")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::"))) (kind redefinition) (ordinal 0)) (authored-target "Interface::participant") (range (start (line 69) (character 21)) (end (line 69) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::"))) (kind redefinition) (ordinal 1)) (authored-target "BinaryConnection::participant") (range (start (line 69) (character 45)) (end (line 69) (character 74))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryConnection::source") (range (start (line 71) (character 34)) (end (line 71) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryConnection::target") (range (start (line 72) (character 34)) (end (line 72) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Connection"))) (kind membershipImport) (ordinal 0)) (authored-target "Connections::Connection") (range (start (line 6) (character 19)) (end (line 6) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0)) (authored-target "Connection") (range (start (line 32) (character 40)) (end (line 32) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::Connection")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Interface::"))) (kind featureTyping) (ordinal 0)) (authored-target "Port") (range (start (line 44) (character 34)) (end (line 44) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::Port")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Interface::"))) (kind redefinition) (ordinal 0)) (authored-target "participant") (range (start (line 44) (character 21)) (end (line 44) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 12) (character 19)) (end (line 12) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::Port"))) (kind membershipImport) (ordinal 0)) (authored-target "Ports::Port") (range (start (line 10) (character 19)) (end (line 10) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::binaryConnections"))) (kind membershipImport) (ordinal 0)) (authored-target "Connections::binaryConnections") (range (start (line 9) (character 19)) (end (line 9) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0)) (authored-target "interfaces") (range (start (line 0) (character 0)) (end (line 0) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::interfaces")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1)) (authored-target "binaryConnections") (range (start (line 0) (character 12)) (end (line 0) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::binaryConnections")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::connections"))) (kind membershipImport) (ordinal 0)) (authored-target "Connections::connections") (range (start (line 7) (character 19)) (end (line 7) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::excludingAt"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::excludingAt") (range (start (line 14) (character 19)) (end (line 14) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::excludingOnce::seq"))) (kind featureTyping) (ordinal 0)) (authored-target "seq[1..*] nonunique ordered") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::excludingOnce::value"))) (kind featureTyping) (ordinal 0)) (authored-target "value[1] :> seq") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0)) (authored-target "connections") (range (start (line 0) (character 0)) (end (line 0) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interfaces::connections")))))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (range (start (line 17) (character 19)) (end (line 17) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::selectOne"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::selectOne") (range (start (line 15) (character 19)) (end (line 15) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interfaces::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 13) (character 19)) (end (line 13) (character 42))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (target (node (document "d0") (qualified-name "Interfaces::BinaryConnection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (target (node (document "d0") (qualified-name "Interfaces::Interface"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (target (node (document "d0") (qualified-name "Interfaces::Port"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (target (node (document "d0") (qualified-name "Interfaces::Port"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::Interface"))) (target (node (document "d0") (qualified-name "Interfaces::Connection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::Interface"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interfaces::Interface::"))) (target (node (document "d0") (qualified-name "Interfaces::Port"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::Interface::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (target (node (document "d0") (qualified-name "Interfaces::binaryConnections"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (target (node (document "d0") (qualified-name "Interfaces::interfaces"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Interfaces::interfaces"))) (target (node (document "d0") (qualified-name "Interfaces::connections"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interfaces::interfaces"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Interfaces::excludingOnce")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 44 34) (end 44 39)) (probe (position 44 34))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Interface::"))
        (kind featureTyping) (ordinal 0) (authored-target "Port")
        (range (start 44 34) (end 44 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::Port") (range (start 10 4) (end 10 31)))
        )
      )
    )
    (query (range (start 57 46) (end 57 55)) (probe (position 57 46))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface"))
        (kind specialization) (ordinal 0) (authored-target "Interface")
        (range (start 57 46) (end 57 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::Interface") (range (start 32 4) (end 32 1084)))
        )
      )
    )
    (query (range (start 0 0) (end 0 10)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Interfaces::binaryInterfaces"))
        (kind specialization) (ordinal 0) (authored-target "interfaces")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::interfaces") (range (start 75 4) (end 75 184)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Interfaces::interfaces"))
        (kind specialization) (ordinal 0) (authored-target "connections")
        (range (start 0 0) (end 0 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::connections") (range (start 7 4) (end 7 44)))
        )
      )
    )
    (query (range (start 32 40) (end 32 50)) (probe (position 32 40))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Interface"))
        (kind specialization) (ordinal 0) (authored-target "Connection")
        (range (start 32 40) (end 32 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::Connection") (range (start 6 4) (end 6 43)))
        )
      )
    )
    (query (range (start 0 0) (end 0 11)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Interfaces::binaryInterfaces"))
        (kind specialization) (ordinal 0) (authored-target "interfaces")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::interfaces") (range (start 75 4) (end 75 184)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Interfaces::interfaces"))
        (kind specialization) (ordinal 0) (authored-target "connections")
        (range (start 0 0) (end 0 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::connections") (range (start 7 4) (end 7 44)))
        )
      )
    )
    (query (range (start 10 19) (end 10 30)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Port"))
        (kind membershipImport) (ordinal 0) (authored-target "Ports::Port")
        (range (start 10 19) (end 10 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 21) (end 44 32)) (probe (position 44 21))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Interface::"))
        (kind redefinition) (ordinal 0) (authored-target "participant")
        (range (start 44 21) (end 44 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 57) (end 57 73)) (probe (position 57 57))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface"))
        (kind specialization) (ordinal 1) (authored-target "BinaryConnection")
        (range (start 57 57) (end 57 73))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::BinaryConnection") (range (start 8 4) (end 8 49)))
        )
      )
    )
    (query (range (start 0 12) (end 0 29)) (probe (position 0 12))
      (reference
        (source (document "d0") (qualified-name "Interfaces::binaryInterfaces"))
        (kind specialization) (ordinal 1) (authored-target "binaryConnections")
        (range (start 0 12) (end 0 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interfaces::binaryConnections") (range (start 9 4) (end 9 50)))
        )
      )
    )
    (query (range (start 12 19) (end 12 40)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 12 19) (end 12 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 21) (end 69 43)) (probe (position 69 21))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface::"))
        (kind redefinition) (ordinal 0) (authored-target "Interface::participant")
        (range (start 69 21) (end 69 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 42)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::Connection"))
        (kind membershipImport) (ordinal 0) (authored-target "Connections::Connection")
        (range (start 6 19) (end 6 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 19) (end 13 42)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 13 19) (end 13 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 43)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::connections"))
        (kind membershipImport) (ordinal 0) (authored-target "Connections::connections")
        (range (start 7 19) (end 7 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 71 34) (end 71 58)) (probe (position 71 34))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryConnection::source")
        (range (start 71 34) (end 71 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 72 34) (end 72 58)) (probe (position 72 34))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryConnection::target")
        (range (start 72 34) (end 72 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 46)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::selectOne"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::selectOne")
        (range (start 15 19) (end 15 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 19) (end 17 46)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 17 19) (end 17 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 48)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryConnection"))
        (kind membershipImport) (ordinal 0) (authored-target "Connections::BinaryConnection")
        (range (start 8 19) (end 8 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 45) (end 69 74)) (probe (position 69 45))
      (reference
        (source (document "d0") (qualified-name "Interfaces::BinaryInterface::"))
        (kind redefinition) (ordinal 1) (authored-target "BinaryConnection::participant")
        (range (start 69 45) (end 69 74))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 49)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::binaryConnections"))
        (kind membershipImport) (ordinal 0) (authored-target "Connections::binaryConnections")
        (range (start 9 19) (end 9 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 19) (end 14 49)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "Interfaces::excludingAt"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excludingAt")
        (range (start 14 19) (end 14 49))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
