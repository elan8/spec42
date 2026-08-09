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

        satisfy req1 : Req1 by system;
        satisfy req1_1 : Req1_1 by system.sub1;
        satisfy req1_2 : Req1_2 by system.sub2;

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
(model
  (namespace
    (package 'RequirementDerivationExample'
      (namespace_import private -> 'RequirementDerivation'[unresolved])
      (requirement_def 'Req1')
      (requirement_def 'Req1_1')
      (requirement_def 'Req1_2')
      (connection_def 'Req1_Derivation'
        (port_usage end 'r1' : 'RequirementDerivationExample::Req1'[requirement_def])
        (port_usage end 'r1_1' : 'RequirementDerivationExample::Req1_1'[requirement_def])
        (port_usage end 'r1_2' : 'RequirementDerivationExample::Req1_2'[requirement_def]))
      (part_def 'System')
      (part_def 'Subsystem1')
      (part_def 'Subsystem2')
      (part_usage 'system' : 'RequirementDerivationExample::System'[part_def]
        (part_usage composite 'sub1' : 'RequirementDerivationExample::Subsystem1'[part_def])
        (part_usage composite 'sub2' : 'RequirementDerivationExample::Subsystem2'[part_def]))
      (part_usage 'satisfactionContext'
        (reference_usage reference :>> 'RequirementDerivationExample::system'[part_usage])
        (satisfy_requirement_usage 'req1' : 'RequirementDerivationExample::Req1'[requirement_def] by ''[reference_usage])
        (satisfy_requirement_usage 'req1_1' : 'RequirementDerivationExample::Req1_1'[requirement_def] by 'RequirementDerivationExample::system::sub1'[part_usage])
        (satisfy_requirement_usage 'req1_2' : 'RequirementDerivationExample::Req1_2'[requirement_def] by 'RequirementDerivationExample::system::sub2'[part_usage])
        (connection_usage composite : 'RequirementDerivationExample::Req1_Derivation'[connection_def]
          (port_usage end 'r1' :> 'RequirementDerivationExample::satisfactionContext::req1'[satisfy_requirement_usage])
          (port_usage end 'r1_1' :> 'RequirementDerivationExample::satisfactionContext::req1_1'[satisfy_requirement_usage])
          (port_usage end 'r1_2' :> 'RequirementDerivationExample::satisfactionContext::req1_1'[satisfy_requirement_usage]))))))
~~~
