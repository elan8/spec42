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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Flows"))) (name "Flows") (declared-name "Flows")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "flow def") (id (node (document "d0") (qualified-name "Flows::Flow"))) (name "Flow") (declared-name "Flow")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::Flow::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::Flow::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::Flow::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (name "FlowTransfer") (declared-name "FlowTransfer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (name "FlowTransferBefore") (declared-name "FlowTransferBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::HappensDuring"))) (name "HappensDuring") (declared-name "HappensDuring"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::Link"))) (name "Link") (declared-name "Link"))
        (element (kind "flow def") (id (node (document "d0") (qualified-name "Flows::Message"))) (name "Message") (declared-name "Message")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::Message::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::Message")))))
          )
        )
        (element (kind "flow def") (id (node (document "d0") (qualified-name "Flows::MessageAction"))) (name "MessageAction") (declared-name "MessageAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::MessageAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::MessageAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "flow def") (id (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (name "SuccessionFlow") (declared-name "SuccessionFlow")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::Transfer"))) (name "Transfer") (declared-name "Transfer"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::actions"))) (name "actions") (declared-name "actions"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::binaryLinkObjects"))) (name "binaryLinkObjects") (declared-name "binaryLinkObjects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::flowTransfers"))) (name "flowTransfers") (declared-name "flowTransfers"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::flowTransfersBefore"))) (name "flowTransfersBefore") (declared-name "flowTransfersBefore"))
        (element (kind "flow") (id (node (document "d0") (qualified-name "Flows::flows"))) (name "flows") (declared-name "flows")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::flows::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::flows::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::flows::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::Flow")))))
          )
        )
        (element (kind "flow") (id (node (document "d0") (qualified-name "Flows::messages"))) (name "messages") (declared-name "messages")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::messages::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::Message")))))
          )
        )
        (element (kind "flow") (id (node (document "d0") (qualified-name "Flows::successionFlows"))) (name "successionFlows") (declared-name "successionFlows")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Flows::successionFlows::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Flows::transfers"))) (name "transfers") (declared-name "transfers"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::Flow::_documentation"))) (to (node (document "d0") (qualified-name "Flows::Flow"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::Message::_documentation"))) (to (node (document "d0") (qualified-name "Flows::Message"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::MessageAction::_documentation"))) (to (node (document "d0") (qualified-name "Flows::MessageAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::SuccessionFlow::_documentation"))) (to (node (document "d0") (qualified-name "Flows::SuccessionFlow"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::_documentation"))) (to (node (document "d0") (qualified-name "Flows"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::flows::_documentation"))) (to (node (document "d0") (qualified-name "Flows::flows"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::messages::_documentation"))) (to (node (document "d0") (qualified-name "Flows::messages"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Flows::successionFlows::_documentation"))) (to (node (document "d0") (qualified-name "Flows::successionFlows"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (to (node (document "d0") (qualified-name "Flows::Flow::source"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (to (node (document "d0") (qualified-name "Flows::Flow::target"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::flows::source"))) (to (node (document "d0") (qualified-name "Flows::Flow::source"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::flows::target"))) (to (node (document "d0") (qualified-name "Flows::Flow::target"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (to (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (to (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Flows::Flow"))) (to (node (document "d0") (qualified-name "Flows::Message"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Flows::Message"))) (to (node (document "d0") (qualified-name "Flows::MessageAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (to (node (document "d0") (qualified-name "Flows::Flow"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flows::flows"))) (to (node (document "d0") (qualified-name "Flows::Flow"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flows::messages"))) (to (node (document "d0") (qualified-name "Flows::Message"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flows::successionFlows"))) (to (node (document "d0") (qualified-name "Flows::SuccessionFlow"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
