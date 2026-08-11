# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_03-Value Expression
type=file
~~~
# SOURCE
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass: MassValue = 1200 [kg];
        attribute length: LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
    	attribute hubDiameter: LengthValue = 18 ['in'];
        attribute width: LengthValue = 245 [mm];
        attribute outerDiameter: LengthValue = (hubDiameter + 2 * tire.height) [mm] {
	        doc
	        /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire: Tire[1];
    }
    
    part def Tire {
    	attribute profileDepth: LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}
    	attribute height: LengthValue = 45 [mm];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_03_value_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 35))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 8) (end 5 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 33))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 8) (end 6 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 26) (end 6 37))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 5) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 28) (end 12 39))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 36))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 14 8) (end 14 263))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 8) (end 14 263))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 33) (end 14 44))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 5) (end 25 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 29) (end 25 40))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 5) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 23) (end 27 34))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,OpenParen,Ident,Plus,DecimalValue,Star,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,KwDefault,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwConstraint,Ident,OpenCurly,Ident,GtEq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_03-Value Expression''
    (import_decl private 'SI::*')
    (import_decl private 'USCustomaryUnits::*')
    (part_def 'Vehicle_1'
      (attribute_usage 'mass' : 'MassValue' value)
      (attribute_usage 'length' : 'LengthValue' value)
      (part_usage 'leftFrontWheel' : 'Wheel')
      (part_usage 'rightFrontWheel' : 'Wheel'))
    (part_def 'Wheel'
      (attribute_usage 'hubDiameter' : 'LengthValue' value)
      (attribute_usage 'width' : 'LengthValue' value)
      (attribute_usage 'outerDiameter' : 'LengthValue' value
        (documentation))
      (part_usage 'tire' : 'Tire' multiplicity))
    (part_def 'Tire'
      (attribute_usage 'profileDepth' : 'LengthValue' value)
      (constraint_usage 'hasLegalProfileDepth'
        (result_expr_member))
      (attribute_usage 'height' : 'LengthValue' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
~~~
# FORMAT
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass: MassValue = 1200 [kg];
        attribute length: LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
        attribute hubDiameter: LengthValue = 18 ['in'];
        attribute width: LengthValue = 245 [mm];
        attribute outerDiameter: LengthValue = (hubDiameter + 2 * tire.height) [mm] {
            doc
            /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire: Tire[1];
    }

    part def Tire {
        attribute profileDepth: LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}
        attribute height: LengthValue = 45 [mm];
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5ad0d1a929a196c5612b056ed6eaf783ef06324cca393cd9a4da7ff48a402c8c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression"))) (kind "package") (name "15_03-Value Expression") (declared-name "15_03-Value Expression") (range (start (line 0) (character 0)) (end (line 0) (character 931))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 25))) (parent (node (document "d0") (qualified-name "15_03-Value Expression"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 21))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "15_03-Value Expression"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 35))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (range (start (line 24) (character 4)) (end (line 24) (character 197))) (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 27) (character 5)) (end (line 27) (character 45))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 27) (character 23)) (end (line 27) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind "attribute") (name "profileDepth") (declared-name "profileDepth") (range (start (line 25) (character 5)) (end (line 25) (character 58))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 25) (character 29)) (end (line 25) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (range (start (line 4) (character 4)) (end (line 4) (character 202))) (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (range (start (line 7) (character 8)) (end (line 7) (character 36))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 7) (character 30)) (end (line 7) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 6) (character 8)) (end (line 6) (character 49))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 6) (character 26)) (end (line 6) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 5) (character 8)) (end (line 5) (character 46))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 5) (character 24)) (end (line 5) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (range (start (line 8) (character 8)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 8) (character 31)) (end (line 8) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 11) (character 4)) (end (line 11) (character 420))) (parent (node (document "d0") (qualified-name "15_03-Value Expression"))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (range (start (line 12) (character 5)) (end (line 12) (character 52))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 12) (character 28)) (end (line 12) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind "attribute") (name "outerDiameter") (declared-name "outerDiameter") (range (start (line 14) (character 8)) (end (line 14) (character 263))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 14) (character 33)) (end (line 14) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind "part") (name "tire") (declared-name "tire") (range (start (line 21) (character 8)) (end (line 21) (character 27))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire") (range (start (line 21) (character 19)) (end (line 21) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 13) (character 8)) (end (line 13) (character 48))) (parent (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 13) (character 25)) (end (line 13) (character 36)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 1) (character 19)) (end (line 1) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 2) (character 19)) (end (line 2) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 27) (character 23)) (end (line 27) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 25) (character 29)) (end (line 25) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 7) (character 30)) (end (line 7) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 6) (character 26)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 5) (character 24)) (end (line 5) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 8) (character 31)) (end (line 8) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 12) (character 28)) (end (line 12) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 14) (character 33)) (end (line 14) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (range (start (line 21) (character 19)) (end (line 21) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_03-Value Expression::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 13) (character 25)) (end (line 13) (character 36))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (target (node (document "d0") (qualified-name "15_03-Value Expression::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::tire"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Tire::height")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Tire::profileDepth")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::length")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Vehicle_1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::hubDiameter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::outerDiameter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_03-Value Expression::Wheel::width")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
