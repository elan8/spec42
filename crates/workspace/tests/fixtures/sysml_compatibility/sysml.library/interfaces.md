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
# EXPECTED
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Connection'
semantic.unresolved_name 'participant'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'interfacingPorts'
semantic.unresolved_name 'BinaryConnection'
semantic.unresolved_name 'Interface::participant'
semantic.unresolved_name 'BinaryConnection::participant'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'BinaryConnection::source'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'BinaryConnection::target'
semantic.unresolved_name 'connections'
semantic.unresolved_name 'binaryConnections'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Connection'
semantic.unresolved_name 'participant'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'interfacingPorts'
semantic.unresolved_name 'BinaryConnection'
semantic.unresolved_name 'Interface::participant'
semantic.unresolved_name 'BinaryConnection::participant'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'BinaryConnection::source'
semantic.unresolved_name 'Port'
semantic.unresolved_name 'BinaryConnection::target'
semantic.unresolved_name 'connections'
semantic.unresolved_name 'binaryConnections'
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
KwPrivate,KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwOrdered,Semicolon,
KwIn,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,EqEq,Ident,CloseCurly,Semicolon,
Ident,Arrow,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwAbstract,KwInterface,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwPort,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwOrdered,OpenCurly,
KwDoc,
RegularComment,
KwProtected,KwRef,Ident,ColonGtGt,Ident,Semicolon,
KwProtected,KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,
KwDefault,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwInterface,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwPort,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,KwNonunique,KwOrdered,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwInterface,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwInterface,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Interfaces'
    (documentation)
    (import_decl private 'Connections::Connection')
    (import_decl private 'Connections::connections')
    (import_decl private 'Connections::BinaryConnection')
    (import_decl private 'Connections::binaryConnections')
    (import_decl private 'Ports::Port')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::excludingAt')
    (import_decl private 'ControlFunctions::selectOne')
    (import_decl private 'SequenceFunctions::notEmpty')
    (calc_def private 'excludingOnce'
      (documentation)
      (default_ref_usage in 'seq' multiplicity ordered nonunique)
      (default_ref_usage in 'value' :> 'seq' multiplicity)
      (attribute_usage private 'position' : 'Natural' multiplicity value)
      (result_expr_member))
    (interface_def abstract 'Interface' :> 'Connection'
      (documentation)
      (port_usage ref :>> 'participant' : 'Port' multiplicity ordered nonunique
        (documentation)
        (ref_usage protected ref 'thisParticipant' :>> 'self')
        (ref_usage protected ref 'otherParticipants' : 'Port' :> 'interfacingPorts' multiplicity nonunique value)))
    (interface_def abstract 'BinaryInterface' :> 'Interface', 'BinaryConnection'
      (documentation)
      (port_usage ref :>> 'Interface::participant', 'BinaryConnection::participant' multiplicity ordered nonunique)
      (interface_end end 'source' : 'Port' :>> 'BinaryConnection::source')
      (interface_end end 'target' : 'Port' :>> 'BinaryConnection::target'))
    (interface_usage 'Interface' :> 'connections' 'interfaces' multiplicity
      (documentation))
    (interface_usage 'BinaryInterface' :> 'interfaces', 'binaryConnections' 'binaryInterfaces' multiplicity
      (documentation))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Interfaces"))) (name "Interfaces") (declared-name "Interfaces")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::BinaryConnection"))) (name "BinaryConnection") (declared-name "BinaryConnection"))
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (name "BinaryInterface") (declared-name "BinaryInterface")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::BinaryInterface")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::BinaryInterface")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::BinaryInterface")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Interfaces::BinaryInterface::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::BinaryInterface")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::Connection"))) (name "Connection") (declared-name "Connection"))
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Interfaces::Interface"))) (name "Interface") (declared-name "Interface")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Interfaces::Interface::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::Interface")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::Interface::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::Interface")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::Port"))) (name "Port") (declared-name "Port"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::binaryConnections"))) (name "binaryConnections") (declared-name "binaryConnections"))
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (name "binaryInterfaces") (declared-name "binaryInterfaces")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::binaryInterfaces::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::binaryInterfaces")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::connections"))) (name "connections") (declared-name "connections"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::excludingAt"))) (name "excludingAt") (declared-name "excludingAt"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Interfaces::excludingOnce"))) (name "excludingOnce") (declared-name "excludingOnce")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::excludingOnce")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::seq"))) (name "seq") (declared-name "seq") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::excludingOnce")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interfaces::excludingOnce::value"))) (name "value") (declared-name "value") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::excludingOnce")))))
          )
        )
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Interfaces::interfaces"))) (name "interfaces") (declared-name "interfaces")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Interfaces::interfaces::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Interfaces::interfaces")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::selectOne"))) (name "selectOne") (declared-name "selectOne"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Interfaces::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::BinaryInterface::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::Interface::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces::Interface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::binaryInterfaces::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::excludingOnce::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces::excludingOnce"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Interfaces::interfaces::_documentation"))) (to (node (document "d0") (qualified-name "Interfaces::interfaces"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Interfaces::BinaryInterface"))) (to (node (document "d0") (qualified-name "Interfaces::Interface"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Interfaces::binaryInterfaces"))) (to (node (document "d0") (qualified-name "Interfaces::interfaces"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
