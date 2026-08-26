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
  (document "memory://snapshot/22_opaque_action_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 20) (end 3 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e3f38762e7581315421a7680d5f802703299316857c394fe2bd0254010906ab4") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean")))))
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (rep (language "Alf") (text " \n\t\t\t * for (sensor in sensors) {\n\t\t\t *     if (sensor.ready) {\n\t\t\t *         Update(sensor);\n\t\t\t *     }\n\t\t\t * }\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (target (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor::ready"))) (target (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (target (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")))
      (subtype (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor::ready")))
      (featured-by (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")))
    )
    (declaration (id (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors")))
      (featured-by (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors")))
      (type (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/22_opaque_action_example.md") (range (start 3 20) (end 3 41)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor::ready"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/22_opaque_action_example.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::UpdateSensors::sensors"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/22_opaque_action_example.md") (qualified-name "Opaque Action Example::Sensor")))))
    )
  )
)
~~~
