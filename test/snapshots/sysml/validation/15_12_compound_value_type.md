# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_12-Compound Value Type
type=file
~~~
# SOURCE
~~~sysml
package '15_12-Compound Value Type' {
	private import ScalarValues::*;
	private import USCustomaryUnits::'in';
	
	/*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x: Real[1];
        attribute y: Real[1];
        attribute z: Real[1];
    }
    
    attribute def LengthValue :> Real;

    attribute def TireInfo {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
        attribute placement: PositionVector[0..1];
    }
    
    attribute frenchTireInfo: TireInfo {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0['in'];
    	attribute :>> width = 245;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_12_compound_value_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 8) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 8) (end 12 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 4) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 5) (end 19 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 33))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,UnrestrictedName,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_12-Compound Value Type''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'USCustomaryUnits::'in'')
    (comment)
    (attribute_def 'PositionVector'
      (attribute_usage 'x' : 'Real' multiplicity)
      (attribute_usage 'y' : 'Real' multiplicity)
      (attribute_usage 'z' : 'Real' multiplicity))
    (attribute_def 'LengthValue' :> 'Real')
    (attribute_def 'TireInfo'
      (attribute_usage 'manufacturer' : 'String')
      (attribute_usage 'hubDiameter' : 'LengthValue')
      (attribute_usage 'width' : 'Integer')
      (attribute_usage 'placement' : 'PositionVector' multiplicity))
    (attribute_usage 'frenchTireInfo' : 'TireInfo'
      (attribute_usage :>> 'manufacturer' value)
      (attribute_usage :>> 'hubDiameter' value)
      (attribute_usage :>> 'width' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# FORMAT
~~~sysml
package '15_12-Compound Value Type' {
    private import ScalarValues::*;
    private import USCustomaryUnits::'in';

    /*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x: Real[1];
        attribute y: Real[1];
        attribute z: Real[1];
    }

    attribute def LengthValue :> Real;

    attribute def TireInfo {
        attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
        attribute placement: PositionVector[0..1];
    }

    attribute frenchTireInfo: TireInfo {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0['in'];
        attribute :>> width = 245;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "75aec00f8071926fcecfd7ec6e45f6564ea0ccc21920b6a6df5e833e1c1b915d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (kind "package") (name "15_12-Compound Value Type") (declared-name "15_12-Compound Value Type") (range (start (line 0) (character 0)) (end (line 0) (character 858))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind "attribute def") (name "LengthValue") (declared-name "LengthValue") (range (start (line 16) (character 4)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (kind "attribute def") (name "PositionVector") (declared-name "PositionVector") (range (start (line 10) (character 4)) (end (line 10) (character 130))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 11) (character 8)) (end (line 11) (character 29))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind "attribute") (name "y") (declared-name "y") (range (start (line 12) (character 8)) (end (line 12) (character 29))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind "attribute") (name "z") (declared-name "z") (range (start (line 13) (character 8)) (end (line 13) (character 29))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (kind "attribute def") (name "TireInfo") (declared-name "TireInfo") (range (start (line 18) (character 4)) (end (line 18) (character 200))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (range (start (line 20) (character 8)) (end (line 20) (character 43))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (range (start (line 19) (character 5)) (end (line 19) (character 36))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind "attribute") (name "placement") (declared-name "placement") (range (start (line 22) (character 8)) (end (line 22) (character 50))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "PositionVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 21) (character 8)) (end (line 21) (character 33))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind "attribute def") (name "frenchTireInfo") (declared-name "frenchTireInfo") (range (start (line 25) (character 4)) (end (line 25) (character 169))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Owning)) (relationships (typing (reference "TireInfo") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (range (start (line 27) (character 5)) (end (line 27) (character 44))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "hubDiameter") (range (start (line 27) (character 19)) (end (line 27) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (range (start (line 26) (character 5)) (end (line 26) (character 45))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "manufacturer") (range (start (line 26) (character 19)) (end (line 26) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 28) (character 5)) (end (line 28) (character 31))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 28) (character 19)) (end (line 28) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "15_12-Compound Value Type::in"))) (kind "import") (name "in") (declared-name "in") (range (start (line 2) (character 1)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "15_12-Compound Value Type"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::in") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 38))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::manufacturer"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0)) (authored-target "PositionVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::width"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0)) (authored-target "TireInfo") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind redefinition) (ordinal 0)) (authored-target "hubDiameter") (range (start (line 27) (character 19)) (end (line 27) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind redefinition) (ordinal 0)) (authored-target "manufacturer") (range (start (line 26) (character 19)) (end (line 26) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 28) (character 19)) (end (line 28) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width")))))
    (reference (id (source (node (document "d0") (qualified-name "15_12-Compound Value Type::in"))) (kind membershipImport) (ordinal 0)) (authored-target "USCustomaryUnits::in") (range (start (line 2) (character 16)) (end (line 2) (character 38))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::PositionVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo::placement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::TireInfo"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::hubDiameter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::manufacturer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (target (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_12-Compound Value Type::frenchTireInfo::width"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
