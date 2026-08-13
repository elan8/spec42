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
  (document "memory://snapshot/transfers.md"
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
        (range (start 7 19) (end 7 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 27))
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
        (range (start 14 19) (end 14 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 37) (end 16 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 50) (end 16 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 28) (end 23 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 49) (end 23 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 34) (end 29 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 28) (end 32 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 49) (end 32 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 33) (end 38 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 27) (end 41 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 25) (end 48 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 28) (end 55 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 42) (end 55 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 28) (end 57 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 41) (end 57 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 54) (end 58 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 73) (end 58 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 24) (end 83 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 24) (end 91 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 36) (end 99 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 105 12) (end 105 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 106 12) (end 106 108))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 35) (end 109 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 115 12) (end 115 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 116 12) (end 116 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 35) (end 119 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 70) (end 119 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 34) (end 127 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 65) (end 127 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 100) (end 127 109))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 134 8) (end 134 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 134 22) (end 134 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 35) (end 136 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 136 66) (end 136 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 136 103) (end 136 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 8) (end 143 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 143 22) (end 143 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 38) (end 145 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 145 81) (end 145 106))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 145 114) (end 145 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 153 53) (end 153 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 28) (end 160 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 160 67) (end 160 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 28) (end 161 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 67) (end 161 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 163 47) (end 163 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 165 8) (end 165 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 166 8) (end 166 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 28) (end 175 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 28) (end 176 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 62) (end 179 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 76) (end 179 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 186 28) (end 186 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 186 67) (end 186 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 28) (end 187 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 187 67) (end 187 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 196 28) (end 196 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 49) (end 196 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 28) (end 197 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 197 49) (end 197 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 200 4) (end 209 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 211 4) (end 221 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 223 4) (end 233 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 235 41) (end 235 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 242 8) (end 242 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 243 8) (end 243 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 244 8) (end 244 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 245 58) (end 245 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 246 40) (end 246 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 248 23) (end 248 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 250 8) (end 250 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 253 43) (end 253 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 259 8) (end 259 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 260 8) (end 260 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 261 61) (end 261 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 262 8) (end 262 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 264 16) (end 264 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 264 26) (end 264 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 267 76) (end 267 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 274 80) (end 274 92))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:0443af57b8b05ed7d9f38e992d07bcea4a0faa7fc342ee027ba05614ecd1859c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Occurrences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Links") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::BinaryLinkObject") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Performance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::performances") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SequenceFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "payload")) (bindTarget (reference "acceptedTransfer::payload"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (subsetting (reference "receiver::incomingTransfersToSelf"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transfer")) (expressionOperand (reference "private")) (expressionOperand (reference "private"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-invariant) (ordinal 1))))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensWhile")) (connectorEnd (reference "targetInputLink::startShot")) (connectorEnd (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isMove"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isPush"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensWhile")) (connectorEnd (reference "sourceOutputLink::endShot")) (connectorEnd (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensWhile")) (connectorEnd (reference "sourceOutputLink::startShot")) (connectorEnd (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")) (connectorEnd (reference "startShot")) (connectorEnd (reference "sourceOutputLink"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BinaryLinkObject"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::targetInputLink"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BinaryLinkObject"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TransferBefore")) (specialization (reference "FlowTransfer"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::source")) (redefinition (reference "TransferBefore::source"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::target")) (redefinition (reference "TransferBefore::target"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transfer"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "receiver::incomingTransfersToSelf")) (bindTarget (reference "sentTransfer"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (subsetting (reference "sender::outgoingTransfersFromSelf"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "payload")) (expressionOperand (reference "SendPerformance::payload"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance")) (specialization (reference "BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instant"))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "startShot")) (bindTarget (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instantNum"))) (kind default-reference) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::isInstant"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural")) (expressionOperand (reference "payload")) (invocationCallee (reference "size"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::source"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::target"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transfer")) (specialization (reference "HappensBefore"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransferBefore")) (redefinition (reference "Performance::self"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::source")) (redefinition (reference "HappensBefore::earlierOccurrence"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::target")) (redefinition (reference "HappensBefore::laterOccurrence"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptPerformance")) (subsetting (reference "performances"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (subsetting (reference "transfers"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "MessageTransfer::source")) (redefinition (reference "transfers::source"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "MessageTransfer::target")) (redefinition (reference "transfers::target"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SendPerformance")) (subsetting (reference "performances"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfer")) (subsetting (reference "performances")) (subsetting (reference "binaryLinks"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::source")) (redefinition (reference "binaryLinks::source"))))
    (declaration (id (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "Transfer::target")) (redefinition (reference "binaryLinks::target"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Occurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Links")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::BinaryLinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "acceptedTransfer::payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind subsetting) (ordinal 0))
      (authored-target "receiver::incomingTransfersToSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind specialization) (ordinal 0))
      (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind expressionOperand) (ordinal 1))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind connectorEnd) (ordinal 0))
      (authored-target "targetInputLink::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind connectorEnd) (ordinal 1))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isMove"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isPush"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind connectorEnd) (ordinal 0))
      (authored-target "sourceOutputLink::endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind connectorEnd) (ordinal 1))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind connectorEnd) (ordinal 0))
      (authored-target "sourceOutputLink::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind connectorEnd) (ordinal 1))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind connectorEnd) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind connectorEnd) (ordinal 1))
      (authored-target "sourceOutputLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink"))) (kind featureTyping) (ordinal 0))
      (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::targetInputLink"))) (kind featureTyping) (ordinal 0))
      (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 0))
      (authored-target "TransferBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 1))
      (authored-target "FlowTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 1))
      (authored-target "TransferBefore::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 1))
      (authored-target "TransferBefore::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (kind specialization) (ordinal 0))
      (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "receiver::incomingTransfersToSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "sentTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind subsetting) (ordinal 0))
      (authored-target "sender::outgoingTransfersFromSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "SendPerformance::payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (kind specialization) (ordinal 0))
      (authored-target "Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (kind specialization) (ordinal 1))
      (authored-target "BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instant"))) (kind bindSource) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instant"))) (kind bindTarget) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instantNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::isInstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind expressionOperand) (ordinal 0))
      (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind specialization) (ordinal 0))
      (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind specialization) (ordinal 1))
      (authored-target "HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransferBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Performance::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind redefinition) (ordinal 1))
      (authored-target "HappensBefore::earlierOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind redefinition) (ordinal 1))
      (authored-target "HappensBefore::laterOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcceptPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind subsetting) (ordinal 0))
      (authored-target "performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind subsetting) (ordinal 0))
      (authored-target "transfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind redefinition) (ordinal 0))
      (authored-target "MessageTransfer::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind redefinition) (ordinal 1))
      (authored-target "transfers::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind redefinition) (ordinal 0))
      (authored-target "MessageTransfer::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind redefinition) (ordinal 1))
      (authored-target "transfers::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind featureTyping) (ordinal 0))
      (authored-target "SendPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind subsetting) (ordinal 0))
      (authored-target "performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind subsetting) (ordinal 0))
      (authored-target "performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind subsetting) (ordinal 1))
      (authored-target "binaryLinks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind redefinition) (ordinal 1))
      (authored-target "binaryLinks::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind redefinition) (ordinal 0))
      (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind redefinition) (ordinal 1))
      (authored-target "binaryLinks::target")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind specialization) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (kind specialization) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind redefinition) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind redefinition) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isMove"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isPush"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/transfers.md") (range (start 7 19) (end 7 33)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Occurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 8 19) (end 8 27)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Links")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 14 19) (end 14 39)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0) (authored-target "SequenceFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 6 19) (end 6 33)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 9 19) (end 9 44)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::BinaryLinkObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 10 19) (end 10 44)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 11 19) (end 11 45)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 12 19) (end 12 40)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 13 19) (end 13 40)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 253 43) (end 253 54)) (probe (position 253 43))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 264 16) (end 264 23)) (probe (position 264 16))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "payload")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 264 26) (end 264 50)) (probe (position 264 26))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "acceptedTransfer::payload")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 261 34) (end 261 49)) (probe (position 261 34))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 261 61) (end 261 93)) (probe (position 261 61))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance::acceptedTransfer"))) (kind subsetting) (ordinal 0) (authored-target "receiver::incomingTransfersToSelf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 75 41) (end 75 49)) (probe (position 75 41))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind specialization) (ordinal 0) (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 134 8) (end 134 15)) (probe (position 134 8))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 143 8) (end 143 15)) (probe (position 143 8))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer"))) (kind expressionOperand) (ordinal 1) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 145 38) (end 145 50)) (probe (position 145 38))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind featureTyping) (ordinal 0) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 145 81) (end 145 106)) (probe (position 145 81))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind connectorEnd) (ordinal 0) (authored-target "targetInputLink::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 145 114) (end 145 121)) (probe (position 145 114))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::delivering"))) (kind connectorEnd) (ordinal 1) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 83 24) (end 83 31)) (probe (position 83 24))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isMove"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 91 24) (end 91 31)) (probe (position 91 24))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::isPush"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 127 34) (end 127 46)) (probe (position 127 34))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind featureTyping) (ordinal 0) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 127 65) (end 127 89)) (probe (position 127 65))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind connectorEnd) (ordinal 0) (authored-target "sourceOutputLink::endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 127 100) (end 127 109)) (probe (position 127 100))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::moving"))) (kind connectorEnd) (ordinal 1) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 136 35) (end 136 47)) (probe (position 136 35))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind featureTyping) (ordinal 0) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 136 66) (end 136 92)) (probe (position 136 66))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind connectorEnd) (ordinal 0) (authored-target "sourceOutputLink::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 136 103) (end 136 112)) (probe (position 136 103))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::pushing"))) (kind connectorEnd) (ordinal 1) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 119 35) (end 119 48)) (probe (position 119 35))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 119 70) (end 119 79)) (probe (position 119 70))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind connectorEnd) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 119 96) (end 119 112)) (probe (position 119 96))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sending"))) (kind connectorEnd) (ordinal 1) (authored-target "sourceOutputLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 99 36) (end 99 52)) (probe (position 99 36))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::sourceOutputLink"))) (kind featureTyping) (ordinal 0) (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 109 35) (end 109 51)) (probe (position 109 35))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer::targetInputLink"))) (kind featureTyping) (ordinal 0) (authored-target "BinaryLinkObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 169 47) (end 169 61)) (probe (position 169 47))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 0) (authored-target "TransferBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 169 63) (end 169 75)) (probe (position 169 63))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore"))) (kind specialization) (ordinal 1) (authored-target "FlowTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 175 28) (end 175 38)) (probe (position 175 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 175 49) (end 175 65)) (probe (position 175 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 175 67) (end 175 89)) (probe (position 175 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::source"))) (kind redefinition) (ordinal 1) (authored-target "TransferBefore::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 176 28) (end 176 38)) (probe (position 176 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 176 49) (end 176 65)) (probe (position 176 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 176 67) (end 176 89)) (probe (position 176 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::FlowTransferBefore::target"))) (kind redefinition) (ordinal 1) (authored-target "TransferBefore::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 66 44) (end 66 52)) (probe (position 66 44))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer"))) (kind specialization) (ordinal 0) (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 235 41) (end 235 52)) (probe (position 235 41))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 248 23) (end 248 55)) (probe (position 248 23))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "receiver::incomingTransfersToSelf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 248 62) (end 248 74)) (probe (position 248 62))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "sentTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 245 30) (end 245 45)) (probe (position 245 30))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 245 58) (end 245 90)) (probe (position 245 58))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance::sentTransfer"))) (kind subsetting) (ordinal 0) (authored-target "sender::outgoingTransfersFromSelf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 246 30) (end 246 37)) (probe (position 246 30))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 246 40) (end 246 64)) (probe (position 246 40))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "SendPerformance::payload")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 16 37) (end 16 48)) (probe (position 16 37))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 16 50) (end 16 60)) (probe (position 16 50))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer"))) (kind specialization) (ordinal 1) (authored-target "BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 58 54) (end 58 63)) (probe (position 58 54))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instant"))) (kind bindSource) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 58 73) (end 58 80)) (probe (position 58 73))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instant"))) (kind bindTarget) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 57 28) (end 57 35)) (probe (position 57 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::instantNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 41 27) (end 41 34)) (probe (position 41 27))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::isInstant"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 48 25) (end 48 33)) (probe (position 48 25))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 55 28) (end 55 35)) (probe (position 55 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 55 47) (end 55 54)) (probe (position 55 47))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind expressionOperand) (ordinal 0) (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payload")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 55 42) (end 55 46)) (probe (position 55 42))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::payloadNum"))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 23 28) (end 23 38)) (probe (position 23 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 23 49) (end 23 67)) (probe (position 23 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 29 34) (end 29 42)) (probe (position 29 34))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 32 28) (end 32 38)) (probe (position 32 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 32 49) (end 32 67)) (probe (position 32 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 38 33) (end 38 41)) (probe (position 38 33))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 153 43) (end 153 51)) (probe (position 153 43))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind specialization) (ordinal 0) (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 153 53) (end 153 66)) (probe (position 153 53))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore"))) (kind specialization) (ordinal 1) (authored-target "HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 163 22) (end 163 36)) (probe (position 163 22))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind featureTyping) (ordinal 0) (authored-target "TransferBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 163 47) (end 163 64)) (probe (position 163 47))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::self"))) (kind redefinition) (ordinal 0) (authored-target "Performance::self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 160 28) (end 160 38)) (probe (position 160 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 160 49) (end 160 65)) (probe (position 160 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 160 67) (end 160 99)) (probe (position 160 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::source"))) (kind redefinition) (ordinal 1) (authored-target "HappensBefore::earlierOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 161 28) (end 161 38)) (probe (position 161 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 161 49) (end 161 65)) (probe (position 161 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 161 67) (end 161 97)) (probe (position 161 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::TransferBefore::target"))) (kind redefinition) (ordinal 1) (authored-target "HappensBefore::laterOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 274 38) (end 274 55)) (probe (position 274 38))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::AcceptPerformance")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 274 80) (end 274 92)) (probe (position 274 80))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::acceptPerformances"))) (kind subsetting) (ordinal 0) (authored-target "performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 190 36) (end 190 51)) (probe (position 190 36))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::MessageTransfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 190 76) (end 190 85)) (probe (position 190 76))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers"))) (kind subsetting) (ordinal 0) (authored-target "transfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 196 28) (end 196 38)) (probe (position 196 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 196 49) (end 196 72)) (probe (position 196 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind redefinition) (ordinal 0) (authored-target "MessageTransfer::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 196 74) (end 196 91)) (probe (position 196 74))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::source"))) (kind redefinition) (ordinal 1) (authored-target "transfers::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 197 28) (end 197 38)) (probe (position 197 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 197 49) (end 197 72)) (probe (position 197 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind redefinition) (ordinal 0) (authored-target "MessageTransfer::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 197 74) (end 197 91)) (probe (position 197 74))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::messageTransfers::target"))) (kind redefinition) (ordinal 1) (authored-target "transfers::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 267 36) (end 267 51)) (probe (position 267 36))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind featureTyping) (ordinal 0) (authored-target "SendPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::SendPerformance")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 267 76) (end 267 88)) (probe (position 267 76))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::sendPerformances"))) (kind subsetting) (ordinal 0) (authored-target "performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 179 29) (end 179 37)) (probe (position 179 29))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind featureTyping) (ordinal 0) (authored-target "Transfer")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 179 62) (end 179 74)) (probe (position 179 62))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind subsetting) (ordinal 0) (authored-target "performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 179 76) (end 179 87)) (probe (position 179 76))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers"))) (kind subsetting) (ordinal 1) (authored-target "binaryLinks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 186 28) (end 186 38)) (probe (position 186 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 186 49) (end 186 65)) (probe (position 186 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::source")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::source")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 186 67) (end 186 86)) (probe (position 186 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::source"))) (kind redefinition) (ordinal 1) (authored-target "binaryLinks::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 187 28) (end 187 38)) (probe (position 187 28))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 187 49) (end 187 65)) (probe (position 187 49))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind redefinition) (ordinal 0) (authored-target "Transfer::target")
      (outcome (status resolved) (target (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::Transfer::target")))))
  )
  (query (document "memory://snapshot/transfers.md") (range (start 187 67) (end 187 86)) (probe (position 187 67))
    (reference (id (source (node (document "memory://snapshot/transfers.md") (qualified-name "Transfers::transfers::target"))) (kind redefinition) (ordinal 1) (authored-target "binaryLinks::target")
      (outcome (status unresolved)))
  )
)
~~~
