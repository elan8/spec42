# META
~~~ini
description=SysML Example (Cause and Effect): MedicalDeviceFailure
type=file
~~~
# SOURCE
~~~sysml
package MedicalDeviceFailure {
	private import CauseAndEffect::*;
	
	part medicalDevice {
		part battery {
			event occurrence depleted;
			event occurrence cannotBeCharged;
		}
		
		event occurrence deviceFails;
		
		ref patient {
			event occurrence therapyDelayed;
		}
		
		#multicausation connection {
			end #cause ::> battery.depleted;
			end #cause ::> battery.cannotBeCharged;
			end #effect ::> deviceFails;
		}
		
		#causation connect deviceFails to patient.therapyDelayed;
	}	
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwRef,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwConnection,OpenCurly,
KwEnd,Hash,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwConnect,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MedicalDeviceFailure'
    (import_decl private 'CauseAndEffect::*')
    (part_usage 'medicalDevice'
      (part_usage 'battery'
        (event_occurrence 'depleted')
        (event_occurrence 'cannotBeCharged'))
      (event_occurrence 'deviceFails')
      (ref_usage ref 'patient'
        (event_occurrence 'therapyDelayed'))
      (malformed)
      (malformed)
      (connection_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package MedicalDeviceFailure {
    private import CauseAndEffect::*;

    part medicalDevice {
        part battery {
            event occurrence depleted;
            event occurrence cannotBeCharged;
        }

        event occurrence deviceFails;

        ref patient {
            event occurrence therapyDelayed;
        }

        #multicausation connection {
            end #cause ::> battery.depleted;
            end #cause ::> battery.cannotBeCharged;
            end #effect ::> deviceFails;
        }

        #causation connect deviceFails to patient.therapyDelayed;
    }

}

~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MedicalDeviceFailure"))) (name "MedicalDeviceFailure") (declared-name "MedicalDeviceFailure")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice"))) (name "medicalDevice") (declared-name "medicalDevice") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery"))) (name "battery") (declared-name "battery") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::cannotBeCharged"))) (name "cannotBeCharged") (declared-name "cannotBeCharged") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::battery::depleted"))) (name "depleted") (declared-name "depleted") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::deviceFails"))) (name "deviceFails") (declared-name "deviceFails") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "MedicalDeviceFailure::medicalDevice::patient"))) (name "patient") (declared-name "patient") (declared (properties (composite false) (reference true))))
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
  (document "sysml/examples/medical_device_failure.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
    )
  )
)
~~~
