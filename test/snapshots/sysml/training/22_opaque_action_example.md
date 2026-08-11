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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "22_opaque_action_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 2) (end 3 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 20) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 25))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ccd9ccb41e406150a11c63d073772165660416d4b1036d548e83ca868ec21cea") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Opaque Action Example"))) (kind "package") (name "Opaque Action Example") (declared-name "Opaque Action Example"))
    (element (id (node (document "d0") (qualified-name "Opaque Action Example::Sensor"))) (kind "part def") (name "Sensor") (declared-name "Sensor") (parent (node (document "d0") (qualified-name "Opaque Action Example"))))
    (element (id (node (document "d0") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind "attribute") (name "ready") (declared-name "ready") (parent (node (document "d0") (qualified-name "Opaque Action Example::Sensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")) (typing (reference "ScalarValues::Boolean")))))
    (element (id (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors"))) (kind "action def") (name "UpdateSensors") (declared-name "UpdateSensors") (parent (node (document "d0") (qualified-name "Opaque Action Example"))))
    (element (id (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind "in out parameter") (name "sensors") (declared-name "sensors") (parent (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors"))) (authored (relationships (typing (reference "sensors : Sensor[*]")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind featureTyping) (ordinal 0)) (authored-target "sensors : Sensor[*]") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 20) (end 3 41)) (probe (position 3 20))
      (reference
        (source (document "d0") (qualified-name "Opaque Action Example::Sensor::ready"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Boolean")
        (range (start 3 20) (end 3 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
