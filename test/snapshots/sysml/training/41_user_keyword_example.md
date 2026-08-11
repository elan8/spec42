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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_user_keyword_example.md"
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
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 11 11) (end 11 418))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "d81dcf6bb85720d5fe864d8cde98849e8022a9e1d2777f15e83968dd4334b651") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "User Keyword Example"))) (kind "package") (name "User Keyword Example") (declared-name "User Keyword Example") (range (start (line 0) (character 0)) (end (line 0) (character 651))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 47))) (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Semantic Metadata Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 43))))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device"))) (kind "part def") (name "Device") (declared-name "Device") (range (start (line 5) (character 1)) (end (line 5) (character 69))) (parent (node (document "d0") (qualified-name "User Keyword Example"))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 6) (character 2)) (end (line 6) (character 47))) (parent (node (document "d0") (qualified-name "User Keyword Example::Device"))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind "attribute") (name "power") (declared-name "power") (range (start (line 7) (character 3)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "User Keyword Example::Device::battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 7) (character 21)) (end (line 7) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::LevelEnum"))) (kind "import") (name "LevelEnum") (declared-name "LevelEnum") (range (start (line 3) (character 1)) (end (line 3) (character 40))) (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::LevelEnum") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 39))))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "User Keyword Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "User Keyword Example::_scenario"))) (kind "metadata keyword") (name "scenario") (declared-name "scenario") (range (start (line 11) (character 1)) (end (line 11) (character 11))) (parent (node (document "d0") (qualified-name "User Keyword Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Semantic Metadata Example::*") (range (start (line 2) (character 16)) (end (line 2) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "User Keyword Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 7) (character 21)) (end (line 7) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "User Keyword Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::LevelEnum"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::LevelEnum") (range (start (line 3) (character 16)) (end (line 3) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "User Keyword Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (target (node (document "d0") (qualified-name "User Keyword Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (target (node (document "d0") (qualified-name "User Keyword Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "User Keyword Example::Device::battery::power"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
