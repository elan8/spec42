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
  (document "memory://snapshot/flows.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 39) (end 21 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 47) (end 21 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 29 9) (end 34 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 37 48) (end 37 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 45 8) (end 45 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 47 8) (end 47 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 49 8) (end 55 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 56 8) (end 62 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 64 8) (end 64 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 65 8) (end 65 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 67 8) (end 67 100))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 68 8) (end 68 98))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 69 8) (end 69 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 70 8) (end 70 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 73 39) (end 73 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 31) (end 81 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 31) (end 82 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 85 46) (end 85 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 92 8) (end 92 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 31) (end 94 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 31) (end 95 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 98 4) (end 103 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 105 4) (end 114 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 116 4) (end 124 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c9a64a01c4c777e61a25df439dd681fafb88806988181a2c8727245e55eb87bb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n     * This package defines the base types for flows and related behavioral elements \n     * in the SysML language.\n     "))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Links::Link") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensDuring") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::binaryLinkObjects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::Transfer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::transfers") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::FlowTransfer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::flowTransfers") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::FlowTransferBefore") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::flowTransfersBefore") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind flow-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * Flow is a subclass of messages that are also flow transfers.\n         * It is the base type for FlowUsages that identify their source output and\n         * target input.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Message")) (specialization (reference "FlowTransfer")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind flow-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * Message is the subclass of message connections that represent \n         * a transfer of objects or values between two occurrences. It is \n         * the base type of all FlowUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MessageAction")) (specialization (reference "Transfer")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (kind flow-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * MessageAction is the most general class of actions that represent\n         * interactions between linked things. It is the base type of all\n         * FlowDefinitions.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "Link")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind flow-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * SuccessionFlow is a subclass of flowss that appen after their source and \n         * before their target. It is the base type for all SuccessionFlowUsages.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Flow")) (specialization (reference "FlowTransferBefore")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Links::Link")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::binaryLinkObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::FlowTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::flowTransfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::FlowTransferBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::flowTransfersBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message")))))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 1))
      (authored-target "FlowTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0))
      (authored-target "MessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction")))))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 1))
      (authored-target "Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 1))
      (authored-target "Link")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0))
      (authored-target "Flow")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow")))))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 1))
      (authored-target "FlowTransferBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::source"))) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::source"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::target"))) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::target"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow")))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message")))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow")))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::source")))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::target")))
      (supertype (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/flows.md") (range (start 7 19) (end 7 30)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Links::Link")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 8 19) (end 8 42)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 9 19) (end 9 45)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 10 19) (end 10 45)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::binaryLinkObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 11 19) (end 11 38)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 12 19) (end 12 39)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 13 19) (end 13 42)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 14 19) (end 14 43)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfers")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 15 19) (end 15 48)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::FlowTransferBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 16 19) (end 16 49)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::flowTransfersBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 17 19) (end 17 34)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 18 19) (end 18 35)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 19 19) (end 19 40)) (probe (position 19 19))
    (reference (id (source (node (document "memory://snapshot/flows.md") (path (named (kind library-package) (name "Flows")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 73 30) (end 73 37)) (probe (position 73 30))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message")))))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 73 39) (end 73 51)) (probe (position 73 39))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow"))) (kind specialization) (ordinal 1) (authored-target "FlowTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 81 31) (end 81 41)) (probe (position 81 31))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 82 31) (end 82 41)) (probe (position 82 31))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 37 33) (end 37 46)) (probe (position 37 33))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 0) (authored-target "MessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction")))))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 37 48) (end 37 56)) (probe (position 37 48))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Message"))) (kind specialization) (ordinal 1) (authored-target "Transfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 21 39) (end 21 45)) (probe (position 21 39))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 21 47) (end 21 51)) (probe (position 21 47))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::MessageAction"))) (kind specialization) (ordinal 1) (authored-target "Link")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 85 40) (end 85 44)) (probe (position 85 40))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 0) (authored-target "Flow")
      (outcome (status resolved) (target (node (document "memory://snapshot/flows.md") (qualified-name "Flows::Flow")))))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 85 46) (end 85 64)) (probe (position 85 46))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow"))) (kind specialization) (ordinal 1) (authored-target "FlowTransferBefore")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 94 31) (end 94 41)) (probe (position 94 31))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/flows.md") (range (start 95 31) (end 95 41)) (probe (position 95 31))
    (reference (id (source (node (document "memory://snapshot/flows.md") (qualified-name "Flows::SuccessionFlow::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
)
~~~
