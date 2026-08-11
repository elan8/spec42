# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Transfers
type=file
~~~
# SOURCE
~~~kerml
standard library package Transfers {
    doc
    /*
     * This package defines the transfer interactions used to type flows.
     */

    private import Base::Anything;
    private import Occurrences::*;
    private import Links::*;
    private import Objects::BinaryLinkObject;
    private import Performances::Performance;
    private import Performances::performances;
    private import ScalarValues::Boolean;
    private import ScalarValues::Natural;
    private import SequenceFunctions::*;
    
    interaction Transfer specializes Performance, BinaryLink {
        doc
        /*
         * A Transfer represents the transfer of a payload from the source of the interaction 
         * to the target of the interaction.
         */
    
        end feature source: Occurrence redefines BinaryLink::source {
            doc
            /*
             * The entity whose output is the source of the payload to be transferred.
             */
        
            feature sourceOutput: Anything[0..*];
        }
        
        end feature target: Occurrence redefines BinaryLink::target {
            doc
            /*
             * The entity whose input is the target of the payload to be transferred.
             */
        
            feature targetInput: Anything[0..*];
        }
        
        feature isInstant: Boolean[1] {
            doc
            /*
             * If isInstance is true, then the transfer is instantaneous.
             */
        }
        
        feature payload: Anything[1..*] {
            doc
            /*
             * The things that are to be transferred.
             */
        }
        
        feature payloadNum: Natural [1] = size(payload);
        
        private instantNum: Natural[1] = if isInstant? 1 else 0;
        private binding instant[instantNum] of [0..1] startShot = [0..1] endShot {
            doc
            /*
             * If isInstant is true, then the start and end of the transfer happen at the same time.
             */
        }
    }
    
    interaction MessageTransfer specializes Transfer {
        doc
        /*
         * A MessageTransfer is a Transfer that does not specify where the payload is picked
         * up and dropped off (see FlowTransfer). They are sent by SendPerformances and
         * accepted by AcceptPerformances.
         */
    }
     
    interaction FlowTransfer specializes Transfer disjoint from MessageTransfer {
        doc
        /*
         * A FlowTransfer is a Transfer identifying an output feature of the source from which
         * to pick up a payload and an input feature of the target to which to drop it off. They can
         * start when the payload is available at the source and move or copy it to the target.
         */
         
        feature isMove: Boolean[1] default true {
            doc
            /*
             * If isMove is true, then the entire payload leaves the source at the start
             * of the transfer.
             */
        }
        
        feature isPush: Boolean[1] default true {
            doc
            /*
             * If isPush is true, then the transfer begins when the payload is available
             * at the source.
             */
        }
        
        connector sourceOutputLink: BinaryLinkObject[payloadNum] {
            doc
            /*
             * The output of the payloads from the sourceOutput.
             */
        
            end [1] feature transferSource references source;
            end [payloadNum] feature transferPayload references payload subsets transferSource.sourceOutput;
        }
        
        connector targetInputLink: BinaryLinkObject[payloadNum] {
            doc
            /*
             * The input of the payload to the targetInput.
             */
        
            end [1] feature transferTarget references target;
            end [payloadNum] feature transferPayload references payload subsets transferTarget.targetInput;
        }
        
        private connector sending: HappensDuring[payloadNum] from [1] startShot to [payloadNum] sourceOutputLink {
          doc
            /*
             * The start of the transfer happens during the output of each of the payloads from the
             * source. 
             */
        }
        
        private connector moving: HappensWhile[0..*] from [0..*] sourceOutputLink.endShot to [0..1] startShot {
            doc
            /*
             * If isMove is true, then all payloads leave the source at the start
             * of the transfer.
             */
        }
        private inv { isMove implies size(moving) == size(sourceOutputLink) }
        
        private connector pushing: HappensWhile[0..*] from [0..*] sourceOutputLink.startShot to [0..1] startShot {
            doc
            /*
             * If isPush is true, then the transfer begins when the payloads are available
             * at the source.
             */
        }
        private inv { isPush implies size(pushing) == size(sourceOutputLink) }
        
        private connector delivering: HappensWhile[payloadNum] from [payloadNum] targetInputLink.startShot to [1] endShot {
            doc
            /*
             * The input of each of the payloads to the target starts at the end of the transfer.
             */
        }
    }
    
    interaction TransferBefore specializes Transfer, HappensBefore intersects Transfer, HappensBefore {
        doc
        /*
         * TransferBefore is a specialization of Transfer in which the source happens before
         * the transfer, which happens before the target.
         */
    
        end feature source: Occurrence redefines Transfer::source, HappensBefore::earlierOccurrence;
        end feature target: Occurrence redefines Transfer::target, HappensBefore::laterOccurrence;
        
        feature self: TransferBefore redefines Performance::self;
        
        private succession source then self;
        private succession self then target;
    }
    
    interaction FlowTransferBefore specializes TransferBefore, FlowTransfer intersects FlowTransfer, TransferBefore {
        doc
        /*
         * FlowTransferBefore is a FlowTransfer that is also a TransferBefore. 
         */
         
        end feature source: Occurrence redefines Transfer::source, TransferBefore::source;
        end feature target: Occurrence redefines Transfer::target, TransferBefore::target;         
    }
    
    abstract step transfers: Transfer[0..*] nonunique subsets performances, binaryLinks {
        doc
        /*
         * transfers is a specialization of performances and binaryLinks restricted to type 
         * Transfer.
         */
    
        end feature source: Occurrence redefines Transfer::source, binaryLinks::source;
        end feature target: Occurrence redefines Transfer::target, binaryLinks::target;
    }
    
    abstract step messageTransfers: MessageTransfer[0..*] nonunique subsets transfers {
        doc
        /*
         * messageTransfers is a specialization of transfers restricted to type MessageTransfers.
         */
        
        end feature source: Occurrence redefines MessageTransfer::source, transfers::source;
        end feature target: Occurrence redefines MessageTransfer::target, transfers::target;      
    }
    
    abstract flow flowTransfers: FlowTransfer[0..*] nonunique subsets transfers {
        doc
        /*
         * flowTransfers is a specialization of transfers restricted to type FlowTransfers.
         * It is the default subsetting for non-succession flows.
         */
         
        end feature source: Occurrence redefines FlowTransfer::source, transfers::source;
        end feature target: Occurrence redefines FlowTransfer::target, transfers::target;
    }
      
    abstract flow transfersBefore: TransferBefore[0..*] nonunique subsets transfers, happensBeforeLinks
        intersects transfers, happensBeforeLinks {
        doc
        /*
         * transfersBefore is a specialization of transfers and happensBeforeLinks restricted to
         * type TransferBefore.
         */
    
        end feature source: Occurrence redefines TransferBefore::source, transfers::source, happensBeforeLinks::earlierOccurrence;
        end feature target: Occurrence redefines TransferBefore::target, transfers::target, happensBeforeLinks::laterOccurrence;
    }
    
    abstract flow flowTransfersBefore: FlowTransferBefore[0..*] nonunique subsets flowTransfers, transfersBefore
        intersects flowTransfers, transfersBefore {
        doc
        /*
         * flowTransfersBefore is a specialization of flowTransfers and transfersBefore that is restricted
         * to type FlowTransferBefore. IT is the default subsetting for succession flows.
         */
    
        end feature source: Occurrence redefines FlowTransferBefore::source, flowTransfers::source, transfersBefore::source;
        end feature target: Occurrence redefines FlowTransferBefore::target, flowTransfers::target, transfersBefore::target;
    }

    behavior SendPerformance specializes Performance  {
        doc
        /*
         * SendPerformances are Performance that require an outgoingTransferFromSelf 
         * from a designated sender Occurrence carrying a given payload, optionally to a designated receiver.
         */
    
        in feature payload [0..*];
        in feature sender: Occurrence[1] default this;
        in feature receiver: Occurrence[0..1];
        feature sentTransfer: MessageTransfer [1] subsets sender.outgoingTransfersFromSelf {
            feature redefines payload = SendPerformance::payload;
        }
        binding [0..1] receiver.incomingTransfersToSelf = [1] sentTransfer;

        succession self then sentTransfer;
    }
    
    behavior AcceptPerformance specializes Performance {
        doc
        /*
         * AcceptPerformance is a performance that requires an incomingTransferToSelf
         * of a desigated receiver Occurrence, providing its payload as output.
         */
        inout feature payload[0..*];
        in feature receiver: Occurrence[1] default this;
        feature acceptedTransfer: MessageTransfer[1] subsets receiver.incomingTransfersToSelf;
        succession acceptedTransfer then self.endShot;
        
        binding payload = acceptedTransfer.payload;
    }

    abstract step sendPerformances: SendPerformance[0..*] nonunique subsets performances {
        doc
        /*
         * sendPerformances is a specialization of performances for SendPerformances.
         */
    }
        
    abstract step acceptPerformances: AcceptPerformance[0..*] nonunique subsets performances {
        doc
        /*
         * acceptPerformances is a specialization of performances for AcceptPerformances.
         */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "transfers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 33))
      )
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
        (range (start 8 19) (end 8 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 45))
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
        (range (start 13 19) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 8) (end 207 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 207 49) (end 207 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 207 71) (end 207 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 208 8) (end 208 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 208 49) (end 208 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 208 71) (end 208 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 8) (end 219 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 219 49) (end 219 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 219 73) (end 219 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 219 92) (end 219 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 8) (end 220 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 220 49) (end 220 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 220 73) (end 220 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 220 92) (end 220 127))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 8) (end 231 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 231 49) (end 231 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 8) (end 232 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 49) (end 232 75))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Transfers {
    doc
    /*
     * This package defines the transfer interactions used to type flows.
     */

    private import Base::Anything;
    private import Occurrences::*;
    private import Links::*;
    private import Objects::BinaryLinkObject;
    private import Performances::Performance;
    private import Performances::performances;
    private import ScalarValues::Boolean;
    private import ScalarValues::Natural;
    private import SequenceFunctions::*;
    
    interaction Transfer specializes Performance, BinaryLink {
        doc
        /*
         * A Transfer represents the transfer of a payload from the source of the interaction 
         * to the target of the interaction.
         */
    
        end feature source: Occurrence redefines BinaryLink::source {
            doc
            /*
             * The entity whose output is the source of the payload to be transferred.
             */
        
            feature sourceOutput: Anything[0..*];
        }
        
        end feature target: Occurrence redefines BinaryLink::target {
            doc
            /*
             * The entity whose input is the target of the payload to be transferred.
             */
        
            feature targetInput: Anything[0..*];
        }
        
        feature isInstant: Boolean[1] {
            doc
            /*
             * If isInstance is true, then the transfer is instantaneous.
             */
        }
        
        feature payload: Anything[1..*] {
            doc
            /*
             * The things that are to be transferred.
             */
        }
        
        feature payloadNum: Natural [1] = size(payload);
        
        private instantNum: Natural[1] = if isInstant? 1 else 0;
        private binding instant[instantNum] of [0..1] startShot = [0..1] endShot {
            doc
            /*
             * If isInstant is true, then the start and end of the transfer happen at the same time.
             */
        }
    }
    
    interaction MessageTransfer specializes Transfer {
        doc
        /*
         * A MessageTransfer is a Transfer that does not specify where the payload is picked
         * up and dropped off (see FlowTransfer). They are sent by SendPerformances and
         * accepted by AcceptPerformances.
         */
    }
     
    interaction FlowTransfer specializes Transfer disjoint from MessageTransfer {
        doc
        /*
         * A FlowTransfer is a Transfer identifying an output feature of the source from which
         * to pick up a payload and an input feature of the target to which to drop it off. They can
         * start when the payload is available at the source and move or copy it to the target.
         */
         
        feature isMove: Boolean[1] default true {
            doc
            /*
             * If isMove is true, then the entire payload leaves the source at the start
             * of the transfer.
             */
        }
        
        feature isPush: Boolean[1] default true {
            doc
            /*
             * If isPush is true, then the transfer begins when the payload is available
             * at the source.
             */
        }
        
        connector sourceOutputLink: BinaryLinkObject[payloadNum] {
            doc
            /*
             * The output of the payloads from the sourceOutput.
             */
        
            end [1] feature transferSource references source;
            end [payloadNum] feature transferPayload references payload subsets transferSource.sourceOutput;
        }
        
        connector targetInputLink: BinaryLinkObject[payloadNum] {
            doc
            /*
             * The input of the payload to the targetInput.
             */
        
            end [1] feature transferTarget references target;
            end [payloadNum] feature transferPayload references payload subsets transferTarget.targetInput;
        }
        
        private connector sending: HappensDuring[payloadNum] from [1] startShot to [payloadNum] sourceOutputLink {
          doc
            /*
             * The start of the transfer happens during the output of each of the payloads from the
             * source. 
             */
        }
        
        private connector moving: HappensWhile[0..*] from [0..*] sourceOutputLink.endShot to [0..1] startShot {
            doc
            /*
             * If isMove is true, then all payloads leave the source at the start
             * of the transfer.
             */
        }
        private inv { isMove implies size(moving) == size(sourceOutputLink) }
        
        private connector pushing: HappensWhile[0..*] from [0..*] sourceOutputLink.startShot to [0..1] startShot {
            doc
            /*
             * If isPush is true, then the transfer begins when the payloads are available
             * at the source.
             */
        }
        private inv { isPush implies size(pushing) == size(sourceOutputLink) }
        
        private connector delivering: HappensWhile[payloadNum] from [payloadNum] targetInputLink.startShot to [1] endShot {
            doc
            /*
             * The input of each of the payloads to the target starts at the end of the transfer.
             */
        }
    }
    
    interaction TransferBefore specializes Transfer, HappensBefore intersects Transfer, HappensBefore {
        doc
        /*
         * TransferBefore is a specialization of Transfer in which the source happens before
         * the transfer, which happens before the target.
         */
    
        end feature source: Occurrence redefines Transfer::source, HappensBefore::earlierOccurrence;
        end feature target: Occurrence redefines Transfer::target, HappensBefore::laterOccurrence;
        
        feature self: TransferBefore redefines Performance::self;
        
        private succession source then self;
        private succession self then target;
    }
    
    interaction FlowTransferBefore specializes TransferBefore, FlowTransfer intersects FlowTransfer, TransferBefore {
        doc
        /*
         * FlowTransferBefore is a FlowTransfer that is also a TransferBefore. 
         */
         
        end feature source: Occurrence redefines Transfer::source, TransferBefore::source;
        end feature target: Occurrence redefines Transfer::target, TransferBefore::target;         
    }
    
    abstract step transfers: Transfer[0..*] nonunique subsets performances, binaryLinks {
        doc
        /*
         * transfers is a specialization of performances and binaryLinks restricted to type 
         * Transfer.
         */
    
        end feature source: Occurrence redefines Transfer::source, binaryLinks::source;
        end feature target: Occurrence redefines Transfer::target, binaryLinks::target;
    }
    
    abstract step messageTransfers: MessageTransfer[0..*] nonunique subsets transfers {
        doc
        /*
         * messageTransfers is a specialization of transfers restricted to type MessageTransfers.
         */
        
        end feature source: Occurrence redefines MessageTransfer::source, transfers::source;
        end feature target: Occurrence redefines MessageTransfer::target, transfers::target;      
    }
    
    abstract flow flowTransfers: FlowTransfer[0..*] nonunique subsets transfers {
        doc
        /*
         * flowTransfers is a specialization of transfers restricted to type FlowTransfers.
         * It is the default subsetting for non-succession flows.
         */
         
        end feature source: Occurrence redefines FlowTransfer::source, transfers::source;
        end feature target: Occurrence redefines FlowTransfer::target, transfers::target;
    }
      
    abstract flow transfersBefore: TransferBefore[0..*] nonunique subsets transfers, happensBeforeLinks
        intersects transfers, happensBeforeLinks {
        doc
        /*
         * transfersBefore is a specialization of transfers and happensBeforeLinks restricted to
         * type TransferBefore.
         */
    
        end feature source: Occurrence redefines TransferBefore::source, transfers::source, happensBeforeLinks::earlierOccurrence;
        end feature target: Occurrence redefines TransferBefore::target, transfers::target, happensBeforeLinks::laterOccurrence;
    }
    
    abstract flow flowTransfersBefore: FlowTransferBefore[0..*] nonunique subsets flowTransfers, transfersBefore
        intersects flowTransfers, transfersBefore {
        doc
        /*
         * flowTransfersBefore is a specialization of flowTransfers and transfersBefore that is restricted
         * to type FlowTransferBefore. IT is the default subsetting for succession flows.
         */
    
        end feature source: Occurrence redefines FlowTransferBefore::source, flowTransfers::source, transfersBefore::source;
        end feature target: Occurrence redefines FlowTransferBefore::target, flowTransfers::target, transfersBefore::target;
    }

    behavior SendPerformance specializes Performance  {
        doc
        /*
         * SendPerformances are Performance that require an outgoingTransferFromSelf 
         * from a designated sender Occurrence carrying a given payload, optionally to a designated receiver.
         */
    
        in feature payload [0..*];
        in feature sender: Occurrence[1] default this;
        in feature receiver: Occurrence[0..1];
        feature sentTransfer: MessageTransfer [1] subsets sender.outgoingTransfersFromSelf {
            feature redefines payload = SendPerformance::payload;
        }
        binding [0..1] receiver.incomingTransfersToSelf = [1] sentTransfer;

        succession self then sentTransfer;
    }
    
    behavior AcceptPerformance specializes Performance {
        doc
        /*
         * AcceptPerformance is a performance that requires an incomingTransferToSelf
         * of a desigated receiver Occurrence, providing its payload as output.
         */
        inout feature payload[0..*];
        in feature receiver: Occurrence[1] default this;
        feature acceptedTransfer: MessageTransfer[1] subsets receiver.incomingTransfersToSelf;
        succession acceptedTransfer then self.endShot;
        
        binding payload = acceptedTransfer.payload;
    }

    abstract step sendPerformances: SendPerformance[0..*] nonunique subsets performances {
        doc
        /*
         * sendPerformances is a specialization of performances for SendPerformances.
         */
    }
        
    abstract step acceptPerformances: AcceptPerformance[0..*] nonunique subsets performances {
        doc
        /*
         * acceptPerformances is a specialization of performances for AcceptPerformances.
         */
    }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ca94de06a2542d93acbee9d606cf43d940a2164d57af074965dc52ad53af5cc2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Transfers"))) (kind "package") (name "Transfers") (declared-name "Transfers") (range (start (line 0) (character 0)) (end (line 0) (character 10817))))
    (element (id (node (document "d0") (qualified-name "Transfers::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 4)) (end (line 7) (character 34))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Transfers::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 4)) (end (line 8) (character 28))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 24))))))
    (element (id (node (document "d0") (qualified-name "Transfers::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 14) (character 4)) (end (line 14) (character 40))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Transfers::AcceptPerformance"))) (kind "kermlDecl") (name "AcceptPerformance") (declared-name "AcceptPerformance") (range (start (line 253) (character 4)) (end (line 253) (character 568))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 4)) (end (line 6) (character 34))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Transfers::BinaryLinkObject"))) (kind "import") (name "BinaryLinkObject") (declared-name "BinaryLinkObject") (range (start (line 9) (character 4)) (end (line 9) (character 45))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::BinaryLinkObject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Transfers::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 12) (character 4)) (end (line 12) (character 41))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Transfers::FlowTransfer"))) (kind "kermlDecl") (name "FlowTransfer") (declared-name "FlowTransfer") (range (start (line 75) (character 4)) (end (line 75) (character 2978))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::FlowTransferBefore"))) (kind "kermlDecl") (name "FlowTransferBefore") (declared-name "FlowTransferBefore") (range (start (line 169) (character 4)) (end (line 169) (character 439))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::MessageTransfer"))) (kind "kermlDecl") (name "MessageTransfer") (declared-name "MessageTransfer") (range (start (line 66) (character 4)) (end (line 66) (character 319))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 13) (character 4)) (end (line 13) (character 41))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 19)) (end (line 13) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Transfers::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (range (start (line 10) (character 4)) (end (line 10) (character 45))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Transfers::SendPerformance"))) (kind "kermlDecl") (name "SendPerformance") (declared-name "SendPerformance") (range (start (line 235) (character 4)) (end (line 235) (character 723))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::Transfer"))) (kind "kermlDecl") (name "Transfer") (declared-name "Transfer") (range (start (line 16) (character 4)) (end (line 16) (character 1524))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::TransferBefore"))) (kind "kermlDecl") (name "TransferBefore") (declared-name "TransferBefore") (range (start (line 153) (character 4)) (end (line 153) (character 674))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 10817))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::acceptPerformances"))) (kind "kermlDecl") (name "acceptPerformances") (declared-name "acceptPerformances") (range (start (line 274) (character 4)) (end (line 274) (character 225))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (kind "flow") (name "flowTransfers") (declared-name "flowTransfers") (range (start (line 200) (character 4)) (end (line 200) (character 470))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Feature)) (relationships (typing (reference "FlowTransfer") (range none)))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfers::_documentation"))) (kind "documentation") (name "") (range (start (line 200) (character 4)) (end (line 200) (character 470))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfers::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 207) (character 8)) (end (line 207) (character 89))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "FlowTransfer::source") (range (start (line 207) (character 49)) (end (line 207) (character 69)))) (redefinition (reference "transfers::source") (range (start (line 207) (character 71)) (end (line 207) (character 88)))))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfers::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 208) (character 8)) (end (line 208) (character 89))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "FlowTransfer::target") (range (start (line 208) (character 49)) (end (line 208) (character 69)))) (redefinition (reference "transfers::target") (range (start (line 208) (character 71)) (end (line 208) (character 88)))))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (kind "flow") (name "flowTransfersBefore") (declared-name "flowTransfersBefore") (range (start (line 223) (character 4)) (end (line 223) (character 657))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Feature)) (relationships (typing (reference "FlowTransferBefore") (range none)))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::_documentation"))) (kind "documentation") (name "") (range (start (line 223) (character 4)) (end (line 223) (character 657))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 231) (character 8)) (end (line 231) (character 124))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "FlowTransferBefore::source") (range (start (line 231) (character 49)) (end (line 231) (character 75)))) (redefinition (reference "flowTransfers::source") (range (start (line 231) (character 77)) (end (line 231) (character 98)))) (redefinition (reference "transfersBefore::source") (range (start (line 231) (character 100)) (end (line 231) (character 123)))))))
    (element (id (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 232) (character 8)) (end (line 232) (character 124))) (parent (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "FlowTransferBefore::target") (range (start (line 232) (character 49)) (end (line 232) (character 75)))) (redefinition (reference "flowTransfers::target") (range (start (line 232) (character 77)) (end (line 232) (character 98)))) (redefinition (reference "transfersBefore::target") (range (start (line 232) (character 100)) (end (line 232) (character 123)))))))
    (element (id (node (document "d0") (qualified-name "Transfers::messageTransfers"))) (kind "kermlDecl") (name "messageTransfers") (declared-name "messageTransfers") (range (start (line 190) (character 4)) (end (line 190) (character 427))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::performances"))) (kind "import") (name "performances") (declared-name "performances") (range (start (line 11) (character 4)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::performances") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Transfers::sendPerformances"))) (kind "kermlDecl") (name "sendPerformances") (declared-name "sendPerformances") (range (start (line 267) (character 4)) (end (line 267) (character 217))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::transfers"))) (kind "kermlDecl") (name "transfers") (declared-name "transfers") (range (start (line 179) (character 4)) (end (line 179) (character 425))) (parent (node (document "d0") (qualified-name "Transfers"))))
    (element (id (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (kind "flow") (name "transfersBefore") (declared-name "transfersBefore") (range (start (line 211) (character 4)) (end (line 211) (character 589))) (parent (node (document "d0") (qualified-name "Transfers"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransferBefore") (range none)))))
    (element (id (node (document "d0") (qualified-name "Transfers::transfersBefore::_documentation"))) (kind "documentation") (name "") (range (start (line 211) (character 4)) (end (line 211) (character 589))) (parent (node (document "d0") (qualified-name "Transfers::transfersBefore"))))
    (element (id (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 219) (character 8)) (end (line 219) (character 130))) (parent (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "TransferBefore::source") (range (start (line 219) (character 49)) (end (line 219) (character 71)))) (redefinition (reference "transfers::source") (range (start (line 219) (character 73)) (end (line 219) (character 90)))) (redefinition (reference "happensBeforeLinks::earlierOccurrence") (range (start (line 219) (character 92)) (end (line 219) (character 129)))))))
    (element (id (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 220) (character 8)) (end (line 220) (character 128))) (parent (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (authored (relationships (typing (reference "Occurrence") (range none)) (redefinition (reference "TransferBefore::target") (range (start (line 220) (character 49)) (end (line 220) (character 71)))) (redefinition (reference "transfers::target") (range (start (line 220) (character 73)) (end (line 220) (character 90)))) (redefinition (reference "happensBeforeLinks::laterOccurrence") (range (start (line 220) (character 92)) (end (line 220) (character 127)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Transfers::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Occurrences::*") (range (start (line 7) (character 19)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Links::*") (range (start (line 8) (character 19)) (end (line 8) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequenceFunctions::*") (range (start (line 14) (character 19)) (end (line 14) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 19)) (end (line 6) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::BinaryLinkObject"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::BinaryLinkObject") (range (start (line 9) (character 19)) (end (line 9) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 12) (character 19)) (end (line 12) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 13) (character 19)) (end (line 13) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (range (start (line 10) (character 19)) (end (line 10) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (kind featureTyping) (ordinal 0)) (authored-target "FlowTransfer") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::FlowTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::source"))) (kind redefinition) (ordinal 0)) (authored-target "FlowTransfer::source") (range (start (line 207) (character 49)) (end (line 207) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::source"))) (kind redefinition) (ordinal 1)) (authored-target "transfers::source") (range (start (line 207) (character 71)) (end (line 207) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::target"))) (kind redefinition) (ordinal 0)) (authored-target "FlowTransfer::target") (range (start (line 208) (character 49)) (end (line 208) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfers::target"))) (kind redefinition) (ordinal 1)) (authored-target "transfers::target") (range (start (line 208) (character 71)) (end (line 208) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (kind featureTyping) (ordinal 0)) (authored-target "FlowTransferBefore") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::FlowTransferBefore")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind redefinition) (ordinal 0)) (authored-target "FlowTransferBefore::source") (range (start (line 231) (character 49)) (end (line 231) (character 75))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind redefinition) (ordinal 1)) (authored-target "flowTransfers::source") (range (start (line 231) (character 77)) (end (line 231) (character 98))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::flowTransfers::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind redefinition) (ordinal 2)) (authored-target "transfersBefore::source") (range (start (line 231) (character 100)) (end (line 231) (character 123))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::transfersBefore::source")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind redefinition) (ordinal 0)) (authored-target "FlowTransferBefore::target") (range (start (line 232) (character 49)) (end (line 232) (character 75))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind redefinition) (ordinal 1)) (authored-target "flowTransfers::target") (range (start (line 232) (character 77)) (end (line 232) (character 98))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::flowTransfers::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind redefinition) (ordinal 2)) (authored-target "transfersBefore::target") (range (start (line 232) (character 100)) (end (line 232) (character 123))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::transfersBefore::target")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::performances"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::performances") (range (start (line 11) (character 19)) (end (line 11) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (kind featureTyping) (ordinal 0)) (authored-target "TransferBefore") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Transfers::TransferBefore")))))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (kind redefinition) (ordinal 0)) (authored-target "TransferBefore::source") (range (start (line 219) (character 49)) (end (line 219) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (kind redefinition) (ordinal 1)) (authored-target "transfers::source") (range (start (line 219) (character 73)) (end (line 219) (character 90))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (kind redefinition) (ordinal 2)) (authored-target "happensBeforeLinks::earlierOccurrence") (range (start (line 219) (character 92)) (end (line 219) (character 129))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (kind redefinition) (ordinal 0)) (authored-target "TransferBefore::target") (range (start (line 220) (character 49)) (end (line 220) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (kind redefinition) (ordinal 1)) (authored-target "transfers::target") (range (start (line 220) (character 73)) (end (line 220) (character 90))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (kind redefinition) (ordinal 2)) (authored-target "happensBeforeLinks::laterOccurrence") (range (start (line 220) (character 92)) (end (line 220) (character 127))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (target (node (document "d0") (qualified-name "Transfers::FlowTransfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (target (node (document "d0") (qualified-name "Transfers::FlowTransferBefore"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (target (node (document "d0") (qualified-name "Transfers::flowTransfers::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind redefinition) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (target (node (document "d0") (qualified-name "Transfers::transfersBefore::source"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))) (kind redefinition) (ordinal 2)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (target (node (document "d0") (qualified-name "Transfers::flowTransfers::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind redefinition) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (target (node (document "d0") (qualified-name "Transfers::transfersBefore::target"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))) (kind redefinition) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (target (node (document "d0") (qualified-name "Transfers::TransferBefore"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transfers::transfersBefore"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 19) (end 8 24)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Links::*")
        (range (start 8 19) (end 8 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 30)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Occurrences::*")
        (range (start 7 19) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 33)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 19) (end 6 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 19) (end 14 36)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions::*")
        (range (start 14 19) (end 14 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 207 71) (end 207 88)) (probe (position 207 71))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfers::source"))
        (kind redefinition) (ordinal 1) (authored-target "transfers::source")
        (range (start 207 71) (end 207 88))
        (outcome (status unresolved))
      )
    )
    (query (range (start 208 71) (end 208 88)) (probe (position 208 71))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfers::target"))
        (kind redefinition) (ordinal 1) (authored-target "transfers::target")
        (range (start 208 71) (end 208 88))
        (outcome (status unresolved))
      )
    )
    (query (range (start 219 73) (end 219 90)) (probe (position 219 73))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::source"))
        (kind redefinition) (ordinal 1) (authored-target "transfers::source")
        (range (start 219 73) (end 219 90))
        (outcome (status unresolved))
      )
    )
    (query (range (start 220 73) (end 220 90)) (probe (position 220 73))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::target"))
        (kind redefinition) (ordinal 1) (authored-target "transfers::target")
        (range (start 220 73) (end 220 90))
        (outcome (status unresolved))
      )
    )
    (query (range (start 207 49) (end 207 69)) (probe (position 207 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfers::source"))
        (kind redefinition) (ordinal 0) (authored-target "FlowTransfer::source")
        (range (start 207 49) (end 207 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 208 49) (end 208 69)) (probe (position 208 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfers::target"))
        (kind redefinition) (ordinal 0) (authored-target "FlowTransfer::target")
        (range (start 208 49) (end 208 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 40)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 12 19) (end 12 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 19) (end 13 40)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 13 19) (end 13 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 231 77) (end 231 98)) (probe (position 231 77))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))
        (kind redefinition) (ordinal 1) (authored-target "flowTransfers::source")
        (range (start 231 77) (end 231 98))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Transfers::flowTransfers::source") (range (start 207 8) (end 207 89)))
        )
      )
    )
    (query (range (start 232 77) (end 232 98)) (probe (position 232 77))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))
        (kind redefinition) (ordinal 1) (authored-target "flowTransfers::target")
        (range (start 232 77) (end 232 98))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Transfers::flowTransfers::target") (range (start 208 8) (end 208 89)))
        )
      )
    )
    (query (range (start 219 49) (end 219 71)) (probe (position 219 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::source"))
        (kind redefinition) (ordinal 0) (authored-target "TransferBefore::source")
        (range (start 219 49) (end 219 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 220 49) (end 220 71)) (probe (position 220 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::target"))
        (kind redefinition) (ordinal 0) (authored-target "TransferBefore::target")
        (range (start 220 49) (end 220 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 231 100) (end 231 123)) (probe (position 231 100))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))
        (kind redefinition) (ordinal 2) (authored-target "transfersBefore::source")
        (range (start 231 100) (end 231 123))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Transfers::transfersBefore::source") (range (start 219 8) (end 219 130)))
        )
      )
    )
    (query (range (start 232 100) (end 232 123)) (probe (position 232 100))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))
        (kind redefinition) (ordinal 2) (authored-target "transfersBefore::target")
        (range (start 232 100) (end 232 123))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Transfers::transfersBefore::target") (range (start 220 8) (end 220 128)))
        )
      )
    )
    (query (range (start 9 19) (end 9 44)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::BinaryLinkObject"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::BinaryLinkObject")
        (range (start 9 19) (end 9 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 44)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 10 19) (end 10 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 45)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "Transfers::performances"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
        (range (start 11 19) (end 11 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 231 49) (end 231 75)) (probe (position 231 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::source"))
        (kind redefinition) (ordinal 0) (authored-target "FlowTransferBefore::source")
        (range (start 231 49) (end 231 75))
        (outcome (status unresolved))
      )
    )
    (query (range (start 232 49) (end 232 75)) (probe (position 232 49))
      (reference
        (source (document "d0") (qualified-name "Transfers::flowTransfersBefore::target"))
        (kind redefinition) (ordinal 0) (authored-target "FlowTransferBefore::target")
        (range (start 232 49) (end 232 75))
        (outcome (status unresolved))
      )
    )
    (query (range (start 220 92) (end 220 127)) (probe (position 220 92))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::target"))
        (kind redefinition) (ordinal 2) (authored-target "happensBeforeLinks::laterOccurrence")
        (range (start 220 92) (end 220 127))
        (outcome (status unresolved))
      )
    )
    (query (range (start 219 92) (end 219 129)) (probe (position 219 92))
      (reference
        (source (document "d0") (qualified-name "Transfers::transfersBefore::source"))
        (kind redefinition) (ordinal 2) (authored-target "happensBeforeLinks::earlierOccurrence")
        (range (start 219 92) (end 219 129))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
