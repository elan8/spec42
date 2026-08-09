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
            constraint { device.battery.power < minPower }
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "User Keyword Example"))) (name "User Keyword Example") (declared-name "User Keyword Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "User Keyword Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "User Keyword Example::Device"))) (name "Device") (declared-name "Device") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery"))) (name "battery") (declared-name "battery") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "User Keyword Example::Device"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (name "power") (declared-name "power") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "User Keyword Example::Device")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "User Keyword Example::LevelEnum"))) (name "LevelEnum") (declared-name "LevelEnum"))
        (element (kind "import") (id (node (document "d0") (qualified-name "User Keyword Example::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "User Keyword Example::_scenario"))) (name "scenario") (declared-name "scenario"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "User Keyword Example::_scenario"))) (to (node (document "d0") (qualified-name "User Keyword Example"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/41_user_keyword_example.md"
    (diagnostics
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
        (range (start 2 16) (end 2 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 3) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 11 1) (end 11 11))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 11 11) (end 11 418))
      )
    )
  )
)
~~~
