# META
~~~ini
description=SysML Example (Simple Tests): StateTest
type=file
~~~
# SOURCE
~~~sysml
package StateTest {
	attribute def Sig {
		x;
	}
	attribute def Exit;
	
	part p;
	
	action act;
	
	state def S {
		do action A;
		entry; then S1;
		
		state S1;
			accept s : Sig
			do action D
			then S2;
				
		state S2 {
			do send new Sig(T.s.x) to p;
			state S3;
		}
		accept Exit then done;
		
		transition
			first S1
			accept s : Sig
			do action D
			then S2.S3;
		
		transition T
			first S2.S3
			accept s : Sig via p
			if true
			do send s to p
			then S1;
			
		exit act;
		
		state S3 {
			state S3a;
		}
		
		transition first S3.S3a then S1; 
	}
	
	state s0 {
  		state s1 {
    		state s2;
  		}
  		state s3 {
  			state s4;
  		}
  		transition t1 first s1.s2 then s3.s4;
	}
	
	state s parallel {
		state s1;
		state s2;
	}
	
	state s4 {
		do action a;
  		action c;
	}
	
	state s5 :> s4 {
  		do action b :>> c;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 1) (end 6 8))
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 10 1) (end 45 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 12) (end 11 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 15 3) (end 17 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 15 3) (end 17 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 19) (end 23 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 25 2) (end 29 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 25 2) (end 29 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 8) (end 29 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 31 2) (end 36 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 31 2) (end 36 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 9) (end 32 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 19) (end 44 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 24) (end 54 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 35) (end 54 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 12) (end 63 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:3939a77ebdac917aefc42d7c3264ac57e3226becddd7ac0e3ada2e8ac0480edc") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Exit"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "A")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "S1")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "S2")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "done")) (transitionTrigger (reference "Exit")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "S1")) (memberAccessOperand (reference "S2::S3")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "act")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "S1")) (memberAccessOperand (reference "S3::S3a")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2::S3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3::S3a"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "S1")) (memberAccessOperand (reference "S2::S3")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (named (kind transition) (name "T")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig::x"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1::s2"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3::s4"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "s1::s2")) (memberAccessOperand (reference "s3::s4")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state) (name "s4")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "a")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4::c"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "s4")))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s2"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "act")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind transitionSource) (ordinal 0))
      (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "S2")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0))
      (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Exit")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Exit")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "S2::S3")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "S3::S3a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind transitionTarget) (ordinal 0))
      (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "S2::S3")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "s1::s2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "s3::s4")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state) (name "s4")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0))
      (authored-target "s4")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))))
  )
  (relationships
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Exit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2::S3"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3::S3a"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (named (kind transition) (name "T")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig::x"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1::s2"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3::s4"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state) (name "s4")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4::c"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s1"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s2"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2::S3")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3::S3a")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (named (kind transition) (name "T")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig::x")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1::s2")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3::s4")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))
      (subtype (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state) (name "s4")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4::c")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5")))
      (supertype (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s1")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s")))
    )
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s2")))
      (featured-by (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_test.md") (range (start 11 12) (end 11 13)) (probe (position 11 12))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 38 7) (end 38 10)) (probe (position 38 7))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "act")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 12 14) (end 12 16)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 26 9) (end 26 11)) (probe (position 26 9))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind transitionSource) (ordinal 0) (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 17 8) (end 17 10)) (probe (position 17 8))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "S2")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 23 19) (end 23 23)) (probe (position 23 19))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 44 31) (end 44 33)) (probe (position 44 31))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0) (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 23 9) (end 23 13)) (probe (position 23 9))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "Exit")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Exit")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 29 8) (end 29 13)) (probe (position 29 8))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "S2::S3")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 44 19) (end 44 25)) (probe (position 44 19))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state-def) (name "S")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "S3::S3a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 36 8) (end 36 10)) (probe (position 36 8))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind transitionTarget) (ordinal 0) (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 32 9) (end 32 14)) (probe (position 32 9))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::T"))) (kind memberAccessOperand) (ordinal 0) (authored-target "S2::S3")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 54 24) (end 54 29)) (probe (position 54 24))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (kind memberAccessOperand) (ordinal 0) (authored-target "s1::s2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 54 35) (end 54 40)) (probe (position 54 35))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::t1"))) (kind memberAccessOperand) (ordinal 1) (authored-target "s3::s4")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 63 12) (end 63 13)) (probe (position 63 12))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (path (named (kind package) (name "StateTest")) (named (kind state) (name "s4")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/state_test.md") (range (start 67 13) (end 67 15)) (probe (position 67 13))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0) (authored-target "s4")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))))
    )
  )
)
~~~
