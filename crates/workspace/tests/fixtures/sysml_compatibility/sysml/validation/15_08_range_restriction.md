# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_08-Range Restriction
type=file
~~~
# SOURCE
~~~sysml
package '15_08-Range Restriction' {
	private import ISQ::*;
	private import SI::*;
	private import '15_01-Constants'::'Mathematical Constants'::pi;
	
	part def HeadLightsTiltKnob {
		attribute headLightsTile : LightBeamTiltAngleValue[1];
	}
	
	attribute def LightBeamTiltAngleValue :> PlaneAngleValue {
		attribute angle: LightBeamTiltAngleValue :>> self {
			doc
			/*
			 * Tilt angle shall be limited to the range between 50 and 80 degrees (inclusive).
			 */
		}
		assert constraint { angle >= 50 ['°'] and angle <= 80 ['°'] }
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,UnrestrictedName,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,GtEq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,KwAnd,Ident,LtEq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_08-Range Restriction''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private ''15_01-Constants'::'Mathematical Constants'::pi')
    (part_def 'HeadLightsTiltKnob'
      (attribute_usage 'headLightsTile' : 'LightBeamTiltAngleValue' multiplicity))
    (attribute_def 'LightBeamTiltAngleValue' :> 'PlaneAngleValue'
      (attribute_usage 'angle' : 'LightBeamTiltAngleValue' :>> 'self'
        (documentation))
      (sysml_decl
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package '15_08-Range Restriction' {
    private import ISQ::*;
    private import SI::*;
    private import '15_01-Constants'::'Mathematical Constants'::pi;

    part def HeadLightsTiltKnob {
        attribute headLightsTile : LightBeamTiltAngleValue [1];
    }

    attribute def LightBeamTiltAngleValue :> PlaneAngleValue {
        attribute angle : LightBeamTiltAngleValue :>> self {
            doc /*
			 * Tilt angle shall be limited to the range between 50 and 80 degrees (inclusive).
			 */
        }
        assert constraint {
            = angle >= 50['°'] and angle <= 80['°'];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'PlaneAngleValue'
semantic.unresolved_name 'self'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PlaneAngleValue'
semantic.unresolved_name 'self'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_08-Range Restriction"))) (name "15_08-Range Restriction") (declared-name "15_08-Range Restriction")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_08-Range Restriction::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_08-Range Restriction::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob"))) (name "HeadLightsTiltKnob") (declared-name "HeadLightsTiltKnob") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (name "headLightsTile") (declared-name "headLightsTile") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))) (name "LightBeamTiltAngleValue") (declared-name "LightBeamTiltAngleValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (name "angle") (declared-name "angle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15_08-Range Restriction::pi"))) (name "pi") (declared-name "pi"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle::_documentation"))) (to (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_08-Range Restriction::HeadLightsTiltKnob::headLightsTile"))) (to (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue::angle"))) (to (node (document "d0") (qualified-name "15_08-Range Restriction::LightBeamTiltAngleValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
