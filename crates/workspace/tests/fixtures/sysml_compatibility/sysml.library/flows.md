# META
~~~ini
description=Standard Library: Systems Library/Flows
type=file
~~~
# SOURCE
~~~sysml
standard library package Flows {
    doc
    /*
     * This package defines the base types for flows and related behavioral elements 
     * in the SysML language.
     */

    private import Links::Link;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import Actions::Action;
    private import Actions::actions;
    private import ScalarValues::Natural;
    
    abstract flow def MessageAction :> Action, Link {
        doc
        /*
         * MessageAction is the most general class of actions that represent
         * interactions between linked things. It is the base type of all
         * FlowDefinitions.
         */
         
         ref payload [0..*] {
         	doc
         	/*
         	 * A payload that may be transferred during the interaction.
         	 */
         }
    }

    abstract flow def Message :> MessageAction, Transfer {
        doc
        /*
         * Message is the subclass of message connections that represent 
         * a transfer of objects or values between two occurrences. It is 
         * the base type of all FlowUsages.
         */
        
        ref payload :>> MessageAction::payload, Transfer::payload;
        
        private ref action thisConnection = self;
        
        in event occurrence sourceEvent [1] default thisConnection.start {
            doc
            /* 
             * An occurrence happening during the source of this message
             * that is either the start of the mssage or happens before it.
             */
        }
        in event occurrence targetEvent [1] default thisConnection.done {
            doc
            /* 
             * An occurrence happening during the target of this message
             * that is either the end of the message or happens after it.
             */
        }
        
        connection :HappensDuring connect sourceEvent to [1] source;
        connection :HappensDuring connect targetEvent to [1] target;
        
        private attribute seBeforeNum: Natural[1] = if sourceEvent==thisConnection.start ? 0 else 1;
        private attribute teAfterNum: Natural[1] = if targetEvent==thisConnection.done ? 0 else 1;
        succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;
        succession [teAfterNum] first [0..1] self then [0..1] targetEvent;
    }
    
    abstract flow def Flow :> Message, FlowTransfer {
        doc
        /*
         * Flow is a subclass of messages that are also flow transfers.
         * It is the base type for FlowUsages that identify their source output and
         * target input.
         */
         
        end occurrence source: Occurrence :>> Message::source, FlowTransfer::source;
        end occurrence target: Occurrence :>> Message::target, FlowTransfer::target;
    }
    
    abstract flow def SuccessionFlow :> Flow, FlowTransferBefore {
        doc
        /*
         * SuccessionFlow is a subclass of flowss that appen after their source and 
         * before their target. It is the base type for all SuccessionFlowUsages.
         */
         
        ref self : SuccessionFlow :>> Flow::self, FlowTransferBefore::self;
    
        end occurrence source: Occurrence :>> Flow::source, FlowTransferBefore::source;
        end occurrence target: Occurrence :>> Flow::target, FlowTransferBefore::target;
    }
    
    abstract message messages: Message[0..*] nonunique :> transfers, actions {
        doc
        /*
         * messages is the base feature of all FlowUsages.
         */
    }
    
    abstract flow flows: Flow[0..*] nonunique :> messages, flowTransfers {
        doc
        /*
         * flows is the base feature for FlowUsages that identify their source output
         * and target input.
         */
    
        end occurrence source: Occurrence :>> Flow::source, messages::source, flowTransfers::source;
        end occurrence target: Occurrence :>> Flow::target, messages::target, flowTransfers::target;
    }
    
    abstract flow successionFlows: SuccessionFlow[0..*] nonunique :> flows, flowTransfersBefore {
        doc
        /*
         * successionFlows is the base feature of all SuccessionFlowUsages.
         */
    
        end occurrence source: Occurrence :>> SuccessionFlow::source, flows::source, flowTransfersBefore::source;
        end occurrence target: Occurrence :>> SuccessionFlow::target, flows::target, flowTransfersBefore::target;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Link'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer::payload'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'FlowTransfer'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Message::source'
semantic.unresolved_name 'FlowTransfer::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Message::target'
semantic.unresolved_name 'FlowTransfer::target'
semantic.unresolved_name 'FlowTransferBefore'
semantic.unresolved_name 'Flow::self'
semantic.unresolved_name 'FlowTransferBefore::self'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'FlowTransferBefore::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'FlowTransferBefore::target'
semantic.unresolved_name 'transfers'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'flowTransfers'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'messages::source'
semantic.unresolved_name 'flowTransfers::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'messages::target'
semantic.unresolved_name 'flowTransfers::target'
semantic.unresolved_name 'flowTransfersBefore'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'flowTransfersBefore::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'flowTransfersBefore::target'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Link'
semantic.unresolved_name 'Transfer'
semantic.unresolved_name 'Transfer::payload'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'FlowTransfer'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Message::source'
semantic.unresolved_name 'FlowTransfer::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Message::target'
semantic.unresolved_name 'FlowTransfer::target'
semantic.unresolved_name 'FlowTransferBefore'
semantic.unresolved_name 'Flow::self'
semantic.unresolved_name 'FlowTransferBefore::self'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'FlowTransferBefore::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'FlowTransferBefore::target'
semantic.unresolved_name 'transfers'
semantic.unresolved_name 'actions'
semantic.unresolved_name 'flowTransfers'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'messages::source'
semantic.unresolved_name 'flowTransfers::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'messages::target'
semantic.unresolved_name 'flowTransfers::target'
semantic.unresolved_name 'flowTransfersBefore'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'flowTransfersBefore::source'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'flowTransfersBefore::target'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwFlow,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwFlow,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwRef,KwAction,Ident,Eq,Ident,Semicolon,
KwIn,KwEvent,KwOccurrence,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwEvent,KwOccurrence,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwConnection,Colon,Ident,KwConnect,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnection,Colon,Ident,KwConnect,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,EqEq,Ident,Dot,Ident,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,EqEq,Ident,Dot,Ident,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwSuccession,OpenSquare,Ident,CloseSquare,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwSuccession,OpenSquare,Ident,CloseSquare,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFlow,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFlow,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMessage,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFlow,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFlow,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwOccurrence,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Flows'
    (documentation)
    (import_decl private 'Links::Link')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'Objects::binaryLinkObjects')
    (import_decl private 'Transfers::Transfer')
    (import_decl private 'Transfers::transfers')
    (import_decl private 'Transfers::FlowTransfer')
    (import_decl private 'Transfers::flowTransfers')
    (import_decl private 'Transfers::FlowTransferBefore')
    (import_decl private 'Transfers::flowTransfersBefore')
    (import_decl private 'Actions::Action')
    (import_decl private 'Actions::actions')
    (import_decl private 'ScalarValues::Natural')
    (flow_def abstract 'MessageAction' :> 'Action', 'Link'
      (documentation)
      (ref_usage ref 'payload' multiplicity
        (documentation)))
    (flow_def abstract 'Message' :> 'MessageAction', 'Transfer'
      (documentation)
      (ref_usage ref 'payload' :>> 'MessageAction::payload', 'Transfer::payload')
      (action_usage private ref 'thisConnection' value)
      (event_occurrence in 'sourceEvent' multiplicity value
        (documentation))
      (event_occurrence in 'targetEvent' multiplicity value
        (documentation))
      (connection_usage 'HappensDuring'
        (connector_end)
        (connector_end))
      (connection_usage 'HappensDuring'
        (connector_end)
        (connector_end))
      (attribute_usage private 'seBeforeNum' : 'Natural' multiplicity value)
      (attribute_usage private 'teAfterNum' : 'Natural' multiplicity value)
      (succession_as_usage multiplicity
        (connector_end)
        (connector_end))
      (succession_as_usage multiplicity
        (connector_end)
        (connector_end)))
    (flow_def abstract 'Flow' :> 'Message', 'FlowTransfer'
      (documentation)
      (interface_end end 'source' : 'Occurrence' :>> 'Message::source', 'FlowTransfer::source')
      (interface_end end 'target' : 'Occurrence' :>> 'Message::target', 'FlowTransfer::target'))
    (flow_def abstract 'SuccessionFlow' :> 'Flow', 'FlowTransferBefore'
      (documentation)
      (ref_usage ref 'self' : 'SuccessionFlow' :>> 'Flow::self', 'FlowTransferBefore::self')
      (interface_end end 'source' : 'Occurrence' :>> 'Flow::source', 'FlowTransferBefore::source')
      (interface_end end 'target' : 'Occurrence' :>> 'Flow::target', 'FlowTransferBefore::target'))
    (message_usage 'Message' :> 'transfers', 'actions' 'messages' multiplicity
      (documentation))
    (flow_usage 'Flow' :> 'messages', 'flowTransfers' 'flows' multiplicity
      (documentation)
      (interface_end end 'source' : 'Occurrence' :>> 'Flow::source', 'messages::source', 'flowTransfers::source')
      (interface_end end 'target' : 'Occurrence' :>> 'Flow::target', 'messages::target', 'flowTransfers::target'))
    (flow_usage 'SuccessionFlow' :> 'flows', 'flowTransfersBefore' 'successionFlows' multiplicity
      (documentation)
      (interface_end end 'source' : 'Occurrence' :>> 'SuccessionFlow::source', 'flows::source', 'flowTransfersBefore::source')
      (interface_end end 'target' : 'Occurrence' :>> 'SuccessionFlow::target', 'flows::target', 'flowTransfersBefore::target'))))
