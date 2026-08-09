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
        in sensors : Sensor [*];
        language "Alf" /* 
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
(model
  (namespace
    (package 'Opaque Action Example'
      (part_def 'Sensor'
        (attribute_usage composite 'ready' : 'ScalarValues::Boolean'[unresolved]))
      (action_def 'UpdateSensors'
        (reference_usage in reference 'sensors' : 'Opaque Action Example::Sensor'[part_def]
          (multiplicity_range [*]))
        (textual_rep)))))
~~~
