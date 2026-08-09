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
(model
  (namespace
    (library_package 'Ports'
      (documentation)
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Objects::objects'[unresolved])
      (port_def abstract 'Port' :> 'Object'[unresolved]
        (documentation)
        (reference_usage reference 'self' : 'Ports::Port'[port_def] :>> 'Object::self'[unresolved])
        (port_usage composite 'subports' : 'Ports::Port'[port_def] :> 'Ports::ports'[port_usage] :> 'timeEnclosedOccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (port_usage abstract reference 'interfacingPorts' : 'Ports::Port'[port_def] :> 'Ports::ports'[port_usage]
          (multiplicity_range [0..*])
          (documentation))
        (reference_usage reference :>> 'outgoingTransfersFromSelf'[unresolved] :> 'interfacingPorts::incomingTransfersToSelf'[unresolved]
          (documentation)
          (port_usage end 'source' :> 'Ports::ports'[port_usage][implied])
          (port_usage end 'target' :> 'Ports::ports'[port_usage][implied])))
      (port_usage abstract 'ports' : 'Ports::Port'[port_def] :> 'objects'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