~~~
# FORMAT
~~~sysml
standard library package Flows {
    doc /*
     * This package defines the base types for flows and related behavioral elements 
     * in the SysML language.
     */

    private import Links::Link;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import Actions::Action;
    private import Actions::actions;
    private import ScalarValues::Natural;

    abstract flow def MessageAction :> Action, Link {
        doc /*
         * MessageAction is the most general class of actions that represent
         * interactions between linked things. It is the base type of all
         * FlowDefinitions.
         */

        ref payload [0..*] {
            doc /*
         	 * A payload that may be transferred during the interaction.
         	 */
        }
    }

    abstract flow def Message :> MessageAction, Transfer {
        doc /*
         * Message is the subclass of message connections that represent 
         * a transfer of objects or values between two occurrences. It is 
         * the base type of all FlowUsages.
         */

        ref payload :>> MessageAction::payload, Transfer::payload;

        private ref action thisConnection = self;

        in event occurrence sourceEvent [1] default = thisConnection.start {
            doc /* 
             * An occurrence happening during the source of this message
             * that is either the start of the mssage or happens before it.
             */
        }
        in event occurrence targetEvent [1] default = thisConnection.done {
            doc /* 
             * An occurrence happening during the target of this message
             * that is either the end of the message or happens after it.
             */
        }

        connection : HappensDuring connect sourceEvent to [1] source;
        connection : HappensDuring connect targetEvent to [1] target;

        private attribute seBeforeNum : Natural [1] = if sourceEvent==thisConnection.start ? 0 else 1;
        private attribute teAfterNum : Natural [1] = if targetEvent==thisConnection.done ? 0 else 1;
        succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;
        succession [teAfterNum] first [0..1] self then [0..1] targetEvent;
    }

    abstract flow def Flow :> Message, FlowTransfer {
        doc /*
         * Flow is a subclass of messages that are also flow transfers.
         * It is the base type for FlowUsages that identify their source output and
         * target input.
         */

        end source : Occurrence :>> Message::source, FlowTransfer::source;
        end target : Occurrence :>> Message::target, FlowTransfer::target;
    }

    abstract flow def SuccessionFlow :> Flow, FlowTransferBefore {
        doc /*
         * SuccessionFlow is a subclass of flowss that appen after their source and 
         * before their target. It is the base type for all SuccessionFlowUsages.
         */

        ref self : SuccessionFlow :>> Flow::self, FlowTransferBefore::self;

        end source : Occurrence :>> Flow::source, FlowTransferBefore::source;
        end target : Occurrence :>> Flow::target, FlowTransferBefore::target;
    }

    abstract message messages : Message :> transfers, actions [0..*] {
        doc /*
         * messages is the base feature of all FlowUsages.
         */
    }

    abstract flow flows : Flow :> messages, flowTransfers [0..*] {
        doc /*
         * flows is the base feature for FlowUsages that identify their source output
         * and target input.
         */
        end source : Occurrence :>> Flow::source, messages::source, flowTransfers::source;
        end target : Occurrence :>> Flow::target, messages::target, flowTransfers::target;
    }

    abstract flow successionFlows : SuccessionFlow :> flows, flowTransfersBefore [0..*] {
        doc /*
         * successionFlows is the base feature of all SuccessionFlowUsages.
         */
        end source : Occurrence :>> SuccessionFlow::source, flows::source, flowTransfersBefore::source;
        end target : Occurrence :>> SuccessionFlow::target, flows::target, flowTransfersBefore::target;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Flows'
      (documentation)
      (membership_import private -> 'Links::Link'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'Objects::binaryLinkObjects'[unresolved])
      (membership_import private -> 'Transfers::Transfer'[unresolved])
      (membership_import private -> 'Transfers::transfers'[unresolved])
      (membership_import private -> 'Transfers::FlowTransfer'[unresolved])
      (membership_import private -> 'Transfers::flowTransfers'[unresolved])
      (membership_import private -> 'Transfers::FlowTransferBefore'[unresolved])
      (membership_import private -> 'Transfers::flowTransfersBefore'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Actions::actions'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (flow_def abstract 'MessageAction' :> 'Action'[unresolved] :> 'Link'[unresolved]
        (documentation)
        (reference_usage reference 'payload'
          (multiplicity_range [0..*])
          (documentation)))
      (flow_def abstract 'Message' :> 'Flows::MessageAction'[flow_def] :> 'Transfer'[unresolved]
        (documentation)
        (reference_usage reference 'payload' :>> 'Flows::MessageAction::payload'[reference_usage] :>> 'Transfer::payload'[unresolved])
        (action_usage reference 'thisConnection'
          (feature_value (=)))
        (event_occurrence_usage in 'sourceEvent'
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (event_occurrence_usage in 'targetEvent'
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (connection_usage composite : 'HappensDuring'[unresolved]
          (connector_end 'sourceEvent')
          (connector_end 'source'))
        (connection_usage composite : 'HappensDuring'[unresolved]
          (connector_end 'targetEvent')
          (connector_end 'target'))
        (attribute_usage composite 'seBeforeNum' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite 'teAfterNum' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (succession_def
          (multiplicity_range [?])
          (connector_end 'sourceEvent')
          (connector_end 'self'))
        (succession_def
          (multiplicity_range [?])
          (connector_end 'self')
          (connector_end 'targetEvent')))
      (flow_def abstract 'Flow' :> 'Flows::Message'[flow_def] :> 'FlowTransfer'[unresolved]
        (documentation)
        (port_usage end 'source' : 'Occurrence'[unresolved] :>> 'Message::source'[unresolved] :>> 'FlowTransfer::source'[unresolved])
        (port_usage end 'target' : 'Occurrence'[unresolved] :>> 'Message::target'[unresolved] :>> 'FlowTransfer::target'[unresolved]))
      (flow_def abstract 'SuccessionFlow' :> 'Flows::Flow'[flow_def] :> 'FlowTransferBefore'[unresolved]
        (documentation)
        (reference_usage reference 'self' : 'Flows::SuccessionFlow'[flow_def] :>> 'Flow::self'[unresolved] :>> 'FlowTransferBefore::self'[unresolved])
        (port_usage end 'source' : 'Occurrence'[unresolved] :>> 'Flows::Flow::source'[port_usage] :>> 'FlowTransferBefore::source'[unresolved])
        (port_usage end 'target' : 'Occurrence'[unresolved] :>> 'Flows::Flow::target'[port_usage] :>> 'FlowTransferBefore::target'[unresolved]))
      (flow_usage abstract 'messages' : 'Flows::Message'[flow_def] :> 'transfers'[unresolved] :> 'actions'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (flow_usage abstract 'flows' : 'Flows::Flow'[flow_def] :> 'Flows::messages'[flow_usage] :> 'flowTransfers'[unresolved]
        (multiplicity_range [0..*])
        (documentation)
        (port_usage end 'source' : 'Occurrence'[unresolved] :>> 'Flows::Flow::source'[port_usage] :>> 'messages::source'[unresolved] :>> 'flowTransfers::source'[unresolved])
        (port_usage end 'target' : 'Occurrence'[unresolved] :>> 'Flows::Flow::target'[port_usage] :>> 'messages::target'[unresolved] :>> 'flowTransfers::target'[unresolved]))
      (flow_usage abstract 'successionFlows' : 'Flows::SuccessionFlow'[flow_def] :> 'Flows::flows'[flow_usage] :> 'flowTransfersBefore'[unresolved]
        (multiplicity_range [0..*])
        (documentation)
        (port_usage end 'source' : 'Occurrence'[unresolved] :>> 'Flows::SuccessionFlow::source'[port_usage] :>> 'Flows::flows::source'[port_usage] :>> 'flowTransfersBefore::source'[unresolved])
        (port_usage end 'target' : 'Occurrence'[unresolved] :>> 'Flows::SuccessionFlow::target'[port_usage] :>> 'Flows::flows::target'[port_usage] :>> 'flowTransfersBefore::target'[unresolved])))))
~~~
