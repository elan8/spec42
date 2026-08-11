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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ac41af51c34a9c77f930748768d17b46367b35d4565bfd0fa8d23d978a242ab0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flows"))) (kind "package") (name "Flows") (declared-name "Flows"))
    (element (id (node (document "d0") (qualified-name "Flows::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow"))) (kind "flow def") (name "Flow") (declared-name "Flow") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Message")) (specializes (reference "FlowTransfer")))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::Flow"))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Flows::Flow"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Message::source")) (redefinition (reference "FlowTransfer::source")))))
    (element (id (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Flows::Flow"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Message::target")) (redefinition (reference "FlowTransfer::target")))))
    (element (id (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (kind "import") (name "FlowTransfer") (declared-name "FlowTransfer") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (kind "import") (name "FlowTransferBefore") (declared-name "FlowTransferBefore") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::FlowTransferBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::Link"))) (kind "import") (name "Link") (declared-name "Link") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::Link") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::Message"))) (kind "flow def") (name "Message") (declared-name "Message") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MessageAction")) (specializes (reference "Transfer")))))
    (element (id (node (document "d0") (qualified-name "Flows::Message::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::Message"))))
    (element (id (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind "flow def") (name "MessageAction") (declared-name "MessageAction") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "Link")))))
    (element (id (node (document "d0") (qualified-name "Flows::MessageAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::MessageAction"))))
    (element (id (node (document "d0") (qualified-name "Flows::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind "flow def") (name "SuccessionFlow") (declared-name "SuccessionFlow") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Flow")) (specializes (reference "FlowTransferBefore")))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Flow::source")) (redefinition (reference "FlowTransferBefore::source")))))
    (element (id (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Flow::target")) (redefinition (reference "FlowTransferBefore::target")))))
    (element (id (node (document "d0") (qualified-name "Flows::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows"))))
    (element (id (node (document "d0") (qualified-name "Flows::actions"))) (kind "import") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::binaryLinkObjects"))) (kind "import") (name "binaryLinkObjects") (declared-name "binaryLinkObjects") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::binaryLinkObjects") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::flowTransfers"))) (kind "import") (name "flowTransfers") (declared-name "flowTransfers") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfers") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::flowTransfersBefore"))) (kind "import") (name "flowTransfersBefore") (declared-name "flowTransfersBefore") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::flowTransfersBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flows::flows"))) (kind "flow") (name "flows") (declared-name "flows") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "Flow")))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::flows"))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Flows::flows"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Flow::source")) (redefinition (reference "messages::source")) (redefinition (reference "flowTransfers::source")))))
    (element (id (node (document "d0") (qualified-name "Flows::flows::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Flows::flows"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "Flow::target")) (redefinition (reference "messages::target")) (redefinition (reference "flowTransfers::target")))))
    (element (id (node (document "d0") (qualified-name "Flows::messages"))) (kind "flow") (name "messages") (declared-name "messages") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "Message")))))
    (element (id (node (document "d0") (qualified-name "Flows::messages::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::messages"))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows"))) (kind "flow") (name "successionFlows") (declared-name "successionFlows") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Feature)) (relationships (typing (reference "SuccessionFlow")))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Flows::successionFlows"))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Flows::successionFlows"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "SuccessionFlow::source")) (redefinition (reference "flows::source")) (redefinition (reference "flowTransfersBefore::source")))))
    (element (id (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Flows::successionFlows"))) (authored (relationships (typing (reference "Occurrence")) (redefinition (reference "SuccessionFlow::target")) (redefinition (reference "flows::target")) (redefinition (reference "flowTransfersBefore::target")))))
    (element (id (node (document "d0") (qualified-name "Flows::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (parent (node (document "d0") (qualified-name "Flows"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flows::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0)) (authored-target "Message") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Message")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 1)) (authored-target "FlowTransfer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::FlowTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind redefinition) (ordinal 0)) (authored-target "Message::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::source"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransfer::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind redefinition) (ordinal 0)) (authored-target "Message::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Flow::target"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransfer::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::FlowTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::FlowTransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::FlowTransferBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Link"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::Link") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0)) (authored-target "MessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 1)) (authored-target "Transfer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Transfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 1)) (authored-target "Link") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Link")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0)) (authored-target "Flow") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 1)) (authored-target "FlowTransferBefore") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::FlowTransferBefore")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::source") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::source"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransferBefore::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::target") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::SuccessionFlow::target"))) (kind redefinition) (ordinal 1)) (authored-target "FlowTransferBefore::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::binaryLinkObjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::binaryLinkObjects") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flowTransfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfers") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flowTransfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::flowTransfersBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows"))) (kind featureTyping) (ordinal 0)) (authored-target "Flow") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::source") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 1)) (authored-target "messages::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::source"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfers::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 0)) (authored-target "Flow::target") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Flow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 1)) (authored-target "messages::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::flows::target"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfers::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::messages"))) (kind featureTyping) (ordinal 0)) (authored-target "Message") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Message")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows"))) (kind featureTyping) (ordinal 0)) (authored-target "SuccessionFlow") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 0)) (authored-target "SuccessionFlow::source") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 1)) (authored-target "flows::source") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::flows::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::source"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfersBefore::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::Occurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 0)) (authored-target "SuccessionFlow::target") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::SuccessionFlow::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 1)) (authored-target "flows::target") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flows::flows::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Flows::successionFlows::target"))) (kind redefinition) (ordinal 2)) (authored-target "flowTransfersBefore::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flows::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 21 47) (end 21 51)) (probe (position 21 47))
      (reference
        (source (document "d0") (qualified-name "Flows::MessageAction"))
        (kind specialization) (ordinal 1) (authored-target "Link")
        (range (start 21 47) (end 21 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Link") (range (start 7 4) (end 7 31)))
        )
      )
    )
    (query (range (start 85 40) (end 85 44)) (probe (position 85 40))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow"))
        (kind specialization) (ordinal 0) (authored-target "Flow")
        (range (start 85 40) (end 85 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Flow") (range (start 73 4) (end 73 455)))
        )
      )
    )
    (query (range (start 21 39) (end 21 45)) (probe (position 21 39))
      (reference
        (source (document "d0") (qualified-name "Flows::MessageAction"))
        (kind specialization) (ordinal 0) (authored-target "Action")
        (range (start 21 39) (end 21 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Action") (range (start 17 4) (end 17 35)))
        )
      )
    )
    (query (range (start 73 30) (end 73 37)) (probe (position 73 30))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow"))
        (kind specialization) (ordinal 0) (authored-target "Message")
        (range (start 73 30) (end 73 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Message") (range (start 37 4) (end 37 1504)))
        )
      )
    )
    (query (range (start 37 48) (end 37 56)) (probe (position 37 48))
      (reference
        (source (document "d0") (qualified-name "Flows::Message"))
        (kind specialization) (ordinal 1) (authored-target "Transfer")
        (range (start 37 48) (end 37 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Transfer") (range (start 11 4) (end 11 39)))
        )
      )
    )
    (query (range (start 7 19) (end 7 30)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Flows::Link"))
        (kind membershipImport) (ordinal 0) (authored-target "Links::Link")
        (range (start 7 19) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 73 39) (end 73 51)) (probe (position 73 39))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow"))
        (kind specialization) (ordinal 1) (authored-target "FlowTransfer")
        (range (start 73 39) (end 73 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::FlowTransfer") (range (start 13 4) (end 13 43)))
        )
      )
    )
    (query (range (start 94 46) (end 94 58)) (probe (position 94 46))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow::source"))
        (kind redefinition) (ordinal 0) (authored-target "Flow::source")
        (range (start 94 46) (end 94 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Flow::source") (range (start 81 8) (end 81 84)))
        )
      )
    )
    (query (range (start 95 46) (end 95 58)) (probe (position 95 46))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow::target"))
        (kind redefinition) (ordinal 0) (authored-target "Flow::target")
        (range (start 95 46) (end 95 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Flow::target") (range (start 82 8) (end 82 84)))
        )
      )
    )
    (query (range (start 112 46) (end 112 58)) (probe (position 112 46))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::source"))
        (kind redefinition) (ordinal 0) (authored-target "Flow::source")
        (range (start 112 46) (end 112 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Flow::source") (range (start 81 8) (end 81 84)))
        )
      )
    )
    (query (range (start 113 46) (end 113 58)) (probe (position 113 46))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::target"))
        (kind redefinition) (ordinal 0) (authored-target "Flow::target")
        (range (start 113 46) (end 113 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::Flow::target") (range (start 82 8) (end 82 84)))
        )
      )
    )
    (query (range (start 37 33) (end 37 46)) (probe (position 37 33))
      (reference
        (source (document "d0") (qualified-name "Flows::Message"))
        (kind specialization) (ordinal 0) (authored-target "MessageAction")
        (range (start 37 33) (end 37 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::MessageAction") (range (start 21 4) (end 21 436)))
        )
      )
    )
    (query (range (start 122 70) (end 122 83)) (probe (position 122 70))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::source"))
        (kind redefinition) (ordinal 1) (authored-target "flows::source")
        (range (start 122 70) (end 122 83))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::flows::source") (range (start 112 8) (end 112 100)))
        )
      )
    )
    (query (range (start 123 70) (end 123 83)) (probe (position 123 70))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::target"))
        (kind redefinition) (ordinal 1) (authored-target "flows::target")
        (range (start 123 70) (end 123 83))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::flows::target") (range (start 113 8) (end 113 100)))
        )
      )
    )
    (query (range (start 17 19) (end 17 34)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "Flows::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 17 19) (end 17 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 46) (end 81 61)) (probe (position 81 46))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow::source"))
        (kind redefinition) (ordinal 0) (authored-target "Message::source")
        (range (start 81 46) (end 81 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 82 46) (end 82 61)) (probe (position 82 46))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow::target"))
        (kind redefinition) (ordinal 0) (authored-target "Message::target")
        (range (start 82 46) (end 82 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 19) (end 18 35)) (probe (position 18 19))
      (reference
        (source (document "d0") (qualified-name "Flows::actions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
        (range (start 18 19) (end 18 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 60) (end 112 76)) (probe (position 112 60))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::source"))
        (kind redefinition) (ordinal 1) (authored-target "messages::source")
        (range (start 112 60) (end 112 76))
        (outcome (status unresolved))
      )
    )
    (query (range (start 113 60) (end 113 76)) (probe (position 113 60))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::target"))
        (kind redefinition) (ordinal 1) (authored-target "messages::target")
        (range (start 113 60) (end 113 76))
        (outcome (status unresolved))
      )
    )
    (query (range (start 85 46) (end 85 64)) (probe (position 85 46))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow"))
        (kind specialization) (ordinal 1) (authored-target "FlowTransferBefore")
        (range (start 85 46) (end 85 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::FlowTransferBefore") (range (start 15 4) (end 15 49)))
        )
      )
    )
    (query (range (start 11 19) (end 11 38)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "Flows::Transfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
        (range (start 11 19) (end 11 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 39)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "Flows::transfers"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfers")
        (range (start 12 19) (end 12 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 81 63) (end 81 83)) (probe (position 81 63))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow::source"))
        (kind redefinition) (ordinal 1) (authored-target "FlowTransfer::source")
        (range (start 81 63) (end 81 83))
        (outcome (status unresolved))
      )
    )
    (query (range (start 82 63) (end 82 83)) (probe (position 82 63))
      (reference
        (source (document "d0") (qualified-name "Flows::Flow::target"))
        (kind redefinition) (ordinal 1) (authored-target "FlowTransfer::target")
        (range (start 82 63) (end 82 83))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 19) (end 19 40)) (probe (position 19 19))
      (reference
        (source (document "d0") (qualified-name "Flows::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 19 19) (end 19 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 78) (end 112 99)) (probe (position 112 78))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::source"))
        (kind redefinition) (ordinal 2) (authored-target "flowTransfers::source")
        (range (start 112 78) (end 112 99))
        (outcome (status unresolved))
      )
    )
    (query (range (start 113 78) (end 113 99)) (probe (position 113 78))
      (reference
        (source (document "d0") (qualified-name "Flows::flows::target"))
        (kind redefinition) (ordinal 2) (authored-target "flowTransfers::target")
        (range (start 113 78) (end 113 99))
        (outcome (status unresolved))
      )
    )
    (query (range (start 122 46) (end 122 68)) (probe (position 122 46))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::source"))
        (kind redefinition) (ordinal 0) (authored-target "SuccessionFlow::source")
        (range (start 122 46) (end 122 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::SuccessionFlow::source") (range (start 94 8) (end 94 87)))
        )
      )
    )
    (query (range (start 123 46) (end 123 68)) (probe (position 123 46))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::target"))
        (kind redefinition) (ordinal 0) (authored-target "SuccessionFlow::target")
        (range (start 123 46) (end 123 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flows::SuccessionFlow::target") (range (start 95 8) (end 95 87)))
        )
      )
    )
    (query (range (start 8 19) (end 8 42)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Flows::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 19) (end 8 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 19) (end 13 42)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "Flows::FlowTransfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransfer")
        (range (start 13 19) (end 13 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 19) (end 14 43)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "Flows::flowTransfers"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfers")
        (range (start 14 19) (end 14 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 45)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Flows::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 9 19) (end 9 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 45)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "Flows::binaryLinkObjects"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::binaryLinkObjects")
        (range (start 10 19) (end 10 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 60) (end 94 86)) (probe (position 94 60))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow::source"))
        (kind redefinition) (ordinal 1) (authored-target "FlowTransferBefore::source")
        (range (start 94 60) (end 94 86))
        (outcome (status unresolved))
      )
    )
    (query (range (start 95 60) (end 95 86)) (probe (position 95 60))
      (reference
        (source (document "d0") (qualified-name "Flows::SuccessionFlow::target"))
        (kind redefinition) (ordinal 1) (authored-target "FlowTransferBefore::target")
        (range (start 95 60) (end 95 86))
        (outcome (status unresolved))
      )
    )
    (query (range (start 122 85) (end 122 112)) (probe (position 122 85))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::source"))
        (kind redefinition) (ordinal 2) (authored-target "flowTransfersBefore::source")
        (range (start 122 85) (end 122 112))
        (outcome (status unresolved))
      )
    )
    (query (range (start 123 85) (end 123 112)) (probe (position 123 85))
      (reference
        (source (document "d0") (qualified-name "Flows::successionFlows::target"))
        (kind redefinition) (ordinal 2) (authored-target "flowTransfersBefore::target")
        (range (start 123 85) (end 123 112))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 48)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "Flows::FlowTransferBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransferBefore")
        (range (start 15 19) (end 15 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 49)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "Flows::flowTransfersBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfersBefore")
        (range (start 16 19) (end 16 49))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
