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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 12) (end 11 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 15 3) (end 15 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 20 3) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 23 2) (end 23 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 25 2) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 31 2) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 44 2) (end 44 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 54 4) (end 54 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 12) (end 63 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 64 4) (end 65 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 68 4) (end 69 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3939a77ebdac917aefc42d7c3264ac57e3226becddd7ac0e3ada2e8ac0480edc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Exit"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "A"))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "S1"))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "act"))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S2::S3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S3::S3a"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::Sig::x"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s1::s2"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s0::s3::s4"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "a"))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "s4"))))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s::s2"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "act")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0))
      (authored-target "s4")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))))
  )
  (relationships
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/state_test.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_test.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_test.md") (range (start 11 12) (end 11 13)) (probe (position 11 12))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_test.md") (range (start 38 7) (end 38 10)) (probe (position 38 7))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "act")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::act")))))
  )
  (query (document "memory://snapshot/state_test.md") (range (start 12 14) (end 12 16)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "S1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::S::S1")))))
  )
  (query (document "memory://snapshot/state_test.md") (range (start 63 12) (end 63 13)) (probe (position 63 12))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_test.md") (range (start 67 13) (end 67 15)) (probe (position 67 13))
    (reference (id (source (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s5"))) (kind subsetting) (ordinal 0) (authored-target "s4")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_test.md") (qualified-name "StateTest::s4")))))
  )
)
~~~
