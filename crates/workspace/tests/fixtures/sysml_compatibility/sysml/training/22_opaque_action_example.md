# META
~~~ini
description=SysML Training 22 (Opaque Actions): Opaque Action Example
type=file
~~~
# SOURCE
~~~sysml
package 'Opaque Action Example' {
	
	part def Sensor {
		attribute ready : ScalarValues::Boolean;
	}
	
	action def UpdateSensors {
		in sensors : Sensor[*];
		language "Alf" 
			/* 
			 * for (sensor in sensors) {
			 *     if (sensor.ready) {
			 *         Update(sensor);
			 *     }
			 * }
			 */
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwLanguage,StringValue,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Opaque Action Example''
    (part_def 'Sensor'
      (attribute_usage 'ready' : 'ScalarValues::Boolean'))
    (action_def 'UpdateSensors'
      (default_ref_usage in 'sensors' : 'Sensor' multiplicity)
      (textual_rep language '"Alf"'))))
~~~
# FORMAT
~~~sysml
package 'Opaque Action Example' {

    part def Sensor {
        attribute ready : ScalarValues::Boolean;
    }

    action def UpdateSensors {
        in sensors : Sensor[*];
        language "Alf"
        /* 
			 * for (sensor in sensors) {
			 *     if (sensor.ready) {
			 *         Update(sensor);
			 *     }
			 * }
			 */
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Opaque Action Example"))) (name "Opaque Action Example") (declared-name "Opaque Action Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Opaque Action Example::Sensor"))) (name "Sensor") (declared-name "Sensor") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Opaque Action Example::Sensor::ready"))) (name "ready") (declared-name "ready") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Opaque Action Example::Sensor")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors"))) (name "UpdateSensors") (declared-name "UpdateSensors")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (name "sensors") (declared-name "sensors") (effective (featuring-type (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors")))))
          )
        )
      )
    )
  )
  (relationships
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
  (document "sysml/training/22_opaque_action_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 2) (end 3 42))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 2) (end 8 145))
      )
    )
  )
)
~~~
