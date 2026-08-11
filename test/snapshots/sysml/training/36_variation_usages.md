# META
~~~ini
description=SysML Training 36 (Variability): Variation Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Usages' {
	private import 'Variation Definitions'::*;
	
	part def Vehicle;
	part def Transmission;
	part manualTransmission;
	part automaticTransmission;
	
	abstract part vehicleFamily : Vehicle {
		part engine : EngineChoices[1];
		
		variation part transmission : Transmission[1] {
			variant manualTransmission;
			variant automaticTransmission;
		}
		
		assert constraint {
			(engine == engine::'4cylEngine' and
			 transmission == transmission::manualTransmission) xor
			(engine == engine::'6cylEngine' and 
			 transmission == transmission::automaticTransmission)
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 29))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAbstract,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwVariation,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Usages''
    (import_decl private ''Variation Definitions'::*')
    (part_def 'Vehicle')
    (part_def 'Transmission')
    (part_usage 'manualTransmission')
    (part_usage 'automaticTransmission')
    (part_usage abstract 'vehicleFamily' : 'Vehicle'
      (part_usage 'engine' : 'EngineChoices' multiplicity)
      (part_usage variation 'transmission' : 'Transmission' multiplicity
        (variant_usage
          (default_ref_usage 'manualTransmission'))
        (variant_usage
          (default_ref_usage 'automaticTransmission')))
      (sysml_decl
        (result_expr_member)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# FORMAT
~~~sysml
package 'Variation Usages' {
    private import 'Variation Definitions'::*;

    part def Vehicle;
    part def Transmission;
    part manualTransmission;
    part automaticTransmission;

    abstract part vehicleFamily : Vehicle {
        part engine : EngineChoices[1];

        variation part transmission : Transmission[1] {
            variant manualTransmission;
            variant automaticTransmission;
        }

        assert constraint {
            (engine == engine::'4cylEngine' and
            transmission == transmission::manualTransmission) xor
            (engine == engine::'6cylEngine' and
            transmission == transmission::automaticTransmission)
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "30c0802480995a246eb31968af76f254862fdd5c0c0695309f1a2d248f8a1df8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Variation Usages"))) (kind "package") (name "Variation Usages") (declared-name "Variation Usages") (range (start (line 0) (character 0)) (end (line 0) (character 602))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 43))) (parent (node (document "d0") (qualified-name "Variation Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Variation Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 4) (character 1)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (range (start (line 6) (character 1)) (end (line 6) (character 28))) (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (range (start (line 5) (character 1)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind "part") (name "vehicleFamily") (declared-name "vehicleFamily") (range (start (line 8) (character 1)) (end (line 8) (character 423))) (parent (node (document "d0") (qualified-name "Variation Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 8) (character 31)) (end (line 8) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 9) (character 2)) (end (line 9) (character 33))) (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineChoices") (range (start (line 9) (character 16)) (end (line 9) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 11) (character 2)) (end (line 11) (character 118))) (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 11) (character 32)) (end (line 11) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::automaticTransmission"))) (kind "variant") (name "automaticTransmission") (declared-name "automaticTransmission") (range (start (line 13) (character 3)) (end (line 13) (character 33))) (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::manualTransmission"))) (kind "variant") (name "manualTransmission") (declared-name "manualTransmission") (range (start (line 12) (character 3)) (end (line 12) (character 30))) (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Variation Definitions::*") (range (start (line 1) (character 16)) (end (line 1) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 8) (character 31)) (end (line 8) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Usages::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineChoices") (range (start (line 9) (character 16)) (end (line 9) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 11) (character 32)) (end (line 11) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Usages::Transmission")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (target (node (document "d0") (qualified-name "Variation Usages::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "d0") (qualified-name "Variation Usages::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
