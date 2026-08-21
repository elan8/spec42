# META
~~~ini
description=SysML Training 36 (Variability): Variation Definitions
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Definitions' {
	private import ScalarValues::Real;
	private import SI::mm;
	
	attribute def Diameter :> ISQ::LengthValue;
	
    part def Cylinder {
        attribute diameter : Diameter[1];
    }

    part def Engine {
    	part cylinder : Cylinder[2..*];
    }
    
    part '4cylEngine' : Engine {
    	part redefines cylinder[4];
    }
    
    part '6cylEngine' : Engine {
    	part redefines cylinder[6];
    }
    
    // Variability model
	
	variation attribute def DiameterChoices :> Diameter {
		variant attribute diameterSmall = 70[mm];
		variant attribute diameterLarge = 100[mm];
	}

	variation part def EngineChoices :> Engine {
		variant '4cylEngine';
		variant '6cylEngine';		
	}	

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/36_variation_definitions.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 27) (end 4 43))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 25 2) (end 26 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 26 2) (end 27 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9d9386d4bb621cb0255685478bfebdac9bd473f33ba854a1631b1b875876e1dc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::mm") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cylinder")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cylinder")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Diameter")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ISQ::LengthValue")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Diameter")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")))))
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine")) (variant (reference "4cylEngine")) (variant (reference "6cylEngine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::mm")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter"))) (kind specialization) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices"))) (kind specialization) (ordinal 0))
      (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 0))
      (authored-target "4cylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 1))
      (authored-target "6cylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine")))
      (type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine")))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (source inherited) (from (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (scopes any))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine")))
      (type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine")))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (source inherited) (from (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (scopes any))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter")))
      (featured-by (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")))
      (type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter")) (scopes any))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices")))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine")) (scopes any))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine")) (scopes any))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")))
      (featured-by (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))
      (type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")) (scopes any))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices")))
      (supertype (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SI::mm")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 14 24) (end 14 30)) (probe (position 14 24))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 15 20) (end 15 28)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "4cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 18 24) (end 18 30)) (probe (position 18 24))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 19 20) (end 19 28)) (probe (position 19 20))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (path (named (kind package) (name "Variation Definitions")) (named (kind part) (name "6cylEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 7 29) (end 7 37)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder::diameter"))) (kind featureTyping) (ordinal 0) (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 4 27) (end 4 43)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter"))) (kind specialization) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 24 44) (end 24 52)) (probe (position 24 44))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::DiameterChoices"))) (kind specialization) (ordinal 0) (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Diameter")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 11 21) (end 11 29)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine::cylinder"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Cylinder")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 29 37) (end 29 43)) (probe (position 29 37))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 30 10) (end 30 22)) (probe (position 30 10))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 0) (authored-target "4cylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::4cylEngine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_definitions.md") (range (start 31 10) (end 31 22)) (probe (position 31 10))
    (reference (id (source (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::EngineChoices"))) (kind variant) (ordinal 1) (authored-target "6cylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_definitions.md") (qualified-name "Variation Definitions::6cylEngine")))))
    )
  )
)
~~~
