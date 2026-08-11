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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flows.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 39))
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
        (range (start 14 19) (end 14 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 19) (end 18 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 19) (end 19 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 46) (end 81 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 63) (end 81 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 82 46) (end 82 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 82 63) (end 82 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 60) (end 94 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 60) (end 95 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 60) (end 112 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 78) (end 112 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 113 60) (end 113 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 113 78) (end 113 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 85) (end 122 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 123 85) (end 123 112))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ac41af51c34a9c77f930748768d17b46367b35d4565bfd0fa8d23d978a242ab0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flows"))) (kind "package") (name "Flows") (declared-name "Flows") (range (start (line 0) (character 0)) (end (line 0) (character 4766))))
    (element (id (node (document "d0") (qualified-name "Flows::Action"))) (kind "import") (name "Action") (declared-name "Action") (range (start (line 17) (character 4)) (end (line 17) (character 35))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow"))) (kind "flow def") (name "Flow") (declared-name "Flow") (range (start (line 73) (character 4)) (end (line 73) (character 455))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Message") (range (start (line 73) (character 30)) (end (line 73) (character 37)))) (specializes (reference "FlowTransfer") (range (start (line 73) (character 39)) (end (line 73) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::_documentation"))) (kind "documentation") (name "") (range (start (line 73) (character 4)) (end (line 73) (character 455))) (parent (node (document "d0") (qualified-name "Flows::Flow"))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 81) (character 8)) (end (line 81) (character 84))) (parent (node (document "d0") (qualified-name "Flows::Flow"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Message::source") (range (start (line 81) (character 46)) (end (line 81) (character 61)))) (redefinition (reference "FlowTransfer::source") (range (start (line 81) (character 63)) (end (line 81) (character 83)))))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 82) (character 8)) (end (line 82) (character 84))) (parent (node (document "d0") (qualified-name "Flows::Flow"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Message::target") (range (start (line 82) (character 46)) (end (line 82) (character 61)))) (redefinition (reference "FlowTransfer::target") (range (start (line 82) (character 63)) (end (line 82) (character 83)))))))
    (element (id (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (kind "import") (name "FlowTransfer") (declared-name "FlowTransfer") (range (start (line 13) (character 4)) (end (line 13) (character 43))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 19)) (end (line 13) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (kind "import") (name "FlowTransferBefore") (declared-name "FlowTransferBefore") (range (start (line 15) (character 4)) (end (line 15) (character 49))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransferBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 48))))))
    (element (id (node (document "d0") (qualified-name "Flows::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (range (start (line 9) (character 4)) (end (line 9) (character 46))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Flows::Link"))) (kind "import") (name "Link") (declared-name "Link") (range (start (line 7) (character 4)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::Link") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Flows::Message"))) (kind "flow def") (name "Message") (declared-name "Message") (range (start (line 37) (character 4)) (end (line 37) (character 1504))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MessageAction") (range (start (line 37) (character 33)) (end (line 37) (character 46)))) (specializes (reference "Transfer") (range (start (line 37) (character 48)) (end (line 37) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "Flows::Message::_documentation"))) (kind "documentation") (name "") (range (start (line 37) (character 4)) (end (line 37) (character 1504))) (parent (node (document "d0") (qualified-name "Flows::Message"))))
    (element (id (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind "flow def") (name "MessageAction") (declared-name "MessageAction") (range (start (line 21) (character 4)) (end (line 21) (character 436))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range (start (line 21) (character 39)) (end (line 21) (character 45)))) (specializes (reference "Link") (range (start (line 21) (character 47)) (end (line 21) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "Flows::MessageAction::_documentation"))) (kind "documentation") (name "") (range (start (line 21) (character 4)) (end (line 21) (character 436))) (parent (node (document "d0") (qualified-name "Flows::MessageAction"))))
    (element (id (node (document "d0") (qualified-name "Flows::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 19) (character 4)) (end (line 19) (character 41))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 19)) (end (line 19) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Flows::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 8) (character 4)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind "flow def") (name "SuccessionFlow") (declared-name "SuccessionFlow") (range (start (line 85) (character 4)) (end (line 85) (character 541))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Flow") (range (start (line 85) (character 40)) (end (line 85) (character 44)))) (specializes (reference "FlowTransferBefore") (range (start (line 85) (character 46)) (end (line 85) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::_documentation"))) (kind "documentation") (name "") (range (start (line 85) (character 4)) (end (line 85) (character 541))) (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 94) (character 8)) (end (line 94) (character 87))) (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Flow::source") (range (start (line 94) (character 46)) (end (line 94) (character 58)))) (redefinition (reference "FlowTransferBefore::source") (range (start (line 94) (character 60)) (end (line 94) (character 86)))))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 95) (character 8)) (end (line 95) (character 87))) (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Flow::target") (range (start (line 95) (character 46)) (end (line 95) (character 58)))) (redefinition (reference "FlowTransferBefore::target") (range (start (line 95) (character 60)) (end (line 95) (character 86)))))))
    (element (id (node (document "d0") (qualified-name "Flows::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (range (start (line 11) (character 4)) (end (line 11) (character 39))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Flows::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4766))) (parent (node (document "d0") (qualified-name "Flows"))))
    (element (id (node (document "d0") (qualified-name "Flows::actions"))) (kind "import") (name "actions") (declared-name "actions") (range (start (line 18) (character 4)) (end (line 18) (character 36))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 19)) (end (line 18) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Flows::binaryLinkObjects"))) (kind "import") (name "binaryLinkObjects") (declared-name "binaryLinkObjects") (range (start (line 10) (character 4)) (end (line 10) (character 46))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::binaryLinkObjects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Flows::flowTransfers"))) (kind "import") (name "flowTransfers") (declared-name "flowTransfers") (range (start (line 14) (character 4)) (end (line 14) (character 44))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfers") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Flows::flowTransfersBefore"))) (kind "import") (name "flowTransfersBefore") (declared-name "flowTransfersBefore") (range (start (line 16) (character 4)) (end (line 16) (character 50))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfersBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 49))))))
    (element (id (node (document "d0") (qualified-name "Flows::flows"))) (kind "flow") (name "flows") (declared-name "flows") (range (start (line 105) (character 4)) (end (line 105) (character 437))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "Flow") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::_documentation"))) (kind "documentation") (name "") (range (start (line 105) (character 4)) (end (line 105) (character 437))) (parent (node (document "d0") (qualified-name "Flows::flows"))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 112) (character 8)) (end (line 112) (character 100))) (parent (node (document "d0") (qualified-name "Flows::flows"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Flow::source") (range (start (line 112) (character 46)) (end (line 112) (character 58)))) (redefinition (reference "messages::source") (range (start (line 112) (character 60)) (end (line 112) (character 76)))) (redefinition (reference "flowTransfers::source") (range (start (line 112) (character 78)) (end (line 112) (character 99)))))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 113) (character 8)) (end (line 113) (character 100))) (parent (node (document "d0") (qualified-name "Flows::flows"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "Flow::target") (range (start (line 113) (character 46)) (end (line 113) (character 58)))) (redefinition (reference "messages::target") (range (start (line 113) (character 60)) (end (line 113) (character 76)))) (redefinition (reference "flowTransfers::target") (range (start (line 113) (character 78)) (end (line 113) (character 99)))))))
    (element (id (node (document "d0") (qualified-name "Flows::messages"))) (kind "flow") (name "messages") (declared-name "messages") (range (start (line 98) (character 4)) (end (line 98) (character 178))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "Message") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flows::messages::_documentation"))) (kind "documentation") (name "") (range (start (line 98) (character 4)) (end (line 98) (character 178))) (parent (node (document "d0") (qualified-name "Flows::messages"))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows"))) (kind "flow") (name "successionFlows") (declared-name "successionFlows") (range (start (line 116) (character 4)) (end (line 116) (character 447))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "SuccessionFlow") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::_documentation"))) (kind "documentation") (name "") (range (start (line 116) (character 4)) (end (line 116) (character 447))) (parent (node (document "d0") (qualified-name "Flows::successionFlows"))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 122) (character 8)) (end (line 122) (character 113))) (parent (node (document "d0") (qualified-name "Flows::successionFlows"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "SuccessionFlow::source") (range (start (line 122) (character 46)) (end (line 122) (character 68)))) (redefinition (reference "flows::source") (range (start (line 122) (character 70)) (end (line 122) (character 83)))) (redefinition (reference "flowTransfersBefore::source") (range (start (line 122) (character 85)) (end (line 122) (character 112)))))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 123) (character 8)) (end (line 123) (character 113))) (parent (node (document "d0") (qualified-name "Flows::successionFlows"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "SuccessionFlow::target") (range (start (line 123) (character 46)) (end (line 123) (character 68)))) (redefinition (reference "flows::target") (range (start (line 123) (character 70)) (end (line 123) (character 83)))) (redefinition (reference "flowTransfersBefore::target") (range (start (line 123) (character 85)) (end (line 123) (character 112)))))))
    (element (id (node (document "d0") (qualified-name "Flows::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (range (start (line 12) (character 4)) (end (line 12) (character 40))) (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flows::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (range (start (line 17) (character 19)) (end (line 17) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0)) (authored-target "Message") (range (start (line 73) (character 30)) (end (line 73) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Message")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 1)) (authored-target "FlowTransfer") (range (start (line 73) (character 39)) (end (line 73) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::FlowTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind redefinition) (ordinal 0)) (authored-target "Message::source") (range (start (line 81) (character 46)) (end (line 81) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransfer::source") (range (start (line 81) (character 63)) (end (line 81) (character 83))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind redefinition) (ordinal 0)) (authored-target "Message::target") (range (start (line 82) (character 46)) (end (line 82) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransfer::target") (range (start (line 82) (character 63)) (end (line 82) (character 83))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransfer") (range (start (line 13) (character 19)) (end (line 13) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransferBefore") (range (start (line 15) (character 19)) (end (line 15) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (range (start (line 9) (character 19)) (end (line 9) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Link"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::Link") (range (start (line 7) (character 19)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0)) (authored-target "MessageAction") (range (start (line 37) (character 33)) (end (line 37) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 1)) (authored-target "Transfer") (range (start (line 37) (character 48)) (end (line 37) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Transfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range (start (line 21) (character 39)) (end (line 21) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 1)) (authored-target "Link") (range (start (line 21) (character 47)) (end (line 21) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Link")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 19) (character 19)) (end (line 19) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 8) (character 19)) (end (line 8) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0)) (authored-target "Flow") (range (start (line 85) (character 40)) (end (line 85) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 1)) (authored-target "FlowTransferBefore") (range (start (line 85) (character 46)) (end (line 85) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::FlowTransferBefore")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::source") (range (start (line 94) (character 46)) (end (line 94) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransferBefore::source") (range (start (line 94) (character 60)) (end (line 94) (character 86))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::target") (range (start (line 95) (character 46)) (end (line 95) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransferBefore::target") (range (start (line 95) (character 60)) (end (line 95) (character 86))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (range (start (line 11) (character 19)) (end (line 11) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (range (start (line 18) (character 19)) (end (line 18) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::binaryLinkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::binaryLinkObjects") (range (start (line 10) (character 19)) (end (line 10) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flowTransfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfers") (range (start (line 14) (character 19)) (end (line 14) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flowTransfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfersBefore") (range (start (line 16) (character 19)) (end (line 16) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows"))) (kind featureTyping) (ordinal 0)) (authored-target "Flow") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::source") (range (start (line 112) (character 46)) (end (line 112) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 1)) (authored-target "messages::source") (range (start (line 112) (character 60)) (end (line 112) (character 76))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfers::source") (range (start (line 112) (character 78)) (end (line 112) (character 99))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::target") (range (start (line 113) (character 46)) (end (line 113) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 1)) (authored-target "messages::target") (range (start (line 113) (character 60)) (end (line 113) (character 76))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfers::target") (range (start (line 113) (character 78)) (end (line 113) (character 99))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::messages"))) (kind featureTyping) (ordinal 0)) (authored-target "Message") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Message")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows"))) (kind featureTyping) (ordinal 0)) (authored-target "SuccessionFlow") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 0)) (authored-target "SuccessionFlow::source") (range (start (line 122) (character 46)) (end (line 122) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 1)) (authored-target "flows::source") (range (start (line 122) (character 70)) (end (line 122) (character 83))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::flows::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfersBefore::source") (range (start (line 122) (character 85)) (end (line 122) (character 112))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 0)) (authored-target "SuccessionFlow::target") (range (start (line 123) (character 46)) (end (line 123) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 1)) (authored-target "flows::target") (range (start (line 123) (character 70)) (end (line 123) (character 83))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::flows::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfersBefore::target") (range (start (line 123) (character 85)) (end (line 123) (character 112))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (range (start (line 12) (character 19)) (end (line 12) (character 39))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::Flow"))) (target (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::Flow"))) (target (node (document "d0") (qualified-name "Flows::Message"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::Message"))) (target (node (document "d0") (qualified-name "Flows::MessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::Message"))) (target (node (document "d0") (qualified-name "Flows::Transfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (target (node (document "d0") (qualified-name "Flows::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (target (node (document "d0") (qualified-name "Flows::Link"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (target (node (document "d0") (qualified-name "Flows::Flow"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (target (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (target (node (document "d0") (qualified-name "Flows::Flow::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (target (node (document "d0") (qualified-name "Flows::Flow::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::flows"))) (target (node (document "d0") (qualified-name "Flows::Flow"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::flows"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::flows::source"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::flows::source"))) (target (node (document "d0") (qualified-name "Flows::Flow::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::flows::target"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::flows::target"))) (target (node (document "d0") (qualified-name "Flows::Flow::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::messages"))) (target (node (document "d0") (qualified-name "Flows::Message"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::messages"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::successionFlows"))) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (target (node (document "d0") (qualified-name "Flows::flows::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (target (node (document "d0") (qualified-name "Flows::Occurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (target (node (document "d0") (qualified-name "Flows::flows::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
