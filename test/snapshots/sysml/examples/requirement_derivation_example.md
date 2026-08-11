# META
~~~ini
description=SysML Example (Requirements): RequirementDerivationExample
type=file
~~~
# SOURCE
~~~sysml
package RequirementDerivationExample {
	private import RequirementDerivation::*;
	
	requirement def Req1;
	
	requirement def Req1_1;
	requirement def Req1_2;
	
	#derivation connection def Req1_Derivation {
		end #original r1 : Req1;
		end #derive r1_1 : Req1_1;
		end #derive r1_2 : Req1_2;
	}
	
	part def System;
	part def Subsystem1;
	part def Subsystem2;
	
	part system : System {
		part sub1 : Subsystem1;
		part sub2 : Subsystem2;
	}
	
	part satisfactionContext {
		ref :>> system;
		
		satisfy requirement req1 : Req1 by system;
		satisfy requirement req1_1 : Req1_1 by system.sub1;
		satisfy requirement req1_2 : Req1_2 by system.sub2;
		
		#derivation connection : Req1_Derivation {
			end r1 ::> req1;
			end r1_1 ::> req1_1;
			end r1_2 ::> req1_1;
		}
		
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwRequirement,KwDef,Ident,Semicolon,
KwRequirement,KwDef,Ident,Semicolon,
KwRequirement,KwDef,Ident,Semicolon,
Hash,Ident,KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
KwEnd,Hash,Ident,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Semicolon,
KwSatisfy,KwRequirement,Ident,Colon,Ident,KwBy,Ident,Semicolon,
KwSatisfy,KwRequirement,Ident,Colon,Ident,KwBy,Ident,Dot,Ident,Semicolon,
KwSatisfy,KwRequirement,Ident,Colon,Ident,KwBy,Ident,Dot,Ident,Semicolon,
Hash,Ident,KwConnection,Colon,Ident,OpenCurly,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RequirementDerivationExample'
    (import_decl private 'RequirementDerivation::*')
    (requirement_def 'Req1')
    (requirement_def 'Req1_1')
    (requirement_def 'Req1_2')
    (connection_def #'derivation' 'Req1_Derivation'
      (interface_end end #'original' 'r1' : 'Req1')
      (interface_end end #'derive' 'r1_1' : 'Req1_1')
      (interface_end end #'derive' 'r1_2' : 'Req1_2'))
    (part_def 'System')
    (part_def 'Subsystem1')
    (part_def 'Subsystem2')
    (part_usage 'system' : 'System'
      (part_usage 'sub1' : 'Subsystem1')
      (part_usage 'sub2' : 'Subsystem2'))
    (part_usage 'satisfactionContext'
      (ref_usage ref :>> 'system')
      (sysml_decl 'req1' : 'Req1')
      (sysml_decl 'req1_1' : 'Req1_1')
      (sysml_decl 'req1_2' : 'Req1_2')
      (connection_usage 'Req1_Derivation'
        (interface_end end 'r1' references 'req1')
        (interface_end end 'r1_1' references 'req1_1')
        (interface_end end 'r1_2' references 'req1_1')))))
~~~
# FORMAT
~~~sysml
package RequirementDerivationExample {
    private import RequirementDerivation::*;

    requirement def Req1;

    requirement def Req1_1;
    requirement def Req1_2;

    #derivation connection def Req1_Derivation {
        end #original r1 : Req1;
        end #derive r1_1 : Req1_1;
        end #derive r1_2 : Req1_2;
    }

    part def System;
    part def Subsystem1;
    part def Subsystem2;

    part system : System {
        part sub1 : Subsystem1;
        part sub2 : Subsystem2;
    }

    part satisfactionContext {
        ref :>> system;

        satisfy requirement req1 : Req1 by system;
        satisfy requirement req1_1 : Req1_1 by system.sub1;
        satisfy requirement req1_2 : Req1_2 by system.sub2;

        #derivation connection : Req1_Derivation {
            end r1 ::> req1;
            end r1_1 ::> req1_1;
            end r1_2 ::> req1_1;
        }

    }

}

~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RequirementDerivationExample"))) (name "RequirementDerivationExample") (declared-name "RequirementDerivationExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementDerivationExample::*"))) (name "*") (declared-name "*"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1"))) (name "Req1") (declared-name "Req1"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1"))) (name "Req1_1") (declared-name "Req1_1"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2"))) (name "Req1_2") (declared-name "Req1_2"))
        (element (kind "derivation connection") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation"))) (name "Req1_Derivation") (declared-name "Req1_Derivation")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (name "r1") (declared-name "r1") (declared (properties (end true))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (name "r1_1") (declared-name "r1_1") (declared (properties (end true))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (name "r1_2") (declared-name "r1_2") (declared (properties (end true))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1"))) (name "Subsystem1") (declared-name "Subsystem1") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2"))) (name "Subsystem2") (declared-name "Subsystem2") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "RequirementDerivationExample::System"))) (name "System") (declared-name "System") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (name "satisfactionContext") (declared-name "satisfactionContext") (declared (properties (ordered false)))
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::system"))) (name "system") (declared-name "system") (declared (properties (composite false) (reference true))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (name "system") (declared-name "system") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (name "sub1") (declared-name "sub1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivationExample::System")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (name "sub2") (declared-name "sub2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivationExample::System")))))
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::unresolved_satisfy_source#diagnostic"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext::unresolved_satisfy_source#diagnostic2"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::Req1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_1"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_Derivation::r1_2"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::System"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (to (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "req1") (target-expression "system") (container-prefix "RequirementDerivationExample::satisfactionContext"))
    (satisfy (status pending-expression) (document "d0") (source-expression "req1_1") (target-expression "system::sub1") (container-prefix "RequirementDerivationExample::satisfactionContext"))
    (satisfy (status pending-expression) (document "d0") (source-expression "req1_2") (target-expression "system::sub2") (container-prefix "RequirementDerivationExample::satisfactionContext"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_1"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::Req1_2"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::Subsystem2"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::System"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::satisfactionContext"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::system"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivationExample::system::sub2"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/requirement_derivation_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 26 22) (end 26 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 27 22) (end 27 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 28 22) (end 28 37))
      )
    )
  )
)
~~~
