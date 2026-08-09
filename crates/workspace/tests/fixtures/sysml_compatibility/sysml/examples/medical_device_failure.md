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

        #multicausation
        connection {
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
(model
  (namespace
    (package 'MedicalDeviceFailure'
      (namespace_import private -> 'CauseAndEffect'[unresolved])
      (part_usage 'medicalDevice'
        (part_usage composite 'battery'
          (event_occurrence_usage 'depleted')
          (event_occurrence_usage 'cannotBeCharged'))
        (event_occurrence_usage 'deviceFails')
        (reference_usage reference 'patient'
          (event_occurrence_usage 'therapyDelayed'))
        (not_implemented 'malformed')
        (not_implemented 'malformed')
        (connection_usage composite
          (connector_end 'deviceFails')
          (connector_end 'patient.therapyDelayed'))))))
~~~
