# META
~~~ini
description=SysML Training 41 (Language Extension): User Keyword Example
type=file
~~~
# SOURCE
~~~sysml
package 'User Keyword Example' {
	private import ScalarValues::Real;
	private import 'Semantic Metadata Example'::*;
	private import RiskMetadata::LevelEnum;
	
	part def Device {
		part battery {
			attribute power : Real;
		}
	}
	
	#scenario def DeviceFailure {
		ref device : Device;
		attribute minPower : Real;
		
		#cause 'battery old' {
			:>> probability = 0.01;			
		}
		
		#causation connect 'battery old' to 'power low';
		
		#situation 'power low' {
			constraint { device.battery.power < minPower }			
		}
		
		#causation connect 'power low' to 'device shutoff';
		
		#failure 'device shutoff' {
			:>> severity = LevelEnum::high;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,KwDef,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
Hash,Ident,UnrestrictedName,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
Hash,Ident,UnrestrictedName,OpenCurly,
KwConstraint,OpenCurly,Ident,Dot,Ident,Dot,Ident,OpenAngle,Ident,CloseCurly,
CloseCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
Hash,Ident,UnrestrictedName,OpenCurly,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''User Keyword Example''
    (import_decl private 'ScalarValues::Real')
    (import_decl private ''Semantic Metadata Example'::*')
    (import_decl private 'RiskMetadata::LevelEnum')
    (part_def 'Device'
      (part_usage 'battery'
        (attribute_usage 'power' : 'Real')))
    (extended_def #'scenario' 'DeviceFailure'
      (ref_usage ref 'device' : 'Device')
      (attribute_usage 'minPower' : 'Real')
      (extended_usage #'cause' ''battery old''
        (default_ref_usage :>> 'probability' value))
      (connection_usage
        (connector_end)
        (connector_end))
      (extended_usage #'situation' ''power low''
        (constraint_usage
          (result_expr_member)))
      (connection_usage
        (connector_end)
        (connector_end))
      (extended_usage #'failure' ''device shutoff''
        (default_ref_usage :>> 'severity' value)))))
~~~
# FORMAT
~~~sysml
package 'User Keyword Example' {
    private import ScalarValues::Real;
    private import 'Semantic Metadata Example'::*;
    private import RiskMetadata::LevelEnum;

    part def Device {
        part battery {
            attribute power : Real;
        }
    }

    #scenario def DeviceFailure {
        ref device : Device;
        attribute minPower : Real;

        #cause 'battery old' {
            :>> probability = 0.01;
        }

        #causation connect 'battery old' to 'power low';

        #situation 'power low' {
            constraint {
                = device.battery.power < minPower;
            }
        }

        #causation connect 'power low' to 'device shutoff';

        #failure 'device shutoff' {
            :>> severity = LevelEnum::high;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'probability'
semantic.unresolved_name 'severity'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'probability'
semantic.unresolved_name 'severity'
~~~
# SMG
~~~
(model
  (namespace
    (package 'User Keyword Example'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Semantic Metadata Example'[unresolved])
      (membership_import private -> 'RiskMetadata::LevelEnum'[unresolved])
      (part_def 'Device'
        (part_usage composite 'battery'
          (attribute_usage composite 'power' : 'Real'[unresolved])))
      (definition 'DeviceFailure'
        (reference_usage reference 'device' : 'User Keyword Example::Device'[part_def])
        (attribute_usage composite 'minPower' : 'Real'[unresolved])
        (reference_usage 'battery old'
          (reference_usage reference :>> 'probability'[unresolved]
            (feature_value (=))))
        (connection_usage composite
          (connector_end ''battery old'')
          (connector_end ''power low''))
        (reference_usage 'power low'
          (constraint_usage composite
            (result_expr_membership)))
        (connection_usage composite
          (connector_end ''power low'')
          (connector_end ''device shutoff''))
        (reference_usage 'device shutoff'
          (reference_usage reference :>> 'severity'[unresolved]
            (feature_value (=))))))))
~~~
