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
(model
  (namespace
    (package '15_08-Range Restriction'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (membership_import private -> '15_01-Constants::Mathematical Constants::pi'[unresolved])
      (part_def 'HeadLightsTiltKnob'
        (attribute_usage composite 'headLightsTile' : '15_08-Range Restriction::LightBeamTiltAngleValue'[attribute_def]
          (multiplicity_range [1])))
      (attribute_def 'LightBeamTiltAngleValue' :> 'PlaneAngleValue'[unresolved]
        (attribute_usage composite 'angle' : '15_08-Range Restriction::LightBeamTiltAngleValue'[attribute_def] :>> 'self'[unresolved]
          (documentation))
        (assert_constraint_usage
          (result_expr_membership))))))
~~~
