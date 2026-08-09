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
# EXPECTED
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object::self'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'outgoingTransfersFromSelf'
semantic.unresolved_name 'interfacingPorts::incomingTransfersToSelf'
semantic.unresolved_name 'objects'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object::self'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'outgoingTransfersFromSelf'
semantic.unresolved_name 'interfacingPorts::incomingTransfersToSelf'
semantic.unresolved_name 'objects'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRef,KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,ColonGtGt,Ident,ColonGt,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwRef,Ident,Semicolon,
KwEnd,KwRef,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Ports'
    (documentation)
    (import_decl private 'Objects::Object')
    (import_decl private 'Objects::objects')
    (port_def abstract 'Port' :> 'Object'
      (documentation)
      (ref_usage ref 'self' : 'Port' :>> 'Object::self')
      (port_usage 'subports' : 'Port' :> 'ports', 'timeEnclosedOccurrences' multiplicity
        (documentation))
      (port_usage abstract ref 'interfacingPorts' : 'Port' :> 'ports' multiplicity nonunique
        (documentation))
      (ref_usage ref :>> 'outgoingTransfersFromSelf' :> 'interfacingPorts.incomingTransfersToSelf'
        (documentation)
        (interface_end end 'source')
        (interface_end end 'target')))
    (port_usage abstract 'ports' : 'Port' :> 'objects' multiplicity nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Ports {
    doc /*
     * This package defines the base types for ports and related structural elements 
     * in the SysML language.
     */

    private import Objects::Object;
    private import Objects::objects;

    abstract port def Port :> Object {
        doc /*
         * Port is the most general class of objects that represent connection points
         * for interacting with a Part. Port is the base type of all PortDefinitions.
         * 
         * Transfers outgoing from a Port are always targeted to a Port connected to
         * the original Port by an Interface.
         */

        ref self : Port :>> Object::self;

        port subports : Port :> ports, timeEnclosedOccurrences [0..*] {
            doc /*
             * The Ports that are subports of this Port.
             */
        }

        abstract ref port interfacingPorts : Port :> ports [0..*] nonunique {
            doc /*
             * Ports that are connected to this Port by an Interface.
             */
        }

        ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf {
            doc /* 
             * The target of each of the outgoingTransfersFromSelf of a Port must be an interfacingPort.
             */

            end source;
            end target;
        }
    }

    abstract port ports : Port :> objects [0..*] nonunique {
        doc /*
         * ports is the base feature of all PortUsages.
         */
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Ports"))) (name "Ports") (declared-name "Ports")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Ports::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Ports::Port"))) (name "Port") (declared-name "Port")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Ports::Port::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Ports::Port")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "Ports::Port::subports"))) (name "subports") (declared-name "subports") (declared (properties (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Ports::Port"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Ports::Port::subports::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Ports::Port")))))
              )
            )
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Ports::Port::~Port"))) (name "~Port") (declared-name "~Port") (effective (featuring-type (node (document "d0") (qualified-name "Ports::Port")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Ports::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Ports::objects"))) (name "objects") (declared-name "objects"))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Ports::ports"))) (name "ports") (declared-name "ports")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Ports::ports::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Ports::ports")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Ports::ports::~ports"))) (name "~ports") (declared-name "~ports") (effective (featuring-type (node (document "d0") (qualified-name "Ports::ports")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Ports::Port::_documentation"))) (to (node (document "d0") (qualified-name "Ports::Port"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Ports::Port::subports::_documentation"))) (to (node (document "d0") (qualified-name "Ports::Port::subports"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Ports::_documentation"))) (to (node (document "d0") (qualified-name "Ports"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Ports::ports::_documentation"))) (to (node (document "d0") (qualified-name "Ports::ports"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Ports::Port::~Port"))) (to (node (document "d0") (qualified-name "Ports::Port"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Ports::ports::~ports"))) (to (node (document "d0") (qualified-name "Ports::ports"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Ports::Port::subports"))) (to (node (document "d0") (qualified-name "Ports::Port"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
