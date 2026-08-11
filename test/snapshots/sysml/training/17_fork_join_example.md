# META
~~~ini
description=SysML Training 17 (Control): Fork Join Example
type=file
~~~
# SOURCE
~~~sysml
package 'Fork Join Example' {
	private import ScalarValues::*;
	
	attribute def TurnKeyToOn;
	attribute def BrakePressure;
	
	action def MonitorBrakePedal { out pressure : BrakePressure; }
	action def MonitorTraction { out modFreq : Real; }
	action def Braking { in brakePressure : BrakePressure; in modulationFrequency : Real; }
	
	action def Brake {
		action TurnOn;
		
		then fork;
			then monitorBrakePedal;
			then monitorTraction;
			then braking;
		
		action monitorBrakePedal : MonitorBrakePedal {
			out brakePressure;
		}
		then joinNode;
		
		action monitorTraction : MonitorTraction {
			out modulationFrequency;
		}
		then joinNode;
		
		flow from monitorBrakePedal.brakePressure to braking.brakePressure;
		flow from monitorTraction.modulationFrequency to braking.modulationFrequency; 
		
		action braking : Braking {
			in brakePressure; 
			in modulationFrequency;
		}
		then joinNode;
		
		join joinNode;
		then done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_fork_join_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 30) (end 7 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 56) (end 8 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 1) (end 10 605))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 3) (end 19 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 3) (end 24 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 3) (end 32 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 3) (end 33 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 2) (end 37 16))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "89e1ac55e67a450690af451d7122db0a184c73a8c671de2d9614499e0700f70e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Fork Join Example"))) (kind "package") (name "Fork Join Example") (declared-name "Fork Join Example"))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Fork Join Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind "action def") (name "Brake") (declared-name "Brake") (parent (node (document "d0") (qualified-name "Fork Join Example"))) (authored (membership (kind Owning)) (relationships (flow (reference "Fork Join Example::Brake::fork")) (perform (reference "Fork Join Example::Brake::TurnOn")) (perform (reference "Fork Join Example::Brake::monitorBrakePedal")) (perform (reference "Fork Join Example::Brake::monitorTraction")) (perform (reference "Fork Join Example::Brake::braking")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::TurnOn"))) (kind "action") (name "TurnOn") (declared-name "TurnOn") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (kind "action") (name "braking") (declared-name "braking") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (authored (membership (kind Feature)) (relationships (typing (reference "Braking")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure"))) (kind "in out parameter") (name "brakePressure") (declared-name "brakePressure") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency"))) (kind "in out parameter") (name "modulationFrequency") (declared-name "modulationFrequency") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::from#flow"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))) (kind "join") (name "join") (declared-name "join") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (authored (relationships (flow (reference "Fork Join Example::Brake::done")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind "action") (name "monitorBrakePedal") (declared-name "monitorBrakePedal") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (authored (membership (kind Feature)) (relationships (typing (reference "MonitorBrakePedal")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure"))) (kind "in out parameter") (name "brakePressure") (declared-name "brakePressure") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind "action") (name "monitorTraction") (declared-name "monitorTraction") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (authored (membership (kind Feature)) (relationships (typing (reference "MonitorTraction")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency"))) (kind "in out parameter") (name "modulationFrequency") (declared-name "modulationFrequency") (parent (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))) (kind "attribute def") (name "BrakePressure") (declared-name "BrakePressure") (parent (node (document "d0") (qualified-name "Fork Join Example"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Braking"))) (kind "action def") (name "Braking") (declared-name "Braking") (parent (node (document "d0") (qualified-name "Fork Join Example"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (kind "in out parameter") (name "brakePressure") (declared-name "brakePressure") (parent (node (document "d0") (qualified-name "Fork Join Example::Braking"))) (authored (relationships (typing (reference "BrakePressure")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::Braking::modulationFrequency"))) (kind "in out parameter") (name "modulationFrequency") (declared-name "modulationFrequency") (parent (node (document "d0") (qualified-name "Fork Join Example::Braking"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (kind "action def") (name "MonitorBrakePedal") (declared-name "MonitorBrakePedal") (parent (node (document "d0") (qualified-name "Fork Join Example"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (kind "in out parameter") (name "pressure") (declared-name "pressure") (parent (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (authored (relationships (typing (reference "BrakePressure")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction"))) (kind "action def") (name "MonitorTraction") (declared-name "MonitorTraction") (parent (node (document "d0") (qualified-name "Fork Join Example"))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction::modFreq"))) (kind "in out parameter") (name "modFreq") (declared-name "modFreq") (parent (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Fork Join Example::TurnKeyToOn"))) (kind "attribute def") (name "TurnKeyToOn") (declared-name "TurnKeyToOn") (parent (node (document "d0") (qualified-name "Fork Join Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowSource) (ordinal 0)) (authored-target "Fork Join Example::Brake::fork") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowSource) (ordinal 0)) (authored-target "monitorBrakePedal::brakePressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowSource) (ordinal 1)) (authored-target "monitorTraction::modulationFrequency") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowTarget) (ordinal 0)) (authored-target "braking::brakePressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowTarget) (ordinal 1)) (authored-target "braking::modulationFrequency") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 0)) (authored-target "Fork Join Example::Brake::TurnOn") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::TurnOn")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 1)) (authored-target "Fork Join Example::Brake::monitorBrakePedal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 2)) (authored-target "Fork Join Example::Brake::monitorTraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 3)) (authored-target "Fork Join Example::Brake::braking") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (kind featureTyping) (ordinal 0)) (authored-target "Braking") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::Braking")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::joinNode"))) (kind flowSource) (ordinal 0)) (authored-target "Fork Join Example::Brake::done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind featureTyping) (ordinal 0)) (authored-target "MonitorBrakePedal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MonitorTraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (kind featureTyping) (ordinal 0)) (authored-target "BrakePressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::BrakePressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::Braking::modulationFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (kind featureTyping) (ordinal 0)) (authored-target "BrakePressure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Fork Join Example::BrakePressure")))))
    (reference (id (source (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction::modFreq"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::TurnOn"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (target (node (document "d0") (qualified-name "Fork Join Example::Braking"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake::braking"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (target (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "monitorBrakePedal::brakePressure") (target "braking::brakePressure")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (target (node (document "d0") (qualified-name "Fork Join Example::MonitorTraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency"))) (target (node (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Brake"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "monitorTraction::modulationFrequency") (target "braking::modulationFrequency")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (target (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::Braking::brakePressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (target (node (document "d0") (qualified-name "Fork Join Example::BrakePressure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Fork Join Example::MonitorBrakePedal::pressure"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Fork Join Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 47) (end 28 68)) (probe (position 28 47))
      (reference
        (source (document "d0") (qualified-name "Fork Join Example::Brake"))
        (kind flowTarget) (ordinal 0) (authored-target "braking::brakePressure")
        (range (start 28 47) (end 28 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Fork Join Example::Brake::braking::brakePressure") (range (start 32 3) (end 32 20)))
        )
      )
    )
    (query (range (start 29 51) (end 29 78)) (probe (position 29 51))
      (reference
        (source (document "d0") (qualified-name "Fork Join Example::Brake"))
        (kind flowTarget) (ordinal 1) (authored-target "braking::modulationFrequency")
        (range (start 29 51) (end 29 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Fork Join Example::Brake::braking::modulationFrequency") (range (start 33 3) (end 33 26)))
        )
      )
    )
    (query (range (start 28 12) (end 28 43)) (probe (position 28 12))
      (reference
        (source (document "d0") (qualified-name "Fork Join Example::Brake"))
        (kind flowSource) (ordinal 0) (authored-target "monitorBrakePedal::brakePressure")
        (range (start 28 12) (end 28 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Fork Join Example::Brake::monitorBrakePedal::brakePressure") (range (start 19 3) (end 19 21)))
        )
      )
    )
    (query (range (start 29 12) (end 29 47)) (probe (position 29 12))
      (reference
        (source (document "d0") (qualified-name "Fork Join Example::Brake"))
        (kind flowSource) (ordinal 1) (authored-target "monitorTraction::modulationFrequency")
        (range (start 29 12) (end 29 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Fork Join Example::Brake::monitorTraction::modulationFrequency") (range (start 24 3) (end 24 27)))
        )
      )
    )
  )
)
~~~
